use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: bulk_rename_files <dir_path> <output_file_name>");
        return;
    }

    let file_path = &args[1];
    for (i, entry) in fs::read_dir(file_path)
        .expect("Error: Failed to read directory")
        .enumerate()
    {
        let entry = entry.unwrap();
        let old_path = entry.path();
        let extension = old_path.extension();

        if let Some(ext) = extension {
            let ext_str = ext.to_string_lossy();

            println!("Target file path: {:?}", old_path);
            let new_file_name = format!("{}_{}.{}", &args[2], i + 1, ext_str);
            let new_path = old_path.parent().unwrap().join(new_file_name);
            rename(old_path, new_path).expect("Error: Failed to rename file");
        } else {
            println!("Error: Failed to read extension");
            continue;
        }
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
