#[cfg(target_os = "windows")]
use windows::{core::*, Win32::UI::Input::KeyboardAndMouse::*};

#[cfg(target_os = "windows")]
pub fn change_keyboard_layout() {
    unsafe {
        let _ = LoadKeyboardLayoutA(
            PCSTR("00010439\0".as_ptr()), // Hindi
            KLF_ACTIVATE,
        );
    }
}
