use std::env;
use std::process::Command;
use std::process::Stdio;
use std::path::Path;
use std::io;

#[cfg(target_family = "windows")]
const PATH_SEPARATOR: char = ';';

#[cfg(target_family = "unix")]
const PATH_SEPARATOR: char = ':';

pub fn execute(target: &Path) -> io::Result<i32> {
    let target_dir = target.parent()
        .and_then(|dir| dir.to_str())
        .expect("Unable to construct target directory");
    let target_file_name = target.file_name()
        .and_then(|file_name| file_name.to_str())
        .expect("Unable to identify target file name");
    trace!("target={:?}", target);
    trace!("target_dir={:?}", target_dir);
    trace!("target_file_name={:?}", target_file_name);

    let current_path_env = env::var("PATH").unwrap_or(String::new());
    let path_env = format!("{}{}{}", target_dir, PATH_SEPARATOR, current_path_env);
    trace!("path_env={:?}", path_env);

    let args: Vec<String> = env::args().skip(1).collect();
    trace!("args={:?}", args);

    Ok(Command::new(target_file_name)
        .env("PATH", path_env)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?
        .code().unwrap_or(1))
}
