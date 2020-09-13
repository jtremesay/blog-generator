use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            String::from("usage: blog-generator config_file output_dir"),
        ));
    }

    let _config_file_path = PathBuf::from(&args[1]);
    let output_path = PathBuf::from(&args[2]);

    // Create and clean the output dir
    fs::create_dir_all(&output_path)?;
    for entry in fs::read_dir(&output_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}
