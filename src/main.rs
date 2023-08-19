use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
  /// The file for preview
  file: String,
  
  /// The max size of the preview in the screen (nº of ascii pixels)
  #[arg(short, long)]
  max_size: Option<u32>
}

fn main() {
  let cli: Cli = Cli::parse();
  // Test for the argument parser
  println!("Filename: {}", cli.file);
  match cli.max_size {
    Some(s) => println!("Provided max-size: {}", s),
    None => {}
  }
}
