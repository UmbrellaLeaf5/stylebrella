use clap::{Args, Parser, Subcommand, ValueEnum};
use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;
use stylebrella_core::{BundledEngines, Engine};

#[derive(Parser)]
#[command(
    about = "Unified style checks for Python, Kotlin and C++",
    arg_required_else_help = true,
    args_conflicts_with_subcommands = true
)]
struct Cli {
    /// Print the Stylebrella version.
    #[arg(short = 'V', long)]
    version: bool,

    /// Also run each bundled engine to report its version.
    #[arg(long, requires = "version")]
    engines: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Report formatting differences without writing files (stub).
    Check(Report),
    /// Report lint violations without writing files (stub).
    Lint(Report),
    /// Format sources (stub).
    Format {
        #[command(flatten)]
        input: Input,
        #[arg(long)]
        check: bool,
        #[arg(long)]
        diff: bool,
    },
    /// Apply safe fixes (stub).
    Fix {
        #[command(flatten)]
        input: Input,
        #[arg(long)]
        diff: bool,
    },
    /// Explain a rule (stub).
    Explain { rule_id: String },
}

impl Command {
    fn name(&self) -> &'static str {
        match self {
            Self::Check(_) => "check",
            Self::Lint(_) => "lint",
            Self::Format { .. } => "format",
            Self::Fix { .. } => "fix",
            Self::Explain { .. } => "explain",
        }
    }
}

#[derive(Args)]
struct Input {
    #[arg(value_name = "PATH")]
    paths: Vec<PathBuf>,
    #[arg(long, value_name = "FILE")]
    config: Option<PathBuf>,
}

#[derive(Args)]
struct Report {
    #[command(flatten)]
    input: Input,
    #[arg(long, value_enum, default_value = "text")]
    output: OutputFormat,
}

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Sarif,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        eprintln!("stylebrella: `{}` is not implemented yet", command.name());
        return ExitCode::from(2);
    }

    println!("stylebrella {}", env!("CARGO_PKG_VERSION"));
    if cli.engines
        && let Err(error) = print_engine_versions()
    {
        eprintln!("stylebrella: {error}");
        return ExitCode::from(2);
    }
    ExitCode::SUCCESS
}

fn print_engine_versions() -> Result<(), Box<dyn Error>> {
    let engines = BundledEngines::beside_current_exe()?;
    for engine in Engine::ALL {
        let output = engines
            .command(engine)?
            .arg("--version")
            .output()
            .map_err(|error| format!("cannot launch {}: {error}", engine.name()))?;
        if !output.status.success() {
            return Err(format!(
                "{}: {}: {}",
                engine.name(),
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            )
            .into());
        }
        let text = String::from_utf8_lossy(&output.stdout);
        println!("{}: {}", engine.name(), text.trim());
    }
    Ok(())
}
