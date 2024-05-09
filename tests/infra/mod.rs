use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[macro_export]
macro_rules! success_tests {
    ($($name:ident: $expected:literal),* $(,)?) => {
        $(
        #[test]
        fn $name() {
            $crate::infra::run_success_test(stringify!($name), $expected);
        }
        )*
    }
}
#[macro_export]
macro_rules! failure_tests {
    ($($name:ident: $expected:literal),* $(,)?) => {
        $(
        #[test]
        fn $name() {
            $crate::infra::run_failure_test(stringify!($name), $expected);
        }
        )*
    }
}

fn compile(name: &str) -> Result<(), String> {
    // Build the project
    let status = Command::new("cargo")
        .arg("build")
        .status()
        .expect("could not run cargo");
    assert!(status.success(), "could not build the project");

    // Run the compiler
    let boa: PathBuf = ["target", "debug", "lec"].iter().collect();
    let output = Command::new(&boa)
        .arg(&mk_path(name, Ext::Snek))
        .arg(&mk_path(name, Ext::Asm))
        .output()
        .expect("could not run the compiler");
    if !output.status.success() {
        return Err(String::from_utf8(output.stderr).unwrap());
    }

    // Assemble and link
    let output = Command::new("make")
        .arg(&mk_path(name, Ext::Run))
        .output()
        .expect("could not run make");
    assert!(output.status.success(), "linking failed");

    Ok(())
}

pub(crate) fn run_success_test(name: &str, expected: &str) {
    if let Err(err) = compile(name) {
        panic!(
            "expected a successful compilation, but got an error: `{}`",
            err
        );
    }

    let output = Command::new(&mk_path(name, Ext::Run)).output().unwrap();
    assert!(
        output.status.success(),
        "unexpected error when running the compiled program: `{}`",
        std::str::from_utf8(&output.stderr).unwrap(),
    );
    let actual_output = String::from_utf8(output.stdout).unwrap();
    let actual_output = actual_output.trim();
    let expected_output = expected.trim();
    if expected_output != actual_output {
        eprintln!(
            "output differed!\n{}",
            prettydiff::diff_lines(actual_output, expected_output)
        );
        panic!("test failed");
    }
}

pub(crate) fn _run_failure_test(name: &str, expected: &str) {
    let Err(actual_err) = compile(name) else {
        panic!("expected a failure, but compilation succeeded");
    };
    assert!(
        actual_err.contains(expected.trim()),
        "the reported error message does not match",
    );
}

fn mk_path(name: &str, ext: Ext) -> PathBuf {
    Path::new("tests").join(format!("{name}.{ext}"))
}

#[derive(Copy, Clone)]
enum Ext {
    Snek,
    Asm,
    Run,
}

impl std::fmt::Display for Ext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ext::Snek => write!(f, "snek"),
            Ext::Asm => write!(f, "s"),
            Ext::Run => write!(f, "run"),
        }
    }
}
