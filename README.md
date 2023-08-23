# cli-image

A cli tool for preview images with ascii characters.
Support colors, adjustable resolution and alpha channel.

## Captures

![github_test](/assets/github_test.png)
![plant_test](/assets/plant_test.png)
![cup_test](/assets/cup_test.png)

## Supported Image Formats

| Format   | Decoding                                  |
| -------- | ----------------------------------------- |
| AVIF     | Only 8-bit \*\*                           |
| BMP      | Yes                                       |
| DDS      | DXT1, DXT3, DXT5                          |
| Farbfeld | Yes                                       |
| GIF      | Yes                                       |
| ICO      | Yes                                       |
| JPEG     | Baseline and progressive                  |
| OpenEXR  | Rgb32F, Rgba32F (no dwa compression)      |
| PNG      | All supported color types                 |
| PNM      | PBM, PGM, PPM, standard PAM               |
| QOI      | Yes                                       |
| TGA      | Yes                                       |
| TIFF     | Baseline(no fax support) + LZW + PackBits |
| WebP     | Yes                                       |

## Development

The tool is completely written in [Rust](https://www.rust-lang.org/) and
I use the [image](https://github.com/image-rs/image) library to decode the image formats.

## Use

```
$ cli-image --help

A cli tool for preview images with ascii characters

Usage: cli-image [OPTIONS] <FILE>

Arguments:
  <FILE>  The file for preview

Options:
  -m, --max-size <MAX_SIZE>    The max size of the preview in the screen (nº of ascii pixels) [default: 100]
  -c, --color                  Preview image with color (default = false) Note: this feature is only to true color terminals
  -o, --omit-ascii-distortion  Omits the ascii distortion filter (default = false)
  -h, --help                   Print help
  -V, --version                Print version
```

## License

cli-image Copyright (c) 2023 Guillex387. All rights reserved.

Licensed under the [Apache-2.0](/LICENSE) license.
