#![cfg(windows)]

use std::io::Write;
use std::process::{Output, Stdio};
use stylebrella_core::{BundledEngines, Engine};

#[test]
#[ignore = "needs an unpacked Windows release; run via cargo xtask package"]
fn bundled_engines_process_sources() {
    let root = std::env::var_os("STYLEBRELLA_TEST_INSTALL").expect("unpacked ZIP directory");
    let engines = BundledEngines::in_installation(root).unwrap();

    for (engine, args, source, expected) in [
        (
            Engine::Ruff,
            &["format", "--isolated", "-"][..],
            "x=1\n",
            "x = 1",
        ),
        (
            Engine::ClangFormat,
            &["--style=LLVM"][..],
            "int x=1;\n",
            "int x = 1;",
        ),
        (
            Engine::Ktlint,
            &["--stdin", "--format"][..],
            "val x=1\n",
            "val x = 1",
        ),
    ] {
        let mut child = engines
            .command(engine)
            .unwrap()
            .env("PATH", "")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(source.as_bytes())
            .unwrap();
        let output = checked_stdout(engine, child.wait_with_output().unwrap());
        assert!(output.contains(expected), "{}: {output}", engine.name());
    }

    let output = engines
        .command(Engine::ClangTidy)
        .unwrap()
        .env("PATH", "")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/basic.cpp"
        ))
        .args(["--checks=-*,modernize-use-nullptr", "--", "-std=c++17"])
        .output()
        .unwrap();
    assert!(checked_stdout(Engine::ClangTidy, output).contains("[modernize-use-nullptr]"));
}

fn checked_stdout(engine: Engine, output: Output) -> String {
    assert!(
        output.status.success(),
        "{}: {}\n{}\n{}",
        engine.name(),
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
