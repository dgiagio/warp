extern crate dirs;

#[macro_use]
extern crate log;
extern crate simple_logger;

use std::env;
use std::error::Error;
use std::ffi::*;
use std::fs;
use std::io;
use std::path::*;
use log::Level;

mod extractor;
mod executor;

static PROG_BUF: &'static [u8] = b"tVQhhsFFlGGD3oWV4lEPST8I8FEPP54IM0q7daes4E1y3p2U2wlJRYmWmjPYfkhZ0PlT14Ls0j8fdDkoj33f2BlRJavLj3mWGibJsGt5uLAtrCDtvxikZ8UX2mQDCrgE\0";

fn prog() -> &'static str {
    let nul_pos = PROG_BUF.iter()
        .position(|elem| *elem == b'\0')
        .expect("PROG_BUF has no NUL terminator");

    let slice = &PROG_BUF[..(nul_pos + 1)];
    CStr::from_bytes_with_nul(slice)
        .expect("Can't convert PROG_BUF slice to CStr")
        .to_str()
        .expect("Can't convert PROG_BUF CStr to str")
}

fn cache_path(prog: &str) -> PathBuf {
    dirs::data_local_dir()
        .expect("No data local dir found")
        .join("warp")
        .join("packages")
        .join(prog)
}

fn extract(exe_path: &Path, cache_path: &Path) -> io::Result<()> {
    fs::remove_dir_all(cache_path).ok();
    extractor::extract_to(&exe_path, &cache_path)?;
    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    if env::var("WARP_TRACE").is_ok() {
        simple_logger::init_with_level(Level::Trace)?;
    }

    let prog = prog();
    let cache_path = cache_path(prog);
    let exe_path = env::current_exe()?;
    trace!("prog={:?}, cache_path={:?}, exe_path={:?}", prog, cache_path, exe_path);

    match fs::metadata(&cache_path) {
        Ok(cache) => {
            if cache.modified()? >= fs::metadata(&exe_path)?.modified()? {
                trace!("cache is up-to-date");
            } else {
                trace!("cache is outdated");
                extract(&exe_path, &cache_path)?;
            }
        }
        Err(_) => {
            trace!("cache not found");
            extract(&exe_path, &cache_path)?;
        }
    }

    executor::execute(&cache_path, &prog);
    Ok(())
}
