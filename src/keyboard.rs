use std::ptr;
use windows::core::PCWSTR;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    ActivateKeyboardLayout, LoadKeyboardLayoutW, HKL, KLF_ACTIVATE,
};

pub struct Keyboard {
    pub layout: String,
}

impl Keyboard {
    pub fn change_keyboard_layout(&mut self) {
        let hkl = unsafe {
            LoadKeyboardLayoutW(
                PCWSTR(
                    self.layout
                        .as_str()
                        .encode_utf16()
                        .collect::<Vec<u16>>()
                        .as_ptr(),
                ),
                windows::Win32::UI::Input::KeyboardAndMouse::ACTIVATE_KEYBOARD_LAYOUT_FLAGS(0),
            )
        };

        let result = unsafe { ActivateKeyboardLayout(hkl.unwrap(), KLF_ACTIVATE) };
    }
}
