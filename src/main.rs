use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tera::Context;
use tera::Tera;

#[derive(Deserialize)]
struct Config {
    title: String,
    sub_title: String,
}

fn read_config(config_file_path: &Path) -> io::Result<Config> {
    // read the config
    let mut config_file = File::open(config_file_path)?;
    let mut config = String::new();
    config_file.read_to_string(&mut config)?;

    let config: Config = match toml::from_str(&config) {
        Ok(c) => Ok(c),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }?;

    Ok(config)
}

fn render_template(
    output_file_path: &Path,
    template_name: &str,
    context: &Context,
    tera: &Tera,
) -> io::Result<()> {
    // Render the page
    let result = match tera.render(template_name, &context) {
        Ok(t) => Ok(t),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }?;

    // Write the output file
    let mut output_file = File::create(output_file_path)?;
    output_file.write(result.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            String::from("usage: blog-generator config_file output_dir"),
        ));
    }

    let config_file_path = PathBuf::from(&args[1]);
    let output_path = PathBuf::from(&args[2]);

    // Load the config
    let config = read_config(&config_file_path)?;

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

    // Load the templates
    let tera = match Tera::new("src/templates/**/*.html") {
        Ok(t) => Ok(t),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }?;

    // Create the index page
    let index_page_path = output_path.join("index.html");
    let mut context = Context::new();
    context.insert("blog_title", &config.title);
    context.insert("blog_sub_title", &config.sub_title);
    render_template(&index_page_path, "index.html", &context, &tera)?;

    Ok(())
}
