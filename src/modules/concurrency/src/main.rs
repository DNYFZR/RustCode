// Concurrency Example : Plotting the Mandelbrot set

// A fractal produced by iterating a simple function on complex numbers
// Using the real & imaginary components as x, y co-ords on a Cartesian plance
// Color using classification -> value in set = black / not in set = light color

use std::env;
use std::str::FromStr;
use std::fs::File;
use num::Complex;
use image::ColorType;
use image::png::PNGEncoder;

fn escape_time (c: Complex<f64>, limit: usize) -> Option<usize> { // Option enum type None || Some(T) 
    // Initiate complex variable
    let mut z = Complex{re: 0.0, im:0.0};
    
    // Loop through range 0..limit
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {return Some(i);}
        z = z * z + c;
    }

    None
}

fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> { 
    // Parse a sting into a pair of values using sep

    // <T: FromStr> - for any type T which implements the FromStr trait  
    // T is a type param of parse_pair

    // Find sep locations
    match s.find(sep) {
        None => None, Some(index) => {
            // match string slice
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) { 
                (Ok(l), Ok(r)) => Some((l, r)), _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    // No input -> None
    assert_eq!(parse_pair::<i32>("", ','), None);

    // Missing input -> None
    assert_eq!(parse_pair::<i32>("10, ", ','), None);
    assert_eq!(parse_pair::<i32>(", 10", ','), None);

    // Valid input -> Some
    assert_eq!(parse_pair::<i32>("10, 20", ','), Some((10, 20)));
}

fn parse_complex (s: &str) -> Option<Complex<f64>> {
    // Parse a pair of floating-point numbers separated by a comma as a complex
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}), None => None
    }
}

#[test]
fn test_parse_complex() {
    // Valid input
    assert_eq!(parse_complex("1.25, -0.0625"), Some(Complex {re: 1.25, im: -0.0625}));

    // Invalid input
    assert_eq!(parse_complex(", -0.0625"), None);
}

fn pixel_to_point (
    bounds: (usize, usize), 
    pixel: (usize, usize), 
    upper_left: Complex<f64>, 
    lower_right: Complex<f64>) -> Complex<f64> {
    // Given the row and column of a pixel in the output image - return the corresponding point on the complex plane

    // bounds : a pair giving the width and height of the image in pixels
    // pixel : a (column, row) pair indicating a particular pixel in that image
    // upper_left & lower_right : points on the complex plane designating the area our image covers
    
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64, 
        im: upper_left.im + pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point((100, 200), (25, 175), Complex{re:-1.0, im: 1.0}, Complex{re:1.0, im:-1.0}),
        Complex{re:-0.5, im:-0.75}
    );
}

fn render (pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    // Render a rectangle of the Mandelbrot set into a buffer of pixels
    // bounds : the width and height of the buffer pixels (one grayscale pixel per byte) 
    // upper_left & lower_right : specify corner points on the complex plane corresponding

    // Check pixels are available
    assert!(pixels.len() == bounds.0 * bounds.1);

    // Generate buffer
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(point, 255) {None => 0, Some(count) => 255 - count as u8};
        }
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    // Write the buffer (pixels) to file
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);

    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(()) 
}


// Builds

fn non_concurrent_plot(){
    // Collect user input args 
    let args: Vec<String> = env::args().collect();
    
    // Check for valid user input
    if args.len() != 5 {
        eprintln!("Usage: {} FILENAME PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    // Assign input
    let bounds = parse_pair(&args[2], 'x').expect("Error parsing image dims");
    let upper_left = parse_complex(&args[3]).expect("Error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("Error parsing lower right corner point");

    // Init pixel vector container
    let mut pixels = vec![0; bounds.0 * bounds.1];

    // Build image
    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds).expect("Error writing PNG file");
}

fn concurrent_plot() {
    // Collect user input args 
    let args: Vec<String> = env::args().collect();
    
    // Check for valid user input
    if args.len() != 5 {
        eprintln!("Usage: {} FILENAME PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    // Assign input
    let bounds = parse_pair(&args[2], 'x').expect("Error parsing image dims");
    let upper_left = parse_complex(&args[3]).expect("Error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("Error parsing lower right corner point");

    // Init pixel vector container
    let mut pixels = vec![0; bounds.0 * bounds.1];

    // Build image
    let threads = 8;
    let rows_per = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        }).unwrap();
    }

    // Write image
    write_image(&args[1], &pixels, bounds).expect("Error writing PNG file");
}


// App

pub fn main() {
    concurrent_plot()
}
