use std::{
    fs,
    error::Error,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

use term;
use difference::{Difference, Changeset};

use asdl_rs::generate;

const TOOLCHAIN: &str = "stable";

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[test]
fn check_code_formatting() {
    if let Err(error) = run_rust_fmt_check() {
        panic!("{}. Please format the code by running `cargo fmt`", error);
    }
}

const TEMPLATE: &str = "src/parser/generated.rs.tera";
const MACROS: &str = "src/parser/macros.tera";
const ASDL: &str = "src/parser/parser.asdl";
const GENERATED: &str = "src/parser/generated.rs";

#[test]
fn check_syntax_is_fresh() {
    let template_file = Path::new(TEMPLATE);
    let macros_file = Path::new(MACROS);
    let asdl_file = project_root().join(ASDL);
    let generated_file = project_root().join(GENERATED);
    let asdl = fs::read_to_string(asdl_file).unwrap();
    let current_content = fs::read_to_string(generated_file).unwrap();
    let new_content = generate(&asdl, &vec![macros_file, template_file]).unwrap();
    let changeset = Changeset::new(&new_content, &current_content, "\n");
    if changeset.diffs.len() == 1 && changeset.diffs[0] == Difference::Same(new_content) {
        return;
    }
    let mut t = term::stdout().unwrap();

    for diff in changeset.diffs {
        match diff {
            Difference::Same(ref x) => {
                t.reset().unwrap();
                writeln!(t, " {}", x).unwrap();
            }
            Difference::Add(ref x) => {
                t.fg(term::color::GREEN).unwrap();
                writeln!(t, "+{}", x).unwrap();
            }
            Difference::Rem(ref x) => {
                t.fg(term::color::RED).unwrap();
                writeln!(t, "-{}", x).unwrap();
            }
        }
    }
    t.reset().unwrap();
    t.flush().unwrap();
    assert!(
        false,
        "Generated syntax is out of date. See the diff above. Please generate new one with 'cargo gen-syntax' command."
    );
}

fn run_rust_fmt_check() -> Result<()> {
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
    Path::new(&env!("CARGO_MANIFEST_DIR")).to_path_buf()
}
