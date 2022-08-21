use anyhow::{anyhow, Context};
use clap::CommandFactory;
use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Generator, Shell};
use nix::fcntl::posix_fadvise;
use nix::fcntl::PosixFadviseAdvice;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Apply advice of POSIX_FADV_NORMAL
    #[clap(display_order = 1, name = "normal")]
    Normal(AdviseInfo),
    /// Apply advice of POSIX_FADV_SEQUENTIAL
    #[clap(display_order = 2, name = "sequential")]
    Sequential(AdviseInfo),
    /// Apply advice of POSIX_FADV_RANDOM
    #[clap(display_order = 3, name = "random")]
    Random(AdviseInfo),
    /// Apply advice of POSIX_FADV_NOREUSE
    #[clap(display_order = 4, name = "noreuse")]
    NoReuse(AdviseInfo),
    /// Apply advice of POSIX_FADV_WILLNEED
    #[clap(display_order = 5, name = "willneed")]
    WillNeed(AdviseInfo),
    /// Apply advice of POSIX_FADV_DONTNEED
    #[clap(display_order = 6, name = "dontneed")]
    DontNeed(AdviseInfo),
    /// Generate code for completion
    #[clap(display_order = 7, name = "completion")]
    Completion {
        /// Target shell to create completion code
        #[clap(long, short, arg_enum)]
        shell: Shell,
    },
}

#[derive(Args)]
struct AdviseInfo {
    /// Filename advice is applied
    #[clap(value_parser, value_name = "FILE")]
    filename: PathBuf,
    /// Offset of a range advice is applied
    #[clap(default_value_t = 0, value_parser = clap::value_parser!(i64).range(0..))]
    offset: i64,
    /// Length of a range advice is applied [default: The size of FILE]
    #[clap(value_parser = clap::value_parser ! (i64).range(0..))]
    len: Option<i64>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FadviseType {
    Normal,
    Sequential,
    Random,
    NoReuse,
    WillNeed,
    DontNeed,
}

impl std::fmt::Display for FadviseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FadviseType::Normal => write!(f, "POSIX_FADV_NORMAL"),
            FadviseType::Sequential => write!(f, "POSIX_FADV_SEQUENTIAL"),
            FadviseType::Random => write!(f, "POSIX_FADV_RANDOM"),
            FadviseType::NoReuse => write!(f, "POSIX_FADV_NOREUSE"),
            FadviseType::WillNeed => write!(f, "POSIX_FADV_WILLNEED"),
            FadviseType::DontNeed => write!(f, "POSIX_FADV_DONTNEED"),
        }
    }
}

impl From<FadviseType> for PosixFadviseAdvice {
    fn from(advise: FadviseType) -> Self {
        match advise {
            FadviseType::Normal => PosixFadviseAdvice::POSIX_FADV_NORMAL,
            FadviseType::Sequential => PosixFadviseAdvice::POSIX_FADV_SEQUENTIAL,
            FadviseType::Random => PosixFadviseAdvice::POSIX_FADV_RANDOM,
            FadviseType::NoReuse => PosixFadviseAdvice::POSIX_FADV_NOREUSE,
            FadviseType::WillNeed => PosixFadviseAdvice::POSIX_FADV_WILLNEED,
            FadviseType::DontNeed => PosixFadviseAdvice::POSIX_FADV_DONTNEED,
        }
    }
}

fn print_completer<G: Generator>(generator: G) -> anyhow::Result<()> {
    let mut app = Cli::into_app();
    let name = app.get_name().to_owned();

    generate(generator, &mut app, name, &mut std::io::stdout());

    Ok(())
}

fn handle_advice(advice: FadviseType, info: AdviseInfo) -> anyhow::Result<()> {
    let filename = info.filename;
    let offset = info.offset;

    // Check file existence and metadata
    let exists = filename
        .try_exists()
        .context("Failed to check existence of the file")?;
    if !exists {
        return Err(anyhow!("'{}' does not exist", filename.display()));
    }
    let metadata = filename
        .metadata()
        .context("Failed to retrieve metadata of the file")?;
    if !metadata.is_file() {
        return Err(anyhow!("'{}' is not a file", filename.display()));
    }

    // Prepare arguments
    let len = info.len.unwrap_or(metadata.len() as i64);
    eprintln!("filename: {}", filename.display());
    eprintln!("advice: {}", advice);
    eprintln!("offset: {}", offset);
    eprintln!("len: {}", len);
    let file = File::open(filename).context("Failed to open the file")?;
    let fd = file.as_raw_fd();

    posix_fadvise(fd, offset, len, advice.into())?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Normal(info) => handle_advice(FadviseType::Normal, info),
        Commands::Sequential(info) => handle_advice(FadviseType::Sequential, info),
        Commands::Random(info) => handle_advice(FadviseType::Random, info),
        Commands::NoReuse(info) => handle_advice(FadviseType::NoReuse, info),
        Commands::WillNeed(info) => handle_advice(FadviseType::WillNeed, info),
        Commands::DontNeed(info) => handle_advice(FadviseType::DontNeed, info),
        Commands::Completion { shell } => print_completer(shell),
    }
}
