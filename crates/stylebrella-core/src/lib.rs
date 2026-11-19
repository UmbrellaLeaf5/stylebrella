use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Engine {
    Ruff,
    ClangFormat,
    ClangTidy,
    Ktlint,
}

impl Engine {
    pub const ALL: [Self; 4] = [Self::Ruff, Self::ClangFormat, Self::ClangTidy, Self::Ktlint];

    pub const fn name(self) -> &'static str {
        match self {
            Self::Ruff => "ruff",
            Self::ClangFormat => "clang-format",
            Self::ClangTidy => "clang-tidy",
            Self::Ktlint => "ktlint",
        }
    }
}

/// Resolves the tools shipped under `engines/` in an unpacked release.
#[derive(Debug)]
pub struct BundledEngines {
    root: PathBuf,
}

impl BundledEngines {
    pub fn beside_current_exe() -> io::Result<Self> {
        let exe = std::env::current_exe()?;
        let root = exe.parent().ok_or_else(|| {
            io::Error::other("cannot locate Stylebrella's installation directory")
        })?;
        Self::in_installation(root)
    }

    pub fn in_installation(root: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            root: std::path::absolute(root)?.join("engines"),
        })
    }

    /// Returns a command to which callers can add engine arguments and set stdio.
    /// Tools are looked up by absolute path, with no system-tool fallback.
    pub fn command(&self, engine: Engine) -> io::Result<Command> {
        let executable = match engine {
            Engine::Ruff => "ruff/ruff.exe",
            Engine::ClangFormat => "llvm/bin/clang-format.exe",
            Engine::ClangTidy => "llvm/bin/clang-tidy.exe",
            Engine::Ktlint => "kotlin/java/bin/java.exe",
        };
        let mut command = Command::new(self.require_file(engine, executable)?);
        if engine == Engine::Ktlint {
            command
                .arg("-jar")
                .arg(self.require_file(engine, "kotlin/ktlint.jar")?)
                .env_remove("JDK_JAVA_OPTIONS")
                .env_remove("JAVA_TOOL_OPTIONS")
                .env_remove("_JAVA_OPTIONS");
        }
        Ok(command)
    }

    fn require_file(&self, engine: Engine, relative: &str) -> io::Result<PathBuf> {
        let path = self.root.join(relative);
        match path.metadata() {
            Ok(metadata) if metadata.is_file() => Ok(path),
            result => Err(io::Error::new(
                result.err().map_or(io::ErrorKind::NotFound, |e| e.kind()),
                format!(
                    "{}: expected a bundled file at {}",
                    engine.name(),
                    path.display()
                ),
            )),
        }
    }
}
