extern crate clap;
extern crate dirs;
extern crate reqwest;
extern crate tempdir;
extern crate tar;
extern crate flate2;

use clap::{App, AppSettings, Arg};
use std::process;
use std::path::PathBuf;
use std::fs;
use std::io::copy;
use tempdir::TempDir;
use std::path::Path;
use std::io;
use std::error::Error;
use std::io::Write;
use std::io::Read;
use flate2::write::GzEncoder;
use flate2::Compression;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const SUPPORTED_ARCHS: &[&str] = &["linux-x64", "windows-x64", "macos-x64"];
const RUNNER_URL_TEMPLATE: &str = "https://github.com/dgiagio/warp/releases/download/v$VERSION$/$ARCH$.warp-runner";
const RUNNER_MAGIC: &[u8] = b"tVQhhsFFlGGD3oWV4lEPST8I8FEPP54IM0q7daes4E1y3p2U2wlJRYmWmjPYfkhZ0PlT14Ls0j8fdDkoj33f2BlRJavLj3mWGibJsGt5uLAtrCDtvxikZ8UX2mQDCrgE\0";

/// Print a message to stderr and exit with error code 1
macro_rules! bail {
    () => (process::exit(1));
    ($($arg:tt)*) => ({
        eprint!("{}\n", format_args!($($arg)*));
        process::exit(1);
    })
}

fn runners_dir() -> PathBuf {
    dirs::data_local_dir()
        .expect("No data local dir found")
        .join("warp")
        .join("runners")
        .join(VERSION)
}

fn runner_url(arch: &str) -> String {
    let ext = if cfg!(target_family = "windows") { ".exe" } else { "" };
    RUNNER_URL_TEMPLATE
        .replace("$VERSION$", VERSION)
        .replace("$ARCH$", arch) + ext
}

fn patch_runner(runner_exec: &Path, new_runner_exec: &Path, exec_name: &str) -> io::Result<()> {
    // Read runner executable in memory
    let mut buf = vec![];
    fs::File::open(runner_exec)?
        .read_to_end(&mut buf)?;

    // Set the correct target executable name into the local magic buffer
    let magic_len = RUNNER_MAGIC.len();
    let mut new_magic = vec![0; magic_len];
    new_magic[..exec_name.len()].clone_from_slice(exec_name.as_bytes());

    // Find the magic buffer offset inside the runner executable
    let mut offs_opt = None;
    for (i, chunk) in buf.windows(magic_len).enumerate() {
        if chunk == RUNNER_MAGIC {
            offs_opt = Some(i);
            break;
        }
    }

    if offs_opt.is_none() {
        return Err(io::Error::new(io::ErrorKind::Other, "no magic found inside runner"))
    }

    // Replace the magic with the new one that points to the target executable
    let offs = offs_opt.unwrap();
    buf[offs..offs + magic_len].clone_from_slice(&new_magic);

    // Write patched runner to disk
    fs::File::create(&new_runner_exec)?
        .write_all(&buf)?;

    Ok(())
}

fn create_tgz(dir: &Path, out: &Path) -> io::Result<()> {
    let f = fs::File::create(out)?;
    let gz = GzEncoder::new(f, Compression::best());
    let mut tar = tar::Builder::new(gz);
    tar.append_dir_all(".", dir)?;
    Ok(())
}

fn create_app(runner_exec: &Path, tgz_path: &Path, out: &Path) -> io::Result<()> {
    let mut outf = fs::File::create(out)?;
    let mut runnerf = fs::File::open(runner_exec)?;
    let mut tgzf = fs::File::open(tgz_path)?;
    copy(&mut runnerf, &mut outf)?;
    copy(&mut tgzf, &mut outf)?;
    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    let args = App::new(APP_NAME)
        .settings(&[AppSettings::ArgRequiredElseHelp, AppSettings::ColoredHelp])
        .version(VERSION)
        .author(AUTHOR)
        .about("Create self-contained single binary application")
        .arg(Arg::with_name("arch")
            .short("a")
            .long("arch")
            .value_name("arch")
            .help(&format!("Sets the architecture. Supported: {:?}", SUPPORTED_ARCHS))
            .display_order(1)
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input_dir")
            .short("i")
            .long("input_dir")
            .value_name("input_dir")
            .help("Sets the input directory containing the application and dependencies")
            .display_order(2)
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("exec")
            .short("e")
            .long("exec")
            .value_name("exec")
            .help("Sets the application executable file name")
            .display_order(3)
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("output")
            .help("Sets the resulting self-contained application file name")
            .display_order(4)
            .takes_value(true)
            .required(true))
        .get_matches();

    let arch = args.value_of("arch").unwrap();
    if !SUPPORTED_ARCHS.contains(&arch) {
        bail!("Unknown architecture specified: {}, supported: {:?}", arch, SUPPORTED_ARCHS);
    }

    let input_dir = Path::new(args.value_of("input_dir").unwrap());
    if fs::metadata(input_dir).is_err() {
        bail!("Cannot access specified input directory {:?}", input_dir);
    }

    let exec_name = args.value_of("exec").unwrap();
    if exec_name.len() >= RUNNER_MAGIC.len() {
        bail!("Executable name is too long, please consider using a shorter name");
    }

    let exec_path = Path::new(input_dir).join(exec_name);
    match fs::metadata(&exec_path) {
        Err(_) => {
            bail!("Cannot find file {:?}", exec_path);
        },
        Ok(metadata) => {
            if !metadata.is_file() {
                bail!("{:?} isn't a file", exec_path);
            }
        }
    }

    let runners_dir = runners_dir();
    fs::create_dir_all(&runners_dir)?;

    let runner_exec = runners_dir.join(arch);
    if !runner_exec.exists() {
        let url = runner_url(arch);
        println!("Downloading runner from {}...", url);
        let mut response = reqwest::get(&url)?.error_for_status()?;
        let mut f = fs::File::create(&runner_exec)?;
        copy(&mut response, &mut f)?;
    }

    let tmp_dir = TempDir::new(APP_NAME)?;
    let new_runner_exec = tmp_dir.path().join("runner");
    patch_runner(&runner_exec, &new_runner_exec, &exec_name)?;

    println!("Compressing input directory {:?}...", input_dir);
    let tgz_path = tmp_dir.path().join("input.tgz");
    create_tgz(&input_dir, &tgz_path)?;

    let exec_name = Path::new(args.value_of("output").unwrap());
    println!("Creating self-contained application binary {:?}...", exec_name);
    create_app(&new_runner_exec, &tgz_path, &exec_name)?;

    println!("All done");
    Ok(())
}
