use anyhow::{Context, Result, bail, ensure};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, ErrorKind};
use std::path::{Component, Path, PathBuf};
use std::time::Duration;
use tempfile::{NamedTempFile, TempDir};
use zip::ZipArchive;

mod package;

const MANIFEST: &str = include_str!("../../engines.windows-x64.json");
const USAGE: &str = "Usage: cargo xtask package\n\
    Or: cargo xtask DESTINATION [CACHE_DIR]\n\
    DESTINATION must not exist. It will contain engines/ and licenses/.\n\
    CACHE_DIR defaults to target/engine-archives in the Stylebrella repository.";

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Manifest {
    ruff: Asset,
    clang_format: Asset,
    clang_tidy: Asset,
    ktlint: Asset,
    java: Asset,
    ruff_license: Asset,
    ktlint_license: Asset,
}

#[derive(Deserialize)]
struct Asset {
    version: Option<String>,
    file: String,
    url: String,
    sha256: String,
}

fn main() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let Some(destination) = args.next() else {
        bail!("{USAGE}");
    };
    if destination == "--help" || destination == "-h" {
        println!("{USAGE}");
        return Ok(());
    }
    ensure!(
        cfg!(all(windows, target_arch = "x86_64")),
        "Windows x64 is required"
    );

    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .context("cannot locate repository root")?;
    let cache = repo.join("target/engine-archives");
    if destination == "package" {
        ensure!(args.next().is_none(), "{USAGE}");
        return package::run(repo, &cache);
    }

    let cache = args.next().map(PathBuf::from).unwrap_or(cache);
    ensure!(args.next().is_none(), "{USAGE}");
    let destination = std::path::absolute(destination)?;
    install(&destination, &cache)?;
    println!("Installed engines in {}", destination.display());
    Ok(())
}

fn install(destination: &Path, cache: &Path) -> Result<()> {
    match destination.symlink_metadata() {
        Ok(_) => bail!("destination already exists: {}", destination.display()),
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error).with_context(|| destination.display().to_string()),
    }
    let parent = destination.parent().context("destination has no parent")?;
    fs::create_dir_all(parent)?;
    fs::create_dir_all(cache)?;

    let manifest: Manifest = serde_json::from_str(MANIFEST).context("invalid engine manifest")?;
    let agent = ureq::Agent::config_builder()
        .timeout_connect(Some(Duration::from_secs(30)))
        .timeout_global(Some(Duration::from_secs(180)))
        .build()
        .new_agent();
    let scratch = TempDir::new_in(parent)?;
    let stage = scratch.path().join("installation");
    let engines = stage.join("engines");
    let licenses = stage.join("licenses");
    let llvm = engines.join("llvm");
    let kotlin = engines.join("kotlin");
    fs::create_dir_all(llvm.join("bin"))?;
    fs::create_dir_all(&kotlin)?;
    fs::create_dir(&licenses)?;

    let ruff = engines.join("ruff");
    unpack(&cached_asset(&agent, cache, &manifest.ruff)?, &ruff)?;
    ensure!(
        ruff.join("ruff.exe").is_file(),
        "Ruff archive has no ruff.exe"
    );

    for (tool, asset) in [
        ("clang-format", &manifest.clang_format),
        ("clang-tidy", &manifest.clang_tidy),
    ] {
        let wheel = scratch.path().join(tool);
        unpack(&cached_asset(&agent, cache, asset)?, &wheel)?;
        let wheel_name = tool.replace('-', "_");
        let data = wheel.join(format!("{wheel_name}/data"));
        let binary = format!("bin/{tool}.exe");
        copy(&data.join(&binary), &llvm.join(&binary))?;
        let version = asset.version.as_deref().context("missing LLVM version")?;
        fs::rename(
            wheel.join(format!("{wheel_name}-{version}.dist-info/licenses")),
            licenses.join(tool),
        )
        .with_context(|| format!("install {tool} licenses"))?;
        if tool == "clang-tidy" {
            fs::rename(data.join("lib"), llvm.join("lib"))
                .context("install Clang resource headers")?;
        }
    }

    let java = scratch.path().join("java");
    unpack(&cached_asset(&agent, cache, &manifest.java)?, &java)?;
    let version = manifest
        .java
        .version
        .as_deref()
        .context("missing Java version")?;
    let java_home = java.join(format!("jdk-{version}-jre"));
    ensure!(
        java_home.join("bin/java.exe").is_file(),
        "unexpected Java runtime layout"
    );
    fs::rename(java_home, kotlin.join("java")).context("install Java runtime")?;
    copy(
        &cached_asset(&agent, cache, &manifest.ktlint)?,
        &kotlin.join("ktlint.jar"),
    )?;

    for asset in [&manifest.ruff_license, &manifest.ktlint_license] {
        let source = cached_asset(&agent, cache, asset)?;
        copy(&source, &licenses.join(&asset.file))?;
    }

    fs::rename(&stage, destination)
        .with_context(|| format!("publish installation at {}", destination.display()))
}

fn cached_asset(agent: &ureq::Agent, cache: &Path, asset: &Asset) -> Result<PathBuf> {
    let mut components = Path::new(&asset.file).components();
    ensure!(
        matches!(components.next(), Some(Component::Normal(name)) if name == OsStr::new(&asset.file))
            && components.next().is_none(),
        "invalid cache filename: {}",
        asset.file
    );
    let path = cache.join(&asset.file);
    match sha256(&path) {
        Ok(hash) if hash.eq_ignore_ascii_case(&asset.sha256) => return Ok(path),
        Ok(_) => eprintln!("Re-downloading {} (cached SHA-256 mismatch)", asset.file),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            eprintln!("Downloading {}", asset.file)
        }
        Err(error) => {
            return Err(error).with_context(|| format!("read cache entry {}", path.display()));
        }
    }

    let mut response = agent
        .get(&asset.url)
        .call()
        .with_context(|| format!("download {}", asset.url))?;
    let mut partial = NamedTempFile::new_in(cache)?;
    io::copy(&mut response.body_mut().as_reader(), &mut partial)
        .with_context(|| format!("download {}", asset.file))?;
    let hash = sha256(partial.path())?;
    ensure!(
        hash.eq_ignore_ascii_case(&asset.sha256),
        "SHA-256 mismatch: {}",
        asset.file
    );
    partial
        .persist(&path)
        .with_context(|| format!("cache {}", asset.file))?;
    Ok(path)
}

fn sha256(path: &Path) -> io::Result<String> {
    let mut hash = Sha256::new();
    io::copy(&mut File::open(path)?, &mut hash)?;
    Ok(format!("{:x}", hash.finalize()))
}

fn unpack(archive: &Path, destination: &Path) -> Result<()> {
    ZipArchive::new(File::open(archive)?)
        .and_then(|mut zip| zip.extract(destination))
        .with_context(|| format!("extract {}", archive.display()))
}

fn copy(source: &Path, destination: &Path) -> Result<()> {
    fs::copy(source, destination).with_context(|| format!("copy {}", source.display()))?;
    Ok(())
}
