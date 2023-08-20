use std::io::{self, Write};
use image::{DynamicImage, GenericImageView, imageops::FilterType, Rgba};
use ansi_term::Colour;

const DENSITY: &'static str = " _.,-=+:;cba!?0123456789$W#@Ñ";

/// An ascii image renderer
pub struct Render {
  /// The image for render
  image: DynamicImage,
  /// The max size of the `image`
  max_size: u32,
  /// RGB render
  color: bool,
  /// Prevents a render distorsion caused by the ascii pixels aspect
  prevent_ascii_distorsion: bool
}

/// Put a color in the character
fn colorize(pixel: char, color: &Rgba<u8>) -> String {
  if pixel == ' ' {
    return pixel.to_string();
  }
  let [red, green, blue, _] = color.0;
  let style = Colour::RGB(red, green, blue);
  let colored = style.paint(pixel.to_string());

  colored.to_string()
}

/// Calculates the brightness of a pixel
/// and returns a float between 0 and 1
fn brightness(pixel: &Rgba<u8>) -> f32 {
  let [red, green, blue, alpha] = pixel.0;
  let total_color = red as u32 + green as u32 + blue as u32;
  let opacity = alpha as f32 / 255.0;

  total_color as f32 / 3.0 / 255.0 * opacity
}

/// Estimate the size of the render buffer
fn calculate_bytes(image: &DynamicImage, color: bool) -> usize {
  let dimensions = image.dimensions();
  let (size_x, size_y) = (dimensions.0 as usize, dimensions.1 as usize);
  let pixel_ascii_size = if color {25} else {2};
  
  size_x * size_y * pixel_ascii_size + size_y
}

impl Render {
  /// Create an instance of a render
  ///
  /// # Arguments
  ///
  /// * `image` - The image for render
  /// * `max_size` - The max size of the `image`
  /// * `color` - RGB render
  /// * `prevent_ascii_distorsion` - Prevents a render distorsion caused by the ascii pixels aspect
  pub fn new(image: DynamicImage, max_size: u32, color: bool, prevent_ascii_distorsion: bool) -> Self {
    Render { image, max_size, color, prevent_ascii_distorsion }
  }

  /// Calculates an ascii pixel with a `rgb_pixel
  fn ascii_pixel(&self, rgb_pixel: &Rgba<u8>) -> String {
    let brightness = brightness(rgb_pixel);

    let density_len = DENSITY.chars().count();
    let density_index = (brightness * density_len as f32 - 1.0).round() as usize;
    let pixel = DENSITY.chars().nth(density_index).unwrap();

    if self.color {
      colorize(pixel, rgb_pixel)
    } else {
      pixel.to_string()
    }
  }

  /// Adjust the scale of the image based on the
  /// `max_size` and the `prevent_ascii_distorsion` 
  fn adjust_scale(&self) -> DynamicImage {
    let adjusted = if self.prevent_ascii_distorsion {
      self.image.resize_exact(self.image.width() * 2, self.image.height(), FilterType::Triangle)
    } else {
      self.image.clone()
    };
    let (size_x, size_y) = adjusted.dimensions();
    
    if size_x <= self.max_size && size_y <= self.max_size {
      return adjusted;
    }
    adjusted.resize(self.max_size, self.max_size, FilterType::Triangle)
  }

  /// Paint the render in the stdout
  pub fn paint(&self) -> io::Result<()> {
    let scaled = self.adjust_scale();
    let buffer = scaled.to_rgba8();
    let bytes_estimation = calculate_bytes(&scaled, self.color);
    drop(scaled);

    let mut paint_buffer = String::new();
    paint_buffer.reserve(bytes_estimation);
    let mut last_line: u32 = 0;

    for (_, y, pixel) in buffer.enumerate_pixels() {
      if y != last_line {
        paint_buffer.push('\n');
        last_line = y;
      }
      paint_buffer.push_str(self.ascii_pixel(pixel).as_str());
    }

    paint_buffer.push('\n');
    io::stdout().write_all(paint_buffer.as_bytes())?;
    // DEBUG
    println!("\nEstimation {}, Real {}", bytes_estimation, paint_buffer.len());
    Ok(())
  }
}

