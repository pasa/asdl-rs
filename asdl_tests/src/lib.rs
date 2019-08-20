use std::{
    error::Error,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

const TOOLCHAIN: &str = "stable";

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn run_rust_fmt_check() -> Result<()> {
    match Command::new("rustup")
        .args(&["run", TOOLCHAIN, "--", "cargo", "fmt", "--version"])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
    {
        Ok(status) if status.success() => (),
        _ => install_rustfmt()?,
    };

    run(&format!("rustup run {} -- cargo fmt -- --check", TOOLCHAIN), ".")?;
    Ok(())
}

fn install_rustfmt() -> Result<()> {
    run(&format!("rustup install {}", TOOLCHAIN), ".")?;
    run(&format!("rustup component add rustfmt --toolchain {}", TOOLCHAIN), ".")
}

fn run(cmdline: &str, dir: &str) -> Result<()> {
    do_run(cmdline, dir, |c| {
        c.stdout(Stdio::inherit());
    })
    .map(|_| ())
}

fn do_run<F>(cmdline: &str, dir: &str, mut f: F) -> Result<Output>
where
    F: FnMut(&mut Command),
{
    eprintln!("\nwill run: {}", cmdline);
    let proj_dir = project_root().join(dir);
    let mut args = cmdline.split_whitespace();
    let exec = args.next().unwrap();
    let mut cmd = Command::new(exec);
    f(cmd.args(args).current_dir(proj_dir).stderr(Stdio::inherit()));
    let output = cmd.output()?;
    if !output.status.success() {
        Err(format!("`{}` exited with {}", cmdline, output.status))?;
    }
    Ok(output)
}

pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1).unwrap().to_path_buf()
}
