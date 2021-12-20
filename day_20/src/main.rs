use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::Chars;


fn main() {
    // let reader = BufReader::new(File::open("src/input.txt").unwrap());
    // let mut lines = reader.lines();
    // let algo_line = lines.next().unwrap().unwrap();
    // let algo: Vec<char> = algo_line.chars().collect();
    // // empty line
    // lines.next();
    
    // let mut original_image: Vec<Vec<char>> = Vec::new();
    // for (_, line) in lines.enumerate() {
    //     let line = line.unwrap();
    //     original_image.push(line.chars().collect());
    // }

    // let padded_image = pad_image(&original_image, '.');
    // let enhanced_image = enhance(&padded_image, &algo, '.');
    // println!("lit pixel count {}", count_lit_pixels(&enhanced_image));
    // let padded_image = pad_image(&enhanced_image, '#');
    // let enhanced_image = enhance(&padded_image, &algo, '#');
    // println!("lit pixel count {}", count_lit_pixels(&enhanced_image));
    run_50_p2();
}

fn run_50_p2() {
    let reader = BufReader::new(File::open("src/input.txt").unwrap());
    let mut lines = reader.lines();
    let algo_line = lines.next().unwrap().unwrap();
    let algo: Vec<char> = algo_line.chars().collect();
    // empty line
    lines.next();
    
    let mut original_image: Vec<Vec<char>> = Vec::new();
    for (_, line) in lines.enumerate() {
        let line = line.unwrap();
        original_image.push(line.chars().collect());
    }

    let mut default_pixel = '.';
    let padded_image = pad_image(&original_image, default_pixel);
    let mut enhanced_image = enhance(&padded_image, &algo, default_pixel);

    for i in 0..49 {
        if default_pixel == '.' {
            default_pixel = '#';
        } else {
            default_pixel = '.';
        }
        let padded_image = pad_image(&enhanced_image, default_pixel);
        enhanced_image = enhance(&padded_image, &algo, default_pixel);
    }
    println!("lit pixel count {}", count_lit_pixels(&enhanced_image));   
}

fn run_50_test() {
    let reader = BufReader::new(File::open("src/test.txt").unwrap());
    let mut lines = reader.lines();
    let algo_line = lines.next().unwrap().unwrap();
    let algo: Vec<char> = algo_line.chars().collect();
    // empty line
    lines.next();
    
    let mut original_image: Vec<Vec<char>> = Vec::new();
    for (_, line) in lines.enumerate() {
        let line = line.unwrap();
        original_image.push(line.chars().collect());
    }

    let padded_image = pad_image(&original_image, '.');
    let mut enhanced_image = enhance(&padded_image, &algo, '.');

    for i in 0..49 {
        let padded_image = pad_image(&enhanced_image, '.');
        enhanced_image = enhance(&padded_image, &algo, '.');
    }
    println!("lit pixel count {}", count_lit_pixels(&enhanced_image));     
}

fn count_lit_pixels(image: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for row in image {
        for pixel in row {
            if pixel == &'#' {
                count += 1;
            }
        }
    }
    count
}

fn pad_image(image: &Vec<Vec<char>>, pixel: char) -> Vec<Vec<char>> {
    let mut new_image: Vec<Vec<char>> = Vec::new();
    new_image.push(vec![pixel; image[0].len() + 2]);
    let mut new_row: Vec<char> = Vec::new();
    for row in image {
        new_row.push(pixel);
        for pixel in row {
            new_row.push(*pixel);
        }
        new_row.push(pixel);
        new_image.push(new_row);
        new_row = Vec::new();
    }
    new_image.push(vec![pixel; image[0].len() + 2]);
    return new_image;
}

fn enhance(original_image: &Vec<Vec<char>>, algo: &Vec<char>, default_pixel: char) -> Vec<Vec<char>> {
    let mut enhanced_image: Vec<Vec<char>> = Vec::new();
    for i in 0..original_image.len() {
        let mut row: Vec<char> = Vec::new();
        for j in 0..original_image[i].len() {
            let bits = get_surrounding_pixels_as_bits(&original_image, i, j, default_pixel);
            let idx = u16::from_str_radix(&bits, 2).unwrap();
            let output_pixel = algo[idx as usize];
            row.push(output_pixel);
        }
        enhanced_image.push(row);
    }
    return enhanced_image;
}

fn get_surrounding_pixels_as_bits(original_image: &Vec<Vec<char>>, i: usize, j: usize, default_pixel: char) -> String {
    let mut pixels: String = String::new();
    // get top left
    if i > 0 && j > 0 {
        pixels.push(original_image[i-1][j-1]);
    } else {
        pixels.push(default_pixel);
    }

    // top
    if i > 0 {
        pixels.push(original_image[i-1][j]);
    } else {
        pixels.push(default_pixel);
    }

    // top right
    if i > 0 && j < original_image[i].len() - 1 {
        pixels.push(original_image[i-1][j+1]);
    } else {
        pixels.push(default_pixel);
    }

    // left
    if j > 0 {
        pixels.push(original_image[i][j-1]);
    } else {
        pixels.push(default_pixel);
    }
    // self
    pixels.push(original_image[i][j]);
    // right
    if j < original_image[i].len() - 1 {
        pixels.push(original_image[i][j+1]);
    } else {
        pixels.push(default_pixel);
    }
    // bottom left
    if i < original_image.len() - 1 && j > 0 {
        pixels.push(original_image[i+1][j-1]);
    } else {
        pixels.push(default_pixel);
    }
    // bottom
    if i < original_image.len() - 1 {
        pixels.push(original_image[i+1][j]);
    } else {
        pixels.push(default_pixel);
    }
    // bottom right
    if i < original_image.len() - 1 && j < original_image[i].len() - 1 {
        pixels.push(original_image[i+1][j+1]);
    } else {
        pixels.push(default_pixel);
    }
    return convert_pixel_to_bits(pixels);
}

fn convert_pixel_to_bits(pixels: String) -> String {
    let mut bits: String = String::new();
    for pixel in pixels.chars() {
        if pixel == '#' {
            bits.push('1');
        } else {
            bits.push('0');
        }
    }
    return bits;
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_get_surrounding_pixels_as_bits() {
        let original_image = vec![vec!['#', '.', '.', '#', '.'], vec!['#', '.', '.', '.', '.'], vec!['#', '#', '.', '.', '#'], vec!['.', '.', '#', '.', '.'], vec!['.', '.', '#', '#', '#']];
        let padded_image = pad_image(&original_image, '.');
        let surrounding_pixels = get_surrounding_pixels_as_bits(&padded_image, 1, 1, '0');
        assert_eq!(surrounding_pixels, "000010010");

        let surrounding_pixels = get_surrounding_pixels_as_bits(&padded_image, 3, 2, '0');
        assert_eq!(surrounding_pixels, "100110001");
    }
}