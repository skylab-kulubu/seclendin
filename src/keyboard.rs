use std::ptr;
use windows::core::PCWSTR;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    ActivateKeyboardLayout, LoadKeyboardLayoutW, KLF_ACTIVATE,
};
use winreg::{enums::*, RegKey};

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

        if result.unwrap().0 == ptr::null_mut() {
            eprintln!("Klavye düzeni etkinleştirilemedi.");
        }
    }

    pub fn change_keyboard_layout_from_registry(&mut self) {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = "Keyboard Layout\\Preload";
        let (preload_key, _) = hkcu.create_subkey(path).unwrap();
        preload_key.set_value("1", &self.layout).unwrap();
    }
}
