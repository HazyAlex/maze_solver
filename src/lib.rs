use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::Rgba;

mod constants;
use constants::{PATH, WALL};

pub mod algorithms;
pub mod data_structures;

use std::collections::HashMap;

use std::convert::TryFrom;
use std::env::Args;

// When checking the image pixel's alpha,
//  if it's more than this value => WALL
const WALL_LIMIT: u8 = 50;

// The color that is written to the image when
//  a PATH is selected by one of the algorithms
const SOLVED_COLOR: Rgba<u8> = Rgba([255, 0, 0, 0]);

pub fn parse_image(args: &mut Args) -> DynamicImage {
    // Get the arguments (image name)
    let args = args.nth(1);

    let filename = match args {
        Some(filename) => filename,
        None => {
            eprintln!("Image not found! Exiting..");
            std::process::exit(1);
        }
    };

    get_image(filename)
}

pub fn get_image(filename: String) -> DynamicImage {
    let image = match image::open(filename) {
        Ok(image) => image,
        Err(_) => {
            eprintln!("Image not found! Exiting..");
            std::process::exit(1);
        }
    };

    image
}

pub fn print_img(img: &DynamicImage) {
    for pixel in img.pixels() {
        if pixel.0 == 0 {
            // When we reach a new line, print a new line
            println!("");
        }

        // If the RGB(r, g, b) values are bellow a certain level,
        //  it's a wall (darker pixel)
        if pixel.2[0] < WALL_LIMIT && pixel.2[1] < WALL_LIMIT && pixel.2[2] < WALL_LIMIT {
            print!("X");
            continue;
        }

        print!("-"); // Path
    }
    println!("");
}

pub fn img_to_vec(img: &DynamicImage) -> Vec<Vec<u8>> {
    // Will panic on a 16-bit platform
    let (width, height): (usize, usize) = {
        (
            usize::try_from(img.width()).unwrap(),
            usize::try_from(img.height()).unwrap(),
        )
    };
    let mut matrix = vec![vec![0; width]; height];

    let mut line = 0;
    let mut col = 0;

    // Due to the format of the img,
    //  we can't really iterate it like we usually do

    for pixel in img.pixels() {
        // pixel.0 -> column
        // pixel.1 -> line

        if pixel.0 == 0 {
            // If the column is zero, then we reached a new line
            if pixel.1 != 0 {
                // Ignore the first line
                col = 0;
                line += 1;
            }
        }

        // If the RGB(r, g, b) values are bellow a certain level,
        //  it's a wall (darker pixel)
        if pixel.2[0] < WALL_LIMIT && pixel.2[1] < WALL_LIMIT && pixel.2[2] < WALL_LIMIT {
            matrix[line][col] = WALL;

            col += 1;
            continue;
        }

        matrix[line][col] = PATH;
        col += 1;
    }

    matrix
}

pub fn print_maze_solution(maze: &Vec<Vec<u8>>, solution: Vec<(usize, usize)>) {
    for (i, row) in maze.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if col == WALL {
                print!("-"); // Wall
                continue;
            }

            if solution.contains(&(i, j)) {
                print!("X"); // Path traversed during solving
                continue;
            }

            print!("*"); // Path that wasn't traversed
        }

        println!(""); // Newline
    }
}

pub fn output_maze_solution(img: &DynamicImage, solution: Vec<(usize, usize)>, output: &str) {
    let mut new_img = img.clone();

    for (x, y) in solution.iter() {
        new_img.put_pixel(
            u32::try_from(*y).unwrap(),
            u32::try_from(*x).unwrap(),
            SOLVED_COLOR,
        );
    }

    match new_img.save(output) {
        Ok(_) => println!("Solution printed to: {}", output),
        Err(e) => eprintln!("Error saving image!\n{}", e),
    }
}

pub fn maze_to_adjacency_list(maze: &Vec<Vec<u8>>) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut adj_list = HashMap::with_capacity(maze.len() * maze[0].len());

    for (i, row) in maze.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            // No path from a WALL to anywhere else
            if maze[i][j] == WALL {
                continue;
            }

            // Check if there is a path in the 4 possible directions
            // Check down
            if i != (maze.len() - 1) && maze[i + 1][j] == PATH {
                // We don't have to check if a key is in there because it's
                //  the first time it's added
                let mut vec = Vec::with_capacity(4);

                vec.push((i + 1, j));
                adj_list.insert((i, j), vec);
            }

            // Check right
            if j != (maze[0].len() - 1) && maze[i][j + 1] == PATH {
                add_node(&mut adj_list, (i, j), (i, j + 1));
            }

            // Check up
            if i != 0 && maze[i - 1][j] == PATH {
                add_node(&mut adj_list, (i, j), (i - 1, j));
            }

            // Check left
            if j != 0 && maze[i][j - 1] == PATH {
                add_node(&mut adj_list, (i, j), (i, j - 1));
            }
        }
    }

    adj_list
}

fn add_node(
    adj_list: &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
    (x, y): (usize, usize),   // Current node
    (dx, dy): (usize, usize), // Next node
) {
    // If a key doesn't exist, add it
    if !adj_list.contains_key(&(x, y)) {
        let mut vec = Vec::with_capacity(4);
        vec.push((dx, dy));

        adj_list.insert((x, y), vec);
        return;
    }

    match adj_list.get_mut(&(x, y)) {
        Some(vec) => {
            vec.push((dx, dy));
            vec
        }
        None => &mut vec![(dx, dy)],
    };
}
