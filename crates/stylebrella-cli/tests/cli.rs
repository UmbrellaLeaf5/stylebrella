use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_stylebrella"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn commands_do_not_claim_success_before_implementation() {
    for args in [
        &["check", "src", "--output", "sarif"][..],
        &["lint", "--output", "json"],
        &["format", "--check", "--diff"],
        &["fix", "--diff", "--config", "stylebrella.toml"],
        &["explain", "CPP-HEADER-PRAGMA-ONCE"],
    ] {
        let output = run(args);
        assert_eq!(output.status.code(), Some(2), "{}", args[0]);
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("not implemented yet"));
    }
}

#[test]
fn version_does_not_require_engines() {
    let output = run(&["--version"]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        concat!("stylebrella ", env!("CARGO_PKG_VERSION"))
    );
    assert_eq!(run(&["--engines"]).status.code(), Some(2));
}
