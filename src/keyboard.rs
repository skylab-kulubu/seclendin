use windows::{core::*, Win32::UI::Input::KeyboardAndMouse::*};

pub fn change_keyboard_layout() {
    unsafe {
        let _ = LoadKeyboardLayoutA(
            PCSTR("00010439\0".as_ptr()), // Hindi
            KLF_ACTIVATE,
        );
    }
}
