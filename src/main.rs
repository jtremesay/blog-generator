use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(String::from("usage: blog-generator config_file output_dir"));
    }

    let config_file_path = &args[1];
    let output_path = &args[2];

    Ok(())
}
