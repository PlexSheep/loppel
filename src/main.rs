use clap::{Parser, Subcommand};
use std::ffi::{OsStr, OsString};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};
use zstd::DEFAULT_COMPRESSION_LEVEL;

const ARCHIVE_ENDINGS: &[&str] = &[".tar.zstd", ".tar.zst"];
const HELP_TEMPLATE: &str = r"{about-section}
{usage-heading} {usage}

{all-args}{tab}

{name}: {version}
Author: {author-with-newline}
";

#[derive(Parser)]
#[command(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = "Simple local backups with a bit of compression",
    help_template = HELP_TEMPLATE
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Do not confirm
    #[clap(short = 'y', long = "yes", global = true)]
    confirm: bool,

    /// Print out every action
    #[clap(short = 'v', long = "verbose", global = true)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create backup of files or directories, default action
    #[clap(visible_alias = "b")]
    #[clap(visible_alias = "bak")]
    Backup {
        /// Files or directories to backup
        path: PathBuf,

        /// Use zstd compression
        #[arg(short = 'z', long)]
        compress: bool,

        /// Delete original after successful backup
        #[arg(short = 'd', long)]
        delete: bool,
    },

    /// Restore from backup
    #[clap(visible_alias = "r")]
    #[clap(visible_alias = "res")]
    Restore {
        /// Backup file to restore from
        path: PathBuf,

        /// Delete backup after successful restore
        #[arg(short = 'd', long)]
        delete: bool,

        /// Directory to restore to
        #[arg(short = 'o', long = "output")]
        output_dir: Option<PathBuf>,
    },
}

fn help_and_exit() -> ! {
    use clap::CommandFactory;
    let mut cmd = Cli::command();
    cmd.print_help().expect("could not print");
    std::process::exit(1)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli;
    let command = {
        let mut a: Vec<String> = std::env::args().collect();
        if a.len() < 2 {
            help_and_exit()
        }
        if !(a[1].starts_with("-")
            || a[1] == "r"
            || a[1] == "res"
            || a[1] == "restore"
            || a[1] == "b"
            || a[1] == "bak"
            || a[1] == "backup")
        {
            let slice = if a[1].contains("bak") {
                &["restore".to_string()]
            } else {
                &["backup".to_string()]
            };

            a.splice(1..1, slice.iter().cloned());
        }
        cli = Cli::parse_from(a.iter());
        cli.command.unwrap()
    };

    let created;
    match command {
        Commands::Backup {
            path,
            compress,
            delete,
        } => {
            if !path.exists() {
                eprintln!("Error: {path:?} does not exist");
                help_and_exit()
            }

            if path.is_dir() {
                created = backup_dir(&path, compress)?;
            } else if path.is_file() {
                created = backup_file(&path, compress)?;
            } else {
                panic!("this is neither a file nor a directory, don't know what to do")
            }

            if delete && (cli.confirm || confirm(format!("delete {}?", path.display()))?) {
                recursive_remove(&path)?;
            }
        }
        Commands::Restore {
            path,
            delete,
            output_dir,
        } => {
            let out = output_dir.unwrap_or(std::env::current_dir()?);
            created = restore(&path, &out)?;
            if delete && (cli.confirm || confirm(format!("delete {}?", path.display()))?) {
                recursive_remove(&path)?;
            }
        }
    }

    if cli.verbose {
        println!("{}", created.display())
    }

    Ok(())
}

fn confirm(prompt: String) -> io::Result<bool> {
    print!("{prompt} - y/N ");
    io::stdout().flush()?;
    let mut buf = String::new();
    loop {
        io::stdin().read_line(&mut buf)?;
        buf = buf.trim().to_lowercase();
        match buf.as_str() {
            "y" | "yes" => return Ok(true),
            "" | "n" | "no" => return Ok(false),
            _ => println!("That is neither yes or no"),
        }
    }
}

fn recursive_remove(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else if path.is_file() || path.is_symlink() {
        fs::remove_file(path)?;
    } else {
        eprintln!("skipping unknown file: {}", path.display());
    }
    Ok(())
}

fn add_extension(path: &Path, postfix: &str) -> PathBuf {
    let parts = [
        path.file_name()
            .expect("this string is weird, no file name"),
        OsStr::new(postfix),
    ];
    let newname: OsString = parts.iter().copied().collect();
    path.with_file_name(newname)
}

fn remove_extension(path: &Path, suffix: &str) -> PathBuf {
    let r = path.display().to_string();
    match r.strip_suffix(&format!(".{suffix}")) {
        None => panic!("that path did not have that suffix"),
        Some(short) => PathBuf::from(short),
    }
}

