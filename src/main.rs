extern crate image;
extern crate maze_solver;

use std::env::args;

use maze_solver::parse_image;
use maze_solver::img_to_vec;
use maze_solver::print_img;
use maze_solver::output_maze_solution;
use maze_solver::maze_to_adjacency_list;

use maze_solver::algorithms::simple_always_left;
use maze_solver::algorithms::wall_follower;
use maze_solver::algorithms::breadth_first;
use maze_solver::algorithms::depth_first;

fn main() {
    let img = parse_image(&mut args());
    let maze = img_to_vec(&img);
    print_img(&img);

    output_maze_solution(&img, simple_always_left(&maze), "examples/always_left.png");
    output_maze_solution(&img, wall_follower(&maze)     , "examples/wall_follower.png");

    let adj_list = maze_to_adjacency_list(&maze);
    output_maze_solution(&img, depth_first(&maze, &adj_list)  , "examples/depth_first.png");
    output_maze_solution(&img, breadth_first(&maze, &adj_list), "examples/breadth_first.png");
}
