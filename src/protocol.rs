#[cfg(windows)]
use std::io;
#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;

#[cfg(windows)]
pub fn register_protocol_handler() -> io::Result<()> {
    let exe_path = std::env::current_exe()?;
    let exe_path_str = exe_path.to_string_lossy();

    // Since we're installing for all users, use HKEY_CLASSES_ROOT
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let (key, _) = hkcr.create_subkey("matapass")?;
    
    key.set_value("", &"URL:MataPass Protocol")?;
    key.set_value("URL Protocol", &"")?;

    let (icon_key, _) = key.create_subkey("DefaultIcon")?;
    icon_key.set_value("", &format!("{},0", exe_path_str))?;

    let (shell_key, _) = key.create_subkey(r"shell\open\command")?;
    shell_key.set_value("", &format!("\"{}\" \"%1\"", exe_path_str))?;

    Ok(())
}

#[cfg(windows)]
#[allow(dead_code)]
pub fn unregister_protocol_handler() -> io::Result<()> {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    hkcr.delete_subkey_all("matapass")?;
    Ok(())
}

#[cfg(not(windows))]
pub fn register_protocol_handler() -> std::io::Result<()> {
    Ok(()) // Do nothing on non-Windows platforms
}

#[cfg(not(windows))]
pub fn unregister_protocol_handler() -> std::io::Result<()> {
    Ok(()) // Do nothing on non-Windows platforms
}