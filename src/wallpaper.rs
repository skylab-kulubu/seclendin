use crate::util::get_desktop_path_using_dirs;
pub struct Wallpaper {
    pub url: String,
    pub image_path: String,
}
impl Wallpaper {
    pub fn new(url: String) -> Self {
        let desktop_path = get_desktop_path_using_dirs().unwrap();
        let target_name = String::from("downloaded_file.txt");
        let binding = String::from(desktop_path.join(target_name).to_str().unwrap());
        let image_path = binding.as_str();
        Wallpaper {
            url: String::from(url),
            image_path: String::from(image_path),
        }
    }

    pub async fn set_wallpaper(&mut self) {
        wallpaper::set_from_path(self.image_path.as_str())
            .expect("Something went wrong while setting wallpaper.");
        wallpaper::set_mode(wallpaper::Mode::Crop)
            .expect("Something went wrong while setting wallpaper mode.");
    }
}
