use std::time::Instant;
use std::collections::HashMap;
use tauri::{AppHandle, Manager};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
use windows::Win32::Foundation::POINT;
use windows::Win32::UI::Accessibility::{IUIAutomation, UIA_BoundingRectanglePropertyId};

use crate::models::error::Error;
use crate::models::element_info::{ElementInfo, ActionCategory};
use crate::models::state::TrackingState;
use crate::models::screen::ScreenContext;
use crate::screenshot::capture_screenshot;
use crate::ui_automation::element::{
    initialize_automation,
    get_element_info,
};

// App identifiers
const APP_WINDOW_IDENTIFIERS: &[&str] = &[
    "MataPass",  // Your window title
];

// Virtual Key Codes
const VK_LBUTTON: i32 = 0x01;
const VK_RBUTTON: i32 = 0x02;
const VK_TAB: i32 = 0x09;
const VK_RETURN: i32 = 0x0D;
const VK_ESCAPE: i32 = 0x1B;
const VK_SPACE: i32 = 0x20;
const VK_LEFT: i32 = 0x25;
const VK_UP: i32 = 0x26;
const VK_RIGHT: i32 = 0x27;
const VK_DOWN: i32 = 0x28;
const VK_SEMICOLON: i32 = 0xBA; // Virtual key code for ';'
const VK_MENU: i32 = 0x12;   // Virtual key code for Alt


const CLICK_DEBOUNCE_TIME: u128 = 50;
const KEYSTROKE_DEBOUNCE_TIME: u128 = 150;


static START_TIME: once_cell::sync::Lazy<Instant> = once_cell::sync::Lazy::new(Instant::now);

fn get_timestamp() -> u128 {
    START_TIME.elapsed().as_millis()
}

#[derive(Debug)]
struct EventEmitter {
    last_click_time: Instant,
    last_keystroke_time: Instant,
    last_key_states: HashMap<i32, bool>,
}

impl EventEmitter {
    fn new() -> Self {
        println!("[INPUT][{}ms] Initializing event emitter", get_timestamp());
        Self {
            last_click_time: Instant::now(),
            last_keystroke_time: Instant::now(),
            last_key_states: HashMap::new(),
        }
    }

    fn emit_event(&self, app_handle: &AppHandle, info: ElementInfo) -> Result<(), Error> {
        println!("[INPUT][{}ms] Emitting {} event at ({:?}, {:?})", 
            get_timestamp(),
            info.action_type, 
            info.x, 
            info.y
        );
        app_handle.emit_all("element_interaction", info)
            .map_err(|e| Error::EmitError(e.to_string()))
    }
}

pub struct InputTracker {
    emitter: EventEmitter,
    automation: IUIAutomation,
}

impl InputTracker {
    pub fn new() -> Result<Self, Error> {
        println!("[INPUT][{}ms] Initializing input tracker", get_timestamp());
        let automation = initialize_automation()?;
        println!("[INPUT][{}ms] UI Automation initialized successfully", get_timestamp());
        
        Ok(Self {
            emitter: EventEmitter::new(),
            automation,
        })
    }

    fn should_skip_window(window_info: &ElementInfo) -> bool {
        println!("\n=== Window Info ===");
        println!("Window Title: {}", window_info.window_title);
        println!("Window Class: {}", window_info.class_name);
        
        let should_skip = APP_WINDOW_IDENTIFIERS.iter().any(|&identifier| {
            let matches = window_info.window_title.to_lowercase().contains(&identifier.to_lowercase());
            println!("Checking identifier: {}", identifier);
            println!("Should skip: {}", matches);
            matches
        });
        
        println!("Final decision - Skip: {}", should_skip);
        println!("=================\n");
        
        should_skip
    }

    fn get_focused_element_position(&self) -> Option<(i32, i32)> {
        unsafe {
            if let Ok(focused_element) = self.automation.GetFocusedElement() {
                if let Ok(rect_variant) = focused_element.GetCurrentPropertyValue(UIA_BoundingRectanglePropertyId) {
                    let rect_array = rect_variant.Anonymous.Anonymous.Anonymous.parray;
                    if !rect_array.is_null() {
                        let rect_data = &*rect_array;
                        if rect_data.rgsabound[0].cElements >= 4 {
                            let elements = std::slice::from_raw_parts(
                                rect_data.pvData as *const f64,
                                4
                            );
                            let x = elements[0] as i32 + (elements[2] as i32 / 2);
                            let y = elements[1] as i32 + (elements[3] as i32 / 2);
                            return Some((x, y));
                        }
                    }
                }
            }
            None
        }
    }

    fn handle_click(&mut self, app_handle: &AppHandle, point: POINT, is_right_click: bool) -> Result<(), Error> {
        // 1. Check if we should process this click (debouncing)
        let now = Instant::now();
        if now.duration_since(self.emitter.last_click_time).as_millis() <= CLICK_DEBOUNCE_TIME {
            return Ok(());
        }
    
        let click_type = if is_right_click { "right_click" } else { "click" };
        println!("[INPUT][{}ms] {} at ({}, {})", get_timestamp(), click_type, point.x, point.y);
        
        // 2. Capture screenshot IMMEDIATELY after detecting click
        // This happens before any processing or UI changes can occur
        let screenshot = capture_screenshot(point.x, point.y);
        
        // 3. Get element info and process the click
        if let Some(mut element_info) = get_element_info(point.x, point.y) {
            if Self::should_skip_window(&element_info) {
                println!("[INPUT][{}ms] Skipping app window click", get_timestamp());
                return Ok(());
            }
    
            println!("[INPUT][{}ms] Clicked {} element", get_timestamp(), element_info.control_type);
            element_info.action_category = ActionCategory::Click;
            element_info.action_type = click_type.to_string();
            // 4. Use the screenshot we captured earlier
            element_info.screenshot = screenshot;
            
            // 5. Small delay to ensure UI state is stable
            std::thread::sleep(std::time::Duration::from_millis(50));
            
            // 6. Emit the event with everything prepared
            self.emitter.emit_event(app_handle, element_info)?;
        }
    
        // 7. Update last click time
        self.emitter.last_click_time = now;
        Ok(())
    }

