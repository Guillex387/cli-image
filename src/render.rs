use std::io::{self, Write};
use image::{DynamicImage, GenericImageView, imageops::FilterType, Rgba};
use ansi_term::Colour;

const DENSITY: &'static str = " _.,-=+:;cba!?0123456789$W#@Ñ";

fn ascii_pixel(rgb: &Rgba<u8>) -> char {
  let [red, green, blue, alpha] = rgb.0;
  let opacity = alpha as f32 / 255.0;
  let [redf, greenf, bluef] = [red as f32, green as f32, blue as f32];
  let brightness = (redf + greenf + bluef) / 3.0 / 255.0 * opacity;
  let density_len = DENSITY.chars().count();
  let density_index = (brightness * density_len as f32 - 1.0).round() as usize;
  DENSITY.chars().nth(density_index).unwrap()
}

fn colorize(pixel: char, color: &Rgba<u8>) -> String {
  let [red, green, blue, _] = color.0;
  let style = Colour::RGB(red, green, blue);
  let colored = style.paint(pixel.to_string());
  colored.to_string()
}

fn adjust_scale(image: &DynamicImage, max_size: u32) -> DynamicImage {
  // Prevents deformations caused by the dimension of the characters
  let deformed = image.resize_exact(image.width() * 2, image.height(), FilterType::Triangle);
  let (size_x, size_y) = deformed.dimensions();
  if size_x <= max_size && size_y <= max_size {
    return deformed;
  }
  deformed.resize(max_size, max_size, FilterType::Triangle)
}

fn calculate_bytes(image: &DynamicImage) -> usize {
  let dimensions = image.dimensions();
  let (size_x, size_y) = (dimensions.0 as usize, dimensions.1 as usize);
  size_x * size_y * 2 + size_y
}

pub fn render(image: &DynamicImage, max_size: u32, color: bool) -> io::Result<()> {
  let scaled = adjust_scale(image, max_size);
  let buffer = scaled.to_rgba8();
  let mut last_line: u32 = 0;
  let mut paint_buffer = String::new();
  if !color {
    paint_buffer.reserve(calculate_bytes(image));
  }
  for (_x, y, pixel) in buffer.enumerate_pixels() {
    if y != last_line {
      paint_buffer.push('\n');
      last_line = y;
    }
    let render_pixel = if color {
      colorize(ascii_pixel(pixel), pixel)
    } else {
      ascii_pixel(pixel).to_string()
    }; 
    paint_buffer.push_str(render_pixel.as_str());
  }
  paint_buffer.push('\n');
  io::stdout().write_all(paint_buffer.as_bytes())?;
  Ok(())
}

