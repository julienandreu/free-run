use platform_dirs::AppDirs;
use registry::Hive;
use std::fs::remove_file;

fn main() {
    let os_type = os_info::get().os_type();

    match os_type {
        os_info::Type::Windows => {
            println!("OS recognized as {}, removing trial...", os_type);

            Hive::CurrentUser
                .delete(r"HKEY_CURRENT_USER\Software\JavaSoft", true)
                .unwrap_or_default();

            let jetbrains_app_dirs = AppDirs::new(Some(r"Jetbrains"), true).unwrap();

            let path = jetbrains_app_dirs.config_dir.join("PermanentUserId");

            remove_file(&path).unwrap_or_default();

            println!("Trial successfully reset, enjoy!");
        }
        _ => {
            println!("Unrecognized OS type: {}", os_type);
        }
    }
}
