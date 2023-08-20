use std::process::ExitCode;
use clap::Parser;
mod render;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
  /// The file for preview
  file: String,
  
  /// The max size of the preview in the screen (nº of ascii pixels)
  #[arg(short, long, default_value_t = 300)]
  max_size: u32,

  /// Preview image with color (default = false)
  /// Note: this feature is only to true color terminals
  #[arg(short, long, default_value_t = false)]
  color: bool
}

fn main() -> ExitCode {
  let cli: Cli = Cli::parse();
  let img = match image::open(&cli.file) {
    Ok(res) => res,
    Err(_) => {
      println!("Error reading {} file", cli.file);
      return ExitCode::FAILURE;
    }
  };
  match render::render(&img, cli.max_size, cli.color) {
    Ok(_) => {},
    Err(_) => {
      println!("Error rendering the image");
      return ExitCode::FAILURE;
    }
  };
  ExitCode::SUCCESS
}
