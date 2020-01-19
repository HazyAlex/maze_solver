#![feature(test)]
extern crate maze_solver;

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    const FILENAME: &str = "maze_100_100_complex.png";

    use super::maze_solver::get_image;
    use super::maze_solver::img_to_vec;
    use super::maze_solver::maze_to_adjacency_list;

    use super::maze_solver::algorithms::*;

    #[bench]
    fn bench_get_image(b: &mut Bencher) {
        b.iter(|| {
            get_image(String::from(FILENAME));
        });
    }

    #[bench]
    fn bench_img_to_vec(b: &mut Bencher) {
        let img = get_image(String::from(FILENAME));

        b.iter(|| {
            img_to_vec(&img);
        });
    }

    #[bench]
    fn bench_simple_always_left(b: &mut Bencher) {
        let img = get_image(String::from(FILENAME));
        let maze = img_to_vec(&img);

        b.iter(|| {
            simple_always_left(&maze);
        });
    }

    #[bench]
    fn bench_wall_follower(b: &mut Bencher) {
        let img = get_image(String::from(FILENAME));
        let maze = img_to_vec(&img);

        b.iter(|| {
            wall_follower(&maze);
        });
    }

    #[bench]
    fn bench_maze_to_adjacency_list(b: &mut Bencher) {
        let img = get_image(String::from(FILENAME));
        let maze = img_to_vec(&img);

        b.iter(|| {
            maze_to_adjacency_list(&maze);
        });
    }

    #[bench]
    fn bench_depth_first_search(b: &mut Bencher) {
        let img = get_image(String::from(FILENAME));
        let maze = img_to_vec(&img);
        let adj_list = maze_to_adjacency_list(&maze);

        b.iter(|| {
            depth_first(&maze, &adj_list);
        });
    }

    #[bench]
    fn bench_breadth_first_search(b: &mut Bencher) {
        let img = get_image(String::from(FILENAME));
        let maze = img_to_vec(&img);
        let adj_list = maze_to_adjacency_list(&maze);

        b.iter(|| {
            breadth_first(&maze, &adj_list);
        });
    }

    #[bench]
    fn bench_backtracking_breadth_first_search(b: &mut Bencher) {
        let img = get_image(String::from(FILENAME));
        let maze = img_to_vec(&img);
        let adj_list = maze_to_adjacency_list(&maze);

        b.iter(|| {
            backtracking_breadth_first(&maze, &adj_list);
        });
    }
}
