use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgb};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};


///TODO REFACTOR WHOLE FILE


pub fn smallest_divider(n: u32) -> u32 {
    (2..n).find(|&i| n % i == 0).unwrap_or(n)
}

pub fn euclidean_distance(p1: (u8, u8, u8), p2: (u8, u8, u8)) -> f64 {
    let (r1, g1, b1) = (p1.0 as f64, p1.1 as f64, p1.2 as f64);
    let (r2, g2, b2) = (p2.0 as f64, p2.1 as f64, p2.2 as f64);
    ((r1 - r2).powi(2) + (g1 - g2).powi(2) + (b1 - b2).powi(2)).sqrt()
}

pub fn closest_colour(pixel: (u8, u8, u8), colours: &[(u8, u8, u8)]) -> Rgb<u8> {
    colours
        .iter()
        .map(|&colour| (euclidean_distance(pixel, colour), colour))
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .map(|(_, colour)| Rgb([colour.0, colour.1, colour.2]))
        .unwrap_or(Rgb([0, 0, 0]))
}

fn create_cube(x: u32, y: u32, chunk: &DynamicImage) -> Vec<Rgb<u8>> {
    let mut cube = Vec::with_capacity(9);
    let width = chunk.width();
    let height = chunk.height();
    let start_x = x.saturating_sub(1);
    let start_y = y.saturating_sub(1);
    let end_x = (x + 1).min(width - 1);
    let end_y = (y + 1).min(height - 1);

    for yy in start_y..=end_y {
        for xx in start_x..=end_x {
            cube.push(chunk.get_pixel(xx, yy).to_rgb());
        }
    }

    cube
}

fn cluster_colour(
    x: u32,
    y: u32,
    chunk: &DynamicImage,
    indesirables: &HashSet<Rgb<u8>>,
) -> Rgb<u8> {
    let pixel = chunk.get_pixel(x, y).to_rgb();

    if !indesirables.contains(&pixel) {
        return pixel;
    } else {
        let cube = create_cube(x, y, chunk);
        let mut count: HashMap<Rgb<u8>, u32> = HashMap::with_capacity(9);

        for pixel in cube {
            *count.entry(pixel).or_insert(0) += 1;
        }

        count
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(&color, _)| {
                if indesirables.contains(&color) {
                    Rgb([0, 0, 0])
                } else {
                    color
                }
            })
            .unwrap_or(Rgb([0, 0, 0]))
    }
}

fn proccessing_pixel(pixel: &Rgb<u8>, colours: &[(u8, u8, u8)]) -> Rgb<u8> {
    closest_colour((pixel[0], pixel[1], pixel[2]), colours)
}

fn proccessing_chunk(chunk: &DynamicImage, colours: &[(u8, u8, u8)]) -> DynamicImage {
    let (width, height) = chunk.dimensions();

    let pixel_data: Vec<_> = chunk
        .pixels()
        .par_bridge()
        .map(|(x, y, pixel)| {
            let rgb = pixel.to_rgb();
            let new_pixel = proccessing_pixel(&rgb, colours);
            (x, y, new_pixel)
        })
        .collect();

    let mut new_chunk = DynamicImage::new_rgb8(width, height);

    for (x, y, pixel) in pixel_data {
        new_chunk.put_pixel(x, y, pixel.to_rgba());
    }

    new_chunk
}

fn post_process_chunk(chunk: &DynamicImage, indesirables: &HashSet<Rgb<u8>>) -> DynamicImage {
    let (width, height) = chunk.dimensions();
    let mut processed_chunk = DynamicImage::new_rgb8(width, height);

    for y in 0..height {
        for x in 0..width {
            let new_pixel = cluster_colour(x, y, chunk, &indesirables);
            processed_chunk.put_pixel(x, y, new_pixel.to_rgba());
        }
    }

    processed_chunk
}

pub fn main() {
    let veg_folder = "content/veg";
    let tmp_folder = "tmp";
    let colours = vec![
        (0, 0, 0),
        (4, 25, 30),
        (25, 50, 60),
        (50, 200, 80),
        (80, 200, 120),
        (255, 255, 255),
        (128, 128, 128),
        (14, 14, 14), // export parasite
    ];
    let indesirables: HashSet<_> = vec![
        Rgb([255, 255, 255]),
        Rgb([128, 128, 128]),
        Rgb([14, 14, 14]),
    ]
    .into_iter()
    .collect();

    for entry in std::fs::read_dir(veg_folder).unwrap() {
        let img_path = entry.unwrap().path();
        let img_name = img_path.file_name().unwrap().to_str().unwrap();
        let mut reader = image::ImageReader::open(&img_path).unwrap();
        reader.no_limits();
        let img = reader.with_guessed_format().unwrap().decode().unwrap();
        
        let (width, height) = img.dimensions();

        let chunk_width = width / smallest_divider(width);
        let chunk_height = height / smallest_divider(height);

        let chunks: Vec<DynamicImage> = (0..height / chunk_height)
            .flat_map(|y| (0..width / chunk_width).map(move |x| (x, y)))
            .collect::<Vec<_>>()
            .par_iter()
            .map(|&(x, y)| {
                let chunk =
                    img.crop_imm(x * chunk_width, y * chunk_height, chunk_width, chunk_height);
                let processed_chunk = proccessing_chunk(&chunk, &colours);
                post_process_chunk(&processed_chunk, &indesirables)
            })
            .collect();

        let mut new_img = DynamicImage::new_rgb8(width, height);
        let (mut x, mut y) = (0, 0);

        for chunk in chunks {
            new_img.copy_from(&chunk, x, y).unwrap();
            x += chunk.width();
            if x >= width {
                x = 0;
                y += chunk.height();
            }
        }

        new_img
            .save(format!("{}/{}", tmp_folder, img_name))
            .unwrap();
        println!("{} traité", img_name);
    }
}