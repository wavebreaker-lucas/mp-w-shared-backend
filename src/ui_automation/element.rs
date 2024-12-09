#![allow(non_upper_case_globals)]

use windows::core::Result as WindowsResult;
use windows::Win32::UI::Accessibility::*;
use windows::Win32::System::Com::*;
use windows::Win32::Foundation::POINT;
use crate::models::element_info::{ElementInfo, ActionCategory};
use crate::models::screen::ScreenContext;
use crate::ui_automation::utils::variant_to_string;
use crate::ui_automation::window::get_window_title_for_element;
use crate::screenshot::capture_screenshot;
use chrono::Utc;

pub fn get_element_info(x: i32, y: i32) -> Option<ElementInfo> {
    unsafe {
        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL).ok()?;
        let element = automation.ElementFromPoint(POINT { x, y }).ok()?;

        // Try to find the actual interactive element if we hit a container
        let element = find_actual_interactive_element(&element).unwrap_or(element);

        let mut info = ElementInfo {
            x: Some(x),  // Wrap with Some()
            y: Some(y),  // Wrap with Some()
            screen_context: ScreenContext::new(),
            name: String::new(),
            control_type: String::new(),
            automation_id: String::new(),
            class_name: String::new(),
            window_title: get_window_title_for_element(&element),
            parent_name: String::new(),
            action_type: "click".to_string(),
            action_category: ActionCategory::Click,  // Default to Click
            timestamp: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            screenshot: capture_screenshot(x, y),
            value: String::new(),
            state: String::new(),
            help_text: String::new(),
        };

        // Rest of the code remains exactly the same...
        if let Ok(name) = element.GetCurrentPropertyValue(UIA_NamePropertyId) {
            info.name = variant_to_string(name);
        }

        if let Ok(control_type_variant) = element.GetCurrentPropertyValue(UIA_ControlTypePropertyId) {
            if control_type_variant.Anonymous.Anonymous.vt == VARENUM(VT_I4.0) {
                let control_type_id = control_type_variant.Anonymous.Anonymous.Anonymous.lVal;
                info.control_type = super::CONTROL_TYPES
                    .iter()
                    .find(|&&(id, _)| id == control_type_id)
                    .map(|(_, name)| name.to_string())
                    .unwrap_or_else(|| format!("Unknown ({})", control_type_id));
            }
        }

        if let Ok(id) = element.GetCurrentPropertyValue(UIA_AutomationIdPropertyId) {
            info.automation_id = variant_to_string(id);
        }

        if let Ok(class) = element.GetCurrentPropertyValue(UIA_ClassNamePropertyId) {
            info.class_name = variant_to_string(class);
        }

        if let Ok(tree_walker) = automation.ControlViewWalker() {
            if let Ok(parent) = tree_walker.GetParentElement(&element) {
                if let Ok(parent_name) = parent.GetCurrentPropertyValue(UIA_NamePropertyId) {
                    info.parent_name = variant_to_string(parent_name);
                }
            }
        }

        // Get value
        if let Ok(value) = element.GetCurrentPropertyValue(UIA_ValueValuePropertyId) {
            info.value = variant_to_string(value);
        }

        // Get combined state
        let mut states = Vec::new();
        if let Ok(enabled) = element.CurrentIsEnabled() {
            if !Into::<bool>::into(enabled) { states.push("disabled"); }
        }
        if let Ok(selected) = element.GetCurrentPropertyValue(UIA_SelectionItemIsSelectedPropertyId) {
            if selected.Anonymous.Anonymous.vt == VARENUM(VT_BOOL.0) {
                if selected.Anonymous.Anonymous.Anonymous.boolVal.as_bool() {
                    states.push("selected");
                }
            }
        }
        if let Ok(checked) = element.GetCurrentPropertyValue(UIA_ToggleToggleStatePropertyId) {
            if checked.Anonymous.Anonymous.vt == VARENUM(VT_I4.0) {
                match checked.Anonymous.Anonymous.Anonymous.lVal {
                    1 => states.push("checked"),        // ToggleState_On
                    2 => states.push("indeterminate"),  // ToggleState_Indeterminate
                    _ => {}
                }
            }
        }
        info.state = states.join(", ");

        // Get help text
        if let Ok(help) = element.GetCurrentPropertyValue(UIA_HelpTextPropertyId) {
            info.help_text = variant_to_string(help);
        }

        Some(info)
    }
}

