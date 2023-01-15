use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let current_dir = env::current_dir().unwrap();
    for entry in fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let metadata = fs::metadata(entry.path().as_path()).unwrap();
        let permissions = metadata.permissions().mode();
        let rwx = convert_octal_to_rwx(permissions);
        println!("{:?} {:?}", rwx, file_name.to_string_lossy());
    }
}

fn convert_octal_to_rwx(mode: u32) -> String {
    let user = (mode / 64) % 8;
    let group = (mode / 8) % 8;
    let others = mode % 8;
    format!("{}{}{}", to_rwx(user), to_rwx(group), to_rwx(others))
}

fn to_rwx(permission: u32) -> String {
    let is_readable = (permission / 4) > 0;
    let is_writable = ((permission % 4) / 2) > 0;
    let is_executable = ((permission % 4) % 2) > 0;
    format!(
        "{}{}{}",
        if is_readable { "r" } else { "-" },
        if is_writable { "w" } else { "-" },
        if is_executable { "x" } else { "-" }
    )
}
