use winapi::um::wingdi::{DEVMODEA, DM_PELSHEIGHT, DM_PELSWIDTH};
use winapi::um::winuser::{ChangeDisplaySettingsA, DISP_CHANGE_SUCCESSFUL};

pub struct Screen {
    pub width: u32,
    pub height: u32,
}

impl Screen {
    pub fn change_resulation(&mut self) {
        let mut dev_mode: DEVMODEA = unsafe { std::mem::zeroed() };
        dev_mode.dmSize = std::mem::size_of::<DEVMODEA>() as u16;
        dev_mode.dmPelsWidth = self.width;
        dev_mode.dmPelsHeight = self.height;
        dev_mode.dmFields = DM_PELSWIDTH | DM_PELSHEIGHT;

        // Change the display settings
        let result = unsafe { ChangeDisplaySettingsA(&mut dev_mode, 0) };

        if result == DISP_CHANGE_SUCCESSFUL {
            println!("Resolution changed to {}x{}", self.width, self.height);
        } else if result == 2 {
            println!("Enter valid resulation ratio!");
        } else {
            println!("Failed to change resolution, {}", result);
        }
    }
}
