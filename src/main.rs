mod render;
use std::process::ExitCode;
use clap::Parser;
use render::Render;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
  /// The file for preview
  file: String,
  
  /// The max size of the preview in the screen (nº of ascii pixels)
  #[arg(short, long, default_value_t = 100)]
  max_size: u32,

  /// Preview image with color (default = false)
  /// Note: this feature is only to true color terminals
  #[arg(short, long, default_value_t = false)]
  color: bool,

  /// Omits the ascii distorsion filter (default = false)
  #[arg(short, long, default_value_t = false)]
  omit_ascii_distorsion: bool
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
  let render: Render = Render::new(img, cli.max_size, cli.color, !cli.omit_ascii_distorsion);
  match render.paint() {
    Ok(_) => {},
    Err(_) => {
      println!("Error rendering the image");
      return ExitCode::FAILURE;
    }
  };
  ExitCode::SUCCESS
}
