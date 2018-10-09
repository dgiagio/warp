use std::env;
use std::process::Command;
use std::process::Stdio;
use std::path::Path;

#[cfg(target_family = "windows")]
const PATH_SEPARATOR: char = ';';

#[cfg(target_family = "unix")]
const PATH_SEPARATOR: char = ':';

pub fn execute(path: &Path, prog: &str) {
    let path_str = path.as_os_str().to_os_string().into_string().unwrap();
    let path_env = match env::var("PATH") {
        Ok(p) => format!("{}{}{}", &path_str, PATH_SEPARATOR, &p),
        _ => path_str
    };

    let mut args: Vec<String> = env::args().collect();
    args[0] = prog.to_owned();

    trace!("PATH={:?} prog={:?} args={:?}", path_env, prog, args);
    Command::new(prog)
        .env("PATH", path_env)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap_or_else(|_| panic!("{} failed to start", prog))
        .wait()
        .unwrap_or_else(|_| panic!("{} failed to wait", prog));
}
