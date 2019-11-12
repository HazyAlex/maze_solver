extern crate image;
extern crate maze_solver;

use std::env::args;

use maze_solver::img_to_vec;
use maze_solver::maze_to_adjacency_list;
use maze_solver::output_maze_solution;
use maze_solver::parse_image;
use maze_solver::print_img;

use maze_solver::algorithms::breadth_first;
use maze_solver::algorithms::depth_first;
use maze_solver::algorithms::simple_always_left;
use maze_solver::algorithms::wall_follower;

fn main() {
    let img = parse_image(&mut args());
    let maze = img_to_vec(&img);
    print_img(&img);

    output_maze_solution(
        &img,                       // Reference to the image
        simple_always_left(&maze),  // Algorithm
        "examples/always_left.png"  // Output image
    );
    output_maze_solution(
        &img,
        wall_follower(&maze),
        "examples/wall_follower.png"
    );

    // For the Depth and Breadth first algorithms:
    let adj_list = maze_to_adjacency_list(&maze);

    output_maze_solution(
        &img,
        depth_first(&maze, &adj_list),
        "examples/depth_first.png",
    );

    output_maze_solution(
        &img,
        breadth_first(&maze, &adj_list),
        "examples/breadth_first.png",
    );
}