fn restore(path: &Path, output_dir: &Path) -> io::Result<PathBuf> {
    if !path.exists() {
        let e = io::Error::new(
            io::ErrorKind::NotFound,
            format!("File or directory not found: {}", path.display()),
        );
        eprintln!("{e}");
        return Err(e);
    }
    if !output_dir.exists() {
        let e = io::Error::new(
            io::ErrorKind::NotFound,
            format!("File or directory not found: {}", output_dir.display()),
        );
        eprintln!("{e}");
        return Err(e);
    }
    if !output_dir.is_dir() {
        let e = io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Output directory is not a directory: {}",
                output_dir.display()
            ),
        );
        eprintln!("{e}");
        return Err(e);
    }

    let path_s: String = path.display().to_string();
    if path_s.ends_with("tar.zstd") || path_s.ends_with("tar.zst") {
        if !path.is_file() {
            panic!("archive name but not an archive")
        }

        read_archive(path, |a| a.unpack(output_dir))?;

        let mut target = output_dir.to_path_buf();
        target.push(remove_archive_ending(path));
        Ok(target)
    } else if path_s.ends_with("bak") {
        if !path.is_file() {
            panic!("bak name but not a file")
        }

        let target = remove_extension(path, "bak");
        let target = output_dir.join(target.file_name().unwrap());
        fs::copy(path, &target)?;
        Ok(target)
    } else if path_s.ends_with("bak.d") {
        if path.is_file() {
            panic!("bak.d name but not a directory")
        }
        let target = remove_extension(path, "bak.d");
        let target = output_dir.join(target.file_name().unwrap());
        copy_dir_all(path, &target)?;
        Ok(target)
    } else {
        panic!("unknown file {path_s}")
    }
}

fn backup_file(path: &Path, compress: bool) -> io::Result<PathBuf> {
    if compress {
        let archive_path = add_extension(path, ".tar.zstd");
        make_archive(&archive_path, |a| a.append_path(path))?;
        Ok(archive_path)
    } else {
        let backup_path = add_extension(path, ".bak");
        fs::copy(path, &backup_path)?;
        Ok(backup_path)
    }
}

fn backup_dir(path: &Path, compress: bool) -> io::Result<PathBuf> {
    if compress {
        let archive_path = add_extension(path, ".tar.zstd");
        make_archive(&archive_path, |a| a.append_dir_all(path, path))?;
        Ok(archive_path)
    } else {
        let backup_path = add_extension(path, ".bak.d");
        copy_dir_all(path, &backup_path)?;
        Ok(backup_path)
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst_path)?;
        } else if ty.is_file() {
            fs::copy(entry.path(), dst_path)?;
        } else {
            eprintln!(
                "neither a file nor a directory, skipping: {}",
                entry.path().display()
            );
        }
    }
    Ok(())
}

fn make_archive<F>(archive_path: &Path, do_this: F) -> std::io::Result<()>
where
    F: FnOnce(
        &mut tar::Builder<zstd::stream::AutoFinishEncoder<std::fs::File>>,
    ) -> std::io::Result<()>,
{
    let compressed_file = fs::File::create(archive_path)?;

    let compressor = zstd::Encoder::new(compressed_file, DEFAULT_COMPRESSION_LEVEL)?.auto_finish();
    let mut archiver = tar::Builder::new(compressor);

    do_this(&mut archiver)?;

    archiver.finish()?;

    Ok(())
}

fn read_archive<F>(archive_path: &Path, do_this: F) -> std::io::Result<()>
where
    F: FnOnce(
        &mut tar::Archive<zstd::Decoder<'_, std::io::BufReader<std::fs::File>>>,
    ) -> std::io::Result<()>,
{
    let compressed_file = match fs::File::open(archive_path) {
        Err(e) => {
            eprintln!("could not open archive: {e}");
            return Err(e);
        }
        Ok(f) => f,
    };

    let decompressor = match zstd::Decoder::new(compressed_file) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("could not open zstd decoder: {e}");
            return Err(e);
        }
    };
    let mut unarchiver = tar::Archive::new(decompressor);

    match do_this(&mut unarchiver) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("could perform read_archive actions: {e}");
            return Err(e);
        }
    };

    Ok(())
}

pub(crate) fn remove_archive_ending(p: impl Into<PathBuf>) -> PathBuf {
    let p: PathBuf = p.into();
    let mut ps: String = p
        .to_str()
        .expect("could not make path to string")
        .to_string();

    // NOTE: If a file has the archive endings in that specific order concatenated, this will
    // remove them all. I don't care.
    for ext_to_be_removed in ARCHIVE_ENDINGS {
        if ps.ends_with(*ext_to_be_removed) {
            ps.truncate(ps.len() - ext_to_be_removed.len());
        }
    }
    ps.into()
}

#[cfg(test)]
mod tests; // unit tests
