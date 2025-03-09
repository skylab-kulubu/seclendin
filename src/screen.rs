extern crate winapi;

#[allow(unused_imports)]
use std::ffi::CString;

#[allow(unused_imports)]
use std::ptr::null_mut;
#[allow(unused_imports)]
use winapi::um::wingdi::{DEVMODEA, DM_PELSHEIGHT, DM_PELSWIDTH};

#[allow(unused_imports)]
use winapi::um::winuser::{ChangeDisplaySettingsA, DISP_CHANGE_SUCCESSFUL};

pub struct Screen {
    pub width: u32,
    pub height: u32,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Screen {
        Screen { width, height }
    }

    pub fn change_resulation(&mut self) {
        let mut dev_mode: DEVMODEA = unsafe { std::mem::zeroed() };
        dev_mode.dmSize = std::mem::size_of::<DEVMODEA>() as u16;
        dev_mode.dmPelsWidth = self.width;
        dev_mode.dmPelsHeight = self.height;
        dev_mode.dmFields = winapi::um::wingdi::DM_PELSWIDTH | winapi::um::wingdi::DM_PELSHEIGHT;

        // Change the display settings
        let result = unsafe { ChangeDisplaySettingsA(&mut dev_mode, 0) };

        if result == DISP_CHANGE_SUCCESSFUL {
            println!("Resolution changed to {}x{}", self.width, self.height);
        } else {
            println!("Failed to change resolution");
        }
    }
}
