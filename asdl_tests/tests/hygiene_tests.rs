use std::{fs, path::Path};

use term;
use difference::{Difference, Changeset};
use asdl_tests::{project_root, run_rust_fmt_check};

use asdl_tera::generate;

#[test]
fn check_code_formatting() {
    if let Err(error) = run_rust_fmt_check() {
        panic!("{}. Please format the code by running `cargo fmt`", error);
    }
}

const TEMPLATE: &str = "asdl/src/ast/generated.rs.tera";
const MACROS: &str = "asdl/src/ast/macros.tera";
const ASDL: &str = "asdl/src/ast/parser.asdl";
const GENERATED: &str = "asdl/src/ast/generated.rs";

#[test]
fn check_syntax_is_fresh() {
    let template_file = Path::new(TEMPLATE);
    let macros_file = Path::new(MACROS);
    let asdl_file = project_root().join(ASDL);
    let generated_file = project_root().join(GENERATED);
    let asdl = fs::read_to_string(asdl_file).unwrap();
    let current_content = fs::read_to_string(generated_file).unwrap();
    std::env::set_current_dir(project_root()).unwrap();
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
