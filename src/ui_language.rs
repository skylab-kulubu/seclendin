use windows::core::PCWSTR;
use windows::Win32::Globalization::{SetThreadPreferredUILanguages, MUI_LANGUAGE_NAME};
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE};
use winreg::RegKey;

pub struct UILanguage {
    // tr-TR --> Turkish
    pub target_lang: String,
    // 041F --> Turkish
    pub target_lang_reg: String,
    pub num_langs: u32,
}

impl UILanguage {
    pub fn change_prefered_lang(&mut self) {
        unsafe {
            let lang = PCWSTR(
                self.target_lang
                    .as_str()
                    .encode_utf16()
                    .collect::<Vec<u16>>()
                    .as_ptr(),
            );
            let mut num_langs = self.num_langs;
            let result =
                SetThreadPreferredUILanguages(MUI_LANGUAGE_NAME, lang, Some(&mut num_langs));

            if result.as_bool() {
                println!("Language changed to {}", self.target_lang);
            } else {
                println!("Failed to change language");
            }
        }
    }

    pub fn change_ui_lang_from_hkey(&mut self) {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = "SYSTEM\\CurrentControlSet\\Control\\Nls\\Language";
        let key = hklm
            .open_subkey_with_flags(path, KEY_WRITE)
            .expect("Registry couldn't open.");
        key.set_value("InstallLanguage", &self.target_lang_reg)
            .expect("Failed to set InstallLanguage.");
        key.set_value("Default", &self.target_lang_reg)
            .expect("Failed to set Default language.");

        println!("System language has been change, it will automaticly activaded before restart.");
    }
}
