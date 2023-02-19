use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process;

/// This is a CLI tool for bulk renaming of files at once.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target directory path
    dir_path: String,
    /// Output file name()
    output_file_name: String,
    /// Dry run mode
    #[arg(long = "dry-run")]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();

    if args.dry_run {
        println!("=== Dry run mode ===");
    }

    let file_path = args.dir_path;
    if let Ok(entries) = fs::read_dir(file_path) {
        let mut index = 1;
        for entry in entries {
            let entry = entry.unwrap();
            let old_path = entry.path();
            let extension = old_path.extension();

            if let Some(ext) = extension {
                let ext_str = ext.to_string_lossy();

                let new_file_name = format!("{}_{}.{}", args.output_file_name, index, ext_str);
                let new_path = old_path.parent().unwrap().join(new_file_name);

                println!("Target file path: {:?} -> {:?}", old_path, new_path);

                if !args.dry_run {
                    rename(old_path, new_path).expect("Error: Failed to rename file");
                }

                index += 1;
            } else {
                println!(
                    "Error: Failed to read extension. Skip file name: {:?}",
                    old_path
                );
                continue;
            }
        }
    } else {
        eprint!("Error: Failed to read directory");
        process::exit(1);
    }

    println!("Finish!");
}

fn rename(old_path: PathBuf, new_path: PathBuf) -> std::io::Result<()> {
    match fs::rename(&old_path, &new_path) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}
