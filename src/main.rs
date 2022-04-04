use clap::Parser;

use image::*;

///Simple program to extract color data from an image
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Image Path
    #[clap(short, long)]
    path: String,

    ///Grid size
    #[clap(short, long)]
    density: u32,
}

fn validateImagePath(image_path: &String) -> Result<bool, String> {
    println!("{:?}", image_path);
    Ok((true))
}

fn extractData(image_path: &String, density: &u32) -> Vec<Rgb<u8>> {
    let image = image::open(image_path).unwrap();
    let dimensions = image.dimensions();

    let mut colorsVector: Vec<Rgb<u8>> = Vec::new();

    for i in 0..(dimensions.0 / density) {
        for j in 0..(dimensions.1 / density) {
            let pixel = image.get_pixel(i, j).to_rgb();
            colorsVector.push(pixel);
        }
    }
    colorsVector
}

fn avgColor(image_path: &String, density: &u32) -> Vec<u64> {
    let image = image::open(image_path).unwrap();
    let dimensions = image.dimensions();
    let mut col: [u64; 3] = [0; 3];
    let pixelCounter: u64 = (dimensions.0 * dimensions.1) as u64 / density.pow(2) as u64;

    for i in 0..(dimensions.0 / density) {
        for j in 0..(dimensions.1 / density) {
            let pixel = image.get_pixel(i, j).to_rgb();
            col[0] += pixel[0] as u64;
            col[1] += pixel[1] as u64;
            col[2] += pixel[2] as u64;
        }
    }
    let colors: Vec<u64> = col
        .into_iter()
        .map(|color: u64| color / pixelCounter)
        .collect();
    colors
}

fn main() {
    let args = Args::parse();

    // let colorsVec: Vec<Rgb<u8>> = extractData(&args.path, &args.density);
    let avg_col: Vec<u64> = avgColor(&args.path, &args.density);
    println!("Average Color {:?}", avg_col);
    // for color in colorsVec.iter() {
    //     println!("{:?}", color);
    // }
}
