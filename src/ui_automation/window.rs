use windows::Win32::UI::Accessibility::*;
use windows::Win32::System::Com::*;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};
use super::utils::variant_to_string;

pub fn get_window_title_for_element(element: &IUIAutomationElement) -> String {
    unsafe {
        if let Ok(automation_id) = element.GetCurrentPropertyValue(UIA_AutomationIdPropertyId) {
            let auto_id = variant_to_string(automation_id);
            if auto_id == "StartButton" {
                return "Windows Taskbar".to_string();
            }
            if auto_id.starts_with("Appid:") {
                if let Ok(name) = element.GetCurrentPropertyValue(UIA_NamePropertyId) {
                    let name_str = variant_to_string(name);
                    if let Some(idx) = name_str.find(" - ") {
                        return name_str[..idx].to_string();
                    }
                    return name_str;
                }
            }
        }

        if let Ok(automation) = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL) {
            let automation: IUIAutomation = automation;
            if let Ok(tree_walker) = automation.ControlViewWalker() {
                let mut current = element.clone();
                while let Ok(parent) = tree_walker.GetParentElement(&current) {
                    if let Ok(control_type_variant) = parent.GetCurrentPropertyValue(UIA_ControlTypePropertyId) {
                        if control_type_variant.Anonymous.Anonymous.vt == VARENUM(VT_I4.0) {
                            let control_type_id = control_type_variant.Anonymous.Anonymous.Anonymous.lVal;
                            if control_type_id == 50032 {
                                if let Ok(name_variant) = parent.GetCurrentPropertyValue(UIA_NamePropertyId) {
                                    let title = variant_to_string(name_variant);
                                    if !title.is_empty() {
                                        return title;
                                    }
                                }
                            }
                        }
                    }
                    current = parent;
                }
            }
        }
        
        let hwnd = GetForegroundWindow();
        let mut title = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut title);
        if len > 0 {
            String::from_utf16_lossy(&title[..len as usize])
        } else {
            "Windows Taskbar".to_string()
        }
    }
}