use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        let username = get_effective_username();
        if username.is_empty() {
            println!("error: Unable to fetch effective user name!");
            exit(1);
        }
        println!("{}", &username);
        exit(0);
    } else if args.len() == 1 {
        let arg = &args[0];
        if arg == "-version" || arg == "-v" {
            println!("{}", VERSION);
            exit(0);
        } else if arg == "-help" || arg == "-h" {
            println!("{}", VERSION);
            println!("{}", HELP);
            exit(0);
        } else {
            println!("error: Unknown command!");
            println!();
            println!("try help for usage:");
            println!("  whoami -help");
            exit(1);
        }
    } else {
        println!("error: Unknown command!");
        println!();
        println!("try help for usage:");
        println!("  whoami -help");
        exit(1);
    }
}

const VERSION: &str = "whoami 1.0.0";
const HELP: &str = r#"A reimplementation of the Linux command 'whoami' in Rust.
- author: csm
- license: MIT
- about:
    whoami displays the user name associated with the 
    current effective user ID. Same as 'id -un'.
- commands:
    whoami <no args>     -> Print effective userid
    whoami -version, -v  -> Print whoami version
    whoami -help,    -h  -> Print whoami commands
"#;

#[cfg(unix)]
fn get_effective_username() -> String {
    use std::ffi::CStr;

    unsafe {
        let uid = libc::geteuid();
        let pw = libc::getpwuid(uid);
        if !pw.is_null() {
            return CStr::from_ptr((*pw).pw_name).to_string_lossy().into_owned();
        }
        "".to_string()
    }
}