    fn handle_keystroke(&mut self, app_handle: &AppHandle, key_code: i32, fallback_point: POINT) -> Result<(), Error> {
        let now = Instant::now();
        
        let current_state = unsafe { GetAsyncKeyState(key_code) };
        let key_is_pressed = current_state < 0;
        let was_pressed = self.emitter.last_key_states.get(&key_code).copied().unwrap_or(false);
        
        self.emitter.last_key_states.insert(key_code, key_is_pressed);
        
        if !was_pressed && 
           key_is_pressed && 
           now.duration_since(self.emitter.last_keystroke_time).as_millis() > KEYSTROKE_DEBOUNCE_TIME {
            
            let action_type = match key_code {
                VK_TAB => "tab",
                VK_RETURN => "enter",
                VK_SPACE => "space",
                VK_ESCAPE => "escape",
                VK_LEFT => "arrow_left",
                VK_UP => "arrow_up",
                VK_RIGHT => "arrow_right",
                VK_DOWN => "arrow_down",
                _ => return Ok(()),
            };

            println!("[INPUT][{}ms] Keystroke: {}", get_timestamp(), action_type);

            let (x, y) = self.get_focused_element_position()
                .unwrap_or((fallback_point.x, fallback_point.y));

            if let Some(mut element_info) = get_element_info(x, y) {
                if Self::should_skip_window(&element_info) {
                    return Ok(());
                }

                element_info.action_category = ActionCategory::Keystroke;
                element_info.action_type = action_type.to_string();
                self.emitter.emit_event(app_handle, element_info)?;
            }

            self.emitter.last_keystroke_time = now;
        }

        Ok(())
    }

    fn handle_manual_screenshot(&mut self, app_handle: &AppHandle) -> Result<(), Error> {
        let now = Instant::now();
        if now.duration_since(self.emitter.last_keystroke_time).as_millis() <= KEYSTROKE_DEBOUNCE_TIME {
            return Ok(());
        }
    
        println!("[INPUT][{}ms] Manual screenshot capture", get_timestamp());
    
        // Create element info for manual screenshot
        let element_info = ElementInfo {
            x: None,  // No position for manual screenshot
            y: None,  // No position for manual screenshot
            screen_context: ScreenContext::new(),  // Using ScreenContext struct
            name: "Manual Screenshot".to_string(),
            control_type: "Screenshot".to_string(),
            automation_id: String::new(),
            class_name: String::new(),
            window_title: "Manual Capture".to_string(),
            parent_name: String::new(),
            action_type: "capture".to_string(),
            action_category: ActionCategory::Manual,
            timestamp: chrono::Utc::now().to_rfc3339(),
            screenshot: capture_screenshot(0,0),
            value: String::new(),
            state: String::new(),
            help_text: String::new(),
        };
    
        self.emitter.emit_event(app_handle, element_info)?;
        self.emitter.last_keystroke_time = now;
        Ok(())
    }

    pub fn track_inputs(&mut self, app_handle: &AppHandle) -> Result<(), Error> {
        unsafe {
            let mut point = POINT { x: 0, y: 0 };
            GetCursorPos(&mut point);

            // Track mouse clicks
            if GetAsyncKeyState(VK_LBUTTON) < 0 {
                self.handle_click(app_handle, point, false)?;
            }
            if GetAsyncKeyState(VK_RBUTTON) < 0 {
                self.handle_click(app_handle, point, true)?;
            }

            if (GetAsyncKeyState(VK_MENU) < 0) && (GetAsyncKeyState(VK_SEMICOLON) < 0) {
                self.handle_manual_screenshot(app_handle)?;
            }      

            // Track important keystrokes
            for &key_code in &[
                VK_TAB,
                VK_RETURN,
                VK_SPACE,
                VK_ESCAPE,
                VK_LEFT,
                VK_UP,
                VK_RIGHT,
                VK_DOWN,
            ] {
                self.handle_keystroke(app_handle, key_code, point)?;
            }
        }

        Ok(())
    }
}

pub fn setup_tracking_loop(app_handle: AppHandle, tracking_state: TrackingState) {
    println!("[INPUT][{}ms] Starting input tracking loop", get_timestamp());
    std::thread::spawn(move || {
        let mut tracker = match InputTracker::new() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("[ERROR][{}ms] Failed to initialize input tracker: {}", get_timestamp(), e);
                return;
            }
        };

        loop {
            if tracking_state.is_running() {
                if let Err(e) = tracker.track_inputs(&app_handle) {
                    eprintln!("[ERROR][{}ms] Error tracking inputs: {}", get_timestamp(), e);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });
}
