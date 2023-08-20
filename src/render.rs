use std::io::{self, Write};
use image::{DynamicImage, GenericImageView, imageops::FilterType, Rgba};
use ansi_term::Colour;

const DENSITY: &'static str = " _.,-=+:;cba!?0123456789$W#@Ñ";

fn density(rgb: &Rgba<u8>) -> char {
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
  let (size_x, size_y) = image.dimensions();
  if size_x <= max_size && size_y <= max_size {
    return image.clone();
  }
  let horizontal = size_x >= size_y;
  let aspect = size_x as f32 / size_y as f32;
  let new_x: u32 =
    if horizontal {
      max_size
    } else {
      (max_size as f32 * aspect).round() as u32
    };
  let new_y: u32 = 
    if horizontal {
      (max_size as f32 * aspect).round() as u32
    } else {
      max_size
    };
  image.resize(new_x, new_y, FilterType::Triangle)
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
      colorize(density(pixel), pixel)
    } else {
      density(pixel).to_string()
    }; 
    paint_buffer.push_str(render_pixel.as_str());
  }
  paint_buffer.push('\n');
  io::stdout().write_all(paint_buffer.as_bytes())?;
  Ok(())
}

