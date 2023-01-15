use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let current_dir = env::current_dir().unwrap();
    for entry in fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let metadata = fs::metadata(entry.path()).unwrap();
        let permissions = metadata.permissions().mode();
        let rwx = convert_octal_to_rwx(permissions);
        println!("{:?} {:?}", rwx, file_name.to_string_lossy());
    }
}

fn convert_octal_to_rwx(mode: u32) -> String {
    let mut permissions = String::new();
    permissions.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    permissions.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    permissions.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    permissions
}
