use windows::Win32::System::Com::{VARIANT, VARENUM, VT_BSTR};
use windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
use windows::Win32::UI::WindowsAndMessaging::{SM_CXSCREEN, SM_CYSCREEN};

pub fn variant_to_string(variant: VARIANT) -> String {
    unsafe {
        if variant.Anonymous.Anonymous.vt == VARENUM(VT_BSTR.0) {
            let bstr = &*variant.Anonymous.Anonymous.Anonymous.bstrVal;
            return bstr.to_string();
        }
        String::new()
    }
}

pub fn get_screen_size() -> (i32, i32) {
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        (width, height)
    }
}

// If you need more detailed screen information, you could also add:
pub fn get_screen_working_area() -> (i32, i32, i32, i32) {
    use windows::Win32::UI::WindowsAndMessaging::{SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN};
    unsafe {
        let x = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let y = GetSystemMetrics(SM_YVIRTUALSCREEN);
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        (x, y, width, height)
    }
}