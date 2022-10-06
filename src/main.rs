use clap::Parser;
use image::{Rgb, RgbImage};
use num::complex::Complex64;
use std::path::PathBuf;

/// a simple program to create mandlebrot set images
#[derive(Parser)]
struct Args {
    /// File name
    file_name: PathBuf,

    /// width of image
    #[arg(short, long, default_value_t = 512)]
    width: u32,

    /// width of image
    #[arg(short, long, default_value_t = 512)]
    height: u32,
}

fn main() {
    let args = Args::parse();

    let mut image = RgbImage::new(args.width, args.height);

    progress_bar::init_progress_bar(args.width as usize + 1);
    progress_bar::set_progress_bar_action(
        "Calculating",
        progress_bar::Color::Blue,
        progress_bar::Style::Bold,
    );

    for i in 0..args.width {
        for j in 0..args.height {
            let brightness = escape_time(pixel_to_point((i, j), &args), 50);

            image.put_pixel(i, j, Rgb([brightness * 4, brightness, brightness]));
        }
        progress_bar::inc_progress_bar();
    }

    progress_bar::print_progress_bar_info(
        "Success",
        "All pixels have been calculated",
        progress_bar::Color::Green,
        progress_bar::Style::Bold,
    );

    progress_bar::set_progress_bar_action(
        "Saving",
        progress_bar::Color::Blue,
        progress_bar::Style::Bold,
    );

    let mut filepath = PathBuf::new();
    filepath.push("output");
    filepath.push(args.file_name);

    image.save(&filepath).unwrap();

    progress_bar::inc_progress_bar();

    progress_bar::print_progress_bar_info(
        "Saved",
        &format!("Saved output file to {}", filepath.to_string_lossy()),
        progress_bar::Color::Green,
        progress_bar::Style::Bold,
    );
    progress_bar::finalize_progress_bar();
}

fn escape_time(c: Complex64, limit: u8) -> u8 {
    let mut z = Complex64::new(0.0, 0.0);

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return i;
        }

        z = z * z + c;
    }

    0
}

fn pixel_to_point(pixel: (u32, u32), args: &Args) -> Complex64 {
    Complex64::new(
        (pixel.0 as f64 / args.width as f64) * 2.0 - 1.5,
        (pixel.1 as f64 / args.height as f64) * 2.0 - 1.0,
    )
}