// Rest of the functions remain exactly the same...
fn find_actual_interactive_element(element: &IUIAutomationElement) -> WindowsResult<IUIAutomationElement> {
    // Check if current element is already interactive
    if is_interactive_element(element)? {
        return Ok(element.clone());
    }

    // Try to find child interactive elements
    if let Ok(children) = get_interactive_children(element) {
        for child in children {
            if is_interactive_element(&child)? {
                return Ok(child);
            }
        }
    }

    // If no interactive element found, return original element
    Ok(element.clone())
}

pub fn is_interactive_element(element: &IUIAutomationElement) -> WindowsResult<bool> {
    unsafe {
        let control_type = match element.CurrentControlType() {
            Ok(ct) => ct,
            Err(_) => return Ok(false),
        };

        // Check if it's an interactive control type
        Ok(matches!(control_type,
            UIA_ButtonControlTypeId |
            UIA_ComboBoxControlTypeId |
            UIA_MenuItemControlTypeId |
            UIA_TabItemControlTypeId |
            UIA_ListItemControlTypeId |
            UIA_TreeItemControlTypeId |
            UIA_SpinnerControlTypeId |
            UIA_HyperlinkControlTypeId
        ))
    }
}

fn get_interactive_children(element: &IUIAutomationElement) -> WindowsResult<Vec<IUIAutomationElement>> {
    unsafe {
        let mut interactive = Vec::new();
        
        if let Ok(tree_walker) = initialize_automation()?.ControlViewWalker() {
            if let Ok(first_child) = tree_walker.GetFirstChildElement(element) {
                let mut current = Some(first_child);
                while let Some(child) = current {
                    if is_interactive_element(&child)? {
                        interactive.push(child.clone());
                    }
                    
                    current = tree_walker.GetNextSiblingElement(&child).ok();
                }
            }
        }
        
        Ok(interactive)
    }
}

pub fn is_focusable(element: &IUIAutomationElement) -> bool {
    unsafe {
        if let Ok(focusable) = element.GetCurrentPropertyValue(UIA_IsKeyboardFocusablePropertyId) {
            if focusable.Anonymous.Anonymous.vt == VARENUM(VT_BOOL.0) {
                return focusable.Anonymous.Anonymous.Anonymous.boolVal.as_bool();
            }
        }
        false
    }
}

pub fn has_focus(element: &IUIAutomationElement) -> bool {
    unsafe {
        if let Ok(focused) = element.GetCurrentPropertyValue(UIA_HasKeyboardFocusPropertyId) {
            if focused.Anonymous.Anonymous.vt == VARENUM(VT_BOOL.0) {
                return focused.Anonymous.Anonymous.Anonymous.boolVal.as_bool();
            }
        }
        false
    }
}

pub fn initialize_automation() -> WindowsResult<IUIAutomation> {
    unsafe {
        let automation: IUIAutomation = CoCreateInstance(
            &CUIAutomation,
            None,
            CLSCTX_ALL
        )?;
        
        Ok(automation)
    }
}

pub fn get_element_at_point(
    automation: &IUIAutomation,
    point: POINT
) -> WindowsResult<Option<IUIAutomationElement>> {
    unsafe {
        match automation.ElementFromPoint(point) {
            Ok(element) => Ok(Some(element)),
            Err(_) => Ok(None)
        }
    }
}

pub fn get_automation_and_element(point: POINT) -> WindowsResult<Option<(IUIAutomation, IUIAutomationElement)>> {
    let automation = initialize_automation()?;
    
    if let Some(element) = get_element_at_point(&automation, point)? {
        Ok(Some((automation, element)))
    } else {
        Ok(None)
    }
}