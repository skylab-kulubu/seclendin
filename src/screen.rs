extern crate winapi;

use std::mem::zeroed;
use winapi::um::wingdi::DEVMODEW;
use winapi::um::wingdi::{DM_BITSPERPEL, DM_PELSHEIGHT, DM_PELSWIDTH};
use winapi::um::winuser::{ChangeDisplaySettingsW, EnumDisplaySettingsW, CDS_UPDATEREGISTRY};

pub struct Screen {}

impl Screen {
    pub fn change_resulation(&mut self) {
        unsafe {
            let mut devmode: DEVMODEW = zeroed();
            devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

            let mut i = 0;
            let mut min_width = u32::MAX;
            let mut min_height = u32::MAX;
            let mut min_dev_mode: DEVMODEW = zeroed();

            while EnumDisplaySettingsW(std::ptr::null(), i, &mut devmode) != 0 {
                if devmode.dmPelsWidth < min_width && devmode.dmPelsHeight < min_height {
                    min_width = devmode.dmPelsWidth;
                    min_height = devmode.dmPelsHeight;
                    min_dev_mode = devmode;
                }
                i += 1;
            }

            if min_width != u32::MAX && min_height != u32::MAX {
                min_dev_mode.dmFields = DM_PELSWIDTH | DM_PELSHEIGHT | DM_BITSPERPEL;
                if ChangeDisplaySettingsW(&mut min_dev_mode, CDS_UPDATEREGISTRY) != 0 {
                    eprintln!("Failed to change the display settings.");
                } else {
                    println!(
                        "Changed the screen resolution to the lowest available: {}x{}",
                        min_width, min_height
                    );
                }
            } else {
                eprintln!("No display settings found.");
            }
        }
    }
}
