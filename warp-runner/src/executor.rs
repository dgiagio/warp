use std::env;
use std::io;
#[cfg(target_family = "unix")]
use std::fs;
#[cfg(target_family = "unix")]
use std::fs::Permissions;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

pub fn execute(target: &Path) -> io::Result<i32> {
    trace!("target={:?}", target);

    let args: Vec<String> = env::args().skip(1).collect();
    trace!("args={:?}", args);

    do_execute(target, &args)
}

#[cfg(target_family = "unix")]
fn ensure_executable(target: &Path) {
    let perms = Permissions::from_mode(0o770);
    fs::set_permissions(target, perms).unwrap();
}

#[cfg(target_family = "unix")]
fn do_execute(target: &Path, args: &[String]) -> io::Result<i32> {
    ensure_executable(target);

    Ok(Command::new(target)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?
        .code().unwrap_or(1))
}

#[cfg(target_family = "windows")]
fn is_script(target: &Path) -> bool {
    const SCRIPT_EXTENSIONS: &[&str] = &["bat", "cmd"];
    SCRIPT_EXTENSIONS.contains(
        &target.extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase().as_str())
}

#[cfg(target_family = "windows")]
fn do_execute(target: &Path, args: &[String]) -> io::Result<i32> {
    let target_str = target.as_os_str().to_str().unwrap();

    if is_script(target) {
        let mut cmd_args = Vec::with_capacity(args.len() + 2);
        cmd_args.push("/c".to_string());
        cmd_args.push(target_str.to_string());
        cmd_args.extend_from_slice(&args);

        Ok(Command::new("cmd")
            .args(cmd_args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?
            .code().unwrap_or(1))
    } else {
        Ok(Command::new(target)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
            .wait()?
            .code().unwrap_or(1))
    }
}