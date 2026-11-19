use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use tempfile::{NamedTempFile, TempDir};
use zip::{ZipWriter, write::SimpleFileOptions};

const TARGET: &str = "x86_64-pc-windows-gnu";

pub(super) fn run(repo: &Path, cache: &Path) -> Result<()> {
    let build = repo.join("target");
    fs::create_dir_all(&build)?;
    let scratch = TempDir::new_in(&build)?;
    let stage = scratch.path().join("release");

    println!("Preparing bundled engines...");
    super::install(&stage, cache)?;

    println!("Building Stylebrella for {TARGET}...");
    let version = release_version(repo)?;
    let name = format!("stylebrella-{version}-windows-x64");
    checked(
        cargo(repo)
            .args([
                "build",
                "--release",
                "--locked",
                "-p",
                "stylebrella",
                "--target",
                TARGET,
                "--target-dir",
            ])
            .arg(&build),
    )?;
    super::copy(
        &build.join(TARGET).join("release/stylebrella.exe"),
        &stage.join("stylebrella.exe"),
    )?;
    super::copy(
        &repo.join("LICENSE"),
        &stage.join("licenses/stylebrella.txt"),
    )?;
    super::copy(
        &repo.join("packaging/THIRD-PARTY.md"),
        &stage.join("THIRD-PARTY.md"),
    )?;
    fs::write(stage.join("engines.windows-x64.json"), super::MANIFEST)?;

    println!("Creating and verifying {name}.zip...");
    let dist = repo.join("dist");
    fs::create_dir_all(&dist)?;
    let mut archive = NamedTempFile::new_in(&dist)?;
    let mut zip = ZipWriter::new(archive.as_file_mut());
    add_directory(&mut zip, &stage, &name)?;
    zip.finish()?.flush()?;

    let verification = scratch.path().join("smoke test");
    super::unpack(archive.path(), &verification)?;
    let installation = verification.join(&name);
    checked(
        Command::new(installation.join("stylebrella.exe"))
            .current_dir(&verification)
            .env("PATH", "")
            .args(["--version", "--engines"]),
    )?;
    checked(
        cargo(repo)
            .args([
                "test",
                "--locked",
                "-p",
                "stylebrella-core",
                "--target",
                TARGET,
                "--target-dir",
            ])
            .arg(&build)
            .args(["--test", "bundled_engines", "--", "--ignored"])
            .env("STYLEBRELLA_TEST_INSTALL", &installation),
    )?;

    let sha256 = super::sha256(archive.path())?;
    let destination = dist.join(format!("{name}.zip"));
    archive
        .persist(&destination)
        .with_context(|| format!("publish {}", destination.display()))?;
    println!("ZIP: {}\nSHA-256: {sha256}", destination.display());
    Ok(())
}

fn cargo(repo: &Path) -> Command {
    let mut command = Command::new(env!("CARGO"));
    command.current_dir(repo);
    command
}

fn checked(command: &mut Command) -> Result<()> {
    let status = command
        .status()
        .with_context(|| format!("run {command:?}"))?;
    ensure!(status.success(), "{command:?} failed: {status}");
    Ok(())
}

fn release_version(repo: &Path) -> Result<String> {
    #[derive(Deserialize)]
    struct Metadata {
        packages: Vec<Package>,
    }
    #[derive(Deserialize)]
    struct Package {
        name: String,
        version: String,
    }

    let output = cargo(repo)
        .args(["metadata", "--no-deps", "--locked", "--format-version", "1"])
        .output()
        .context("run cargo metadata")?;
    ensure!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: Metadata =
        serde_json::from_slice(&output.stdout).context("read Cargo metadata")?;
    metadata
        .packages
        .into_iter()
        .find(|package| package.name == "stylebrella")
        .map(|package| package.version)
        .context("Stylebrella package not found")
}

fn add_directory(zip: &mut ZipWriter<&mut File>, directory: &Path, name: &str) -> Result<()> {
    zip.add_directory(format!("{name}/"), SimpleFileOptions::default())?;
    let mut entries = fs::read_dir(directory)?.collect::<io::Result<Vec<_>>>()?;
    entries.sort_unstable_by_key(|entry| entry.file_name());
    for entry in entries {
        let entry_name = entry.file_name();
        let entry_name = entry_name.to_str().context("non-UTF-8 release filename")?;
        let archive_name = format!("{name}/{entry_name}");
        let kind = entry.file_type()?;
        if kind.is_dir() {
            add_directory(zip, &entry.path(), &archive_name)?;
        } else {
            ensure!(
                kind.is_file(),
                "unexpected release entry: {}",
                entry.path().display()
            );
            zip.start_file(
                archive_name,
                SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated),
            )?;
            io::copy(&mut File::open(entry.path())?, zip)?;
        }
    }
    Ok(())
}
