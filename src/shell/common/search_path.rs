use std::env;
use std::path::Path;

pub fn is_valid_program_name(name: &str) -> bool {
    let executable = Path::new(name);
    if let Some(paths) = env::var_os("PATH") {
        for dir in env::split_paths(&paths).into_iter() {
            let full_path = dir.join(executable);
            if full_path.is_file() {
                return true;
            }
        }
    }

    return false;
}
