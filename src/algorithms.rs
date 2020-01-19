use crate::constants::{PATH, WALL};
use crate::data_structures::{Queue, Stack};

use std::collections::HashMap;
use std::hash::BuildHasher;

// ---------
// Utilities
// ---------
fn get_start(maze: &[Vec<u8>]) -> (usize, usize) {
    // Check the top
    for (col, &square) in maze[0].iter().enumerate() {
        if square != WALL {
            return (0, col);
        }
    }

    // Check the left
    for (line, rows) in maze.iter().enumerate() {
        if rows[0] != WALL {
            return (line, 0);
        }
    }

    eprintln!("Start point not found! Exiting..");
    std::process::exit(1);
}

fn get_end(maze: &[Vec<u8>]) -> (usize, usize) {
    // Check the bottom
    for (i, &row) in maze[maze.len() - 1].iter().enumerate() {
        if row == PATH {
            return (maze.len() - 1, i);
        }
    }

    // Check the right
    for (line, rows) in maze.iter().enumerate() {
        if rows[maze[line].len() - 1] == PATH {
            return (line, maze[line].len() - 1);
        }
    }

    eprintln!("Ending point not found! Exiting..");
    std::process::exit(1);
}

fn path_up(maze: &[Vec<u8>], x: usize, y: usize, visited_square: &[Vec<bool>]) -> bool {
    if x != 0 && maze[x - 1][y] == PATH && !visited_square[x - 1][y] {
        return true;
    }

    false
}

fn path_right(maze: &[Vec<u8>], x: usize, y: usize, visited_square: &[Vec<bool>]) -> bool {
    if y != (maze[x].len() - 1) && maze[x][y + 1] == PATH && !visited_square[x][y + 1] {
        return true;
    }

    false
}

fn path_left(maze: &[Vec<u8>], x: usize, y: usize, visited_square: &[Vec<bool>]) -> bool {
    if y != 0 && maze[x][y - 1] == PATH && !visited_square[x][y - 1] {
        return true;
    }

    false
}

fn path_down(maze: &[Vec<u8>], x: usize, y: usize, visited_square: &[Vec<bool>]) -> bool {
    if x != (maze.len() - 1) && maze[x + 1][y] == PATH && !visited_square[x + 1][y] {
        return true;
    }

    false
}
// ------------
// </utilities>
// ------------

// ALWAYS LEFT
pub fn simple_always_left(maze: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut solved_path = Vec::with_capacity(maze.len() * maze[0].len());
    let mut visited_square = vec![vec![false; maze[0].len()]; maze.len()];

    let start = get_start(&maze);
    let end = get_end(&maze);

    solved_path.push(start);

    let mut current_path = start;

    // Solve it
    while current_path != end {
        let (x, y) = current_path;

        // Can I go left ?
        if path_left(&maze, x, y, &visited_square) {
            current_path = (x, y - 1);
            visited_square[x][y] = true;

            solved_path.push(current_path);
            continue;
        }

        // Can I go down ?
        if path_down(&maze, x, y, &visited_square) {
            current_path = (x + 1, y);
            visited_square[x][y] = true;

            solved_path.push(current_path);
            continue;
        }

        // Can I go right?
        if path_right(&maze, x, y, &visited_square) {
            current_path = (x, y + 1);
            visited_square[x][y] = true;

            solved_path.push(current_path);
            continue;
        }

        // Can I go up?
        if path_up(&maze, x, y, &visited_square) {
            current_path = (x - 1, y);
            visited_square[x][y] = true;

            solved_path.push(current_path);
            continue;
        }

        // Stuck..
        break;
    }

    solved_path
}

// -------------
// Wall Follower
// -------------
#[derive(PartialEq)]
enum Direction {
    LEFT,
    DOWN,
    RIGHT,
    UP,
}

pub fn wall_follower(maze: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut solved_path = Vec::with_capacity(maze.len() * maze[0].len());
    let mut visited_square = vec![vec![false; maze[0].len()]; maze.len()];

    let start = get_start(&maze);
    let end = get_end(&maze);

    let mut visit_later: Stack<(usize, usize)> = Stack::new();

    // Solve it
    let mut current_path = start;

    while current_path != end {
        let (x, y) = current_path;

        // Can I go left ?
        if path_left(&maze, x, y, &visited_square) {
            check_visit_later(
                &maze,
                &visited_square,
                current_path,
                &mut visit_later,
                Direction::LEFT,
            );

            visited_square[x][y] = true;

            solved_path.push(current_path);
            current_path = (x, y - 1);
            continue;
        }

        // Can I go down ?
        if path_down(&maze, x, y, &visited_square) {
            check_visit_later(
                &maze,
                &visited_square,
                current_path,
                &mut visit_later,
                Direction::DOWN,
            );

            visited_square[x][y] = true;

            solved_path.push(current_path);
            current_path = (x + 1, y);
            continue;
        }

        // Can I go right?
        if path_right(&maze, x, y, &visited_square) {
            check_visit_later(
                &maze,
                &visited_square,
                current_path,
                &mut visit_later,
                Direction::RIGHT,
            );

            visited_square[x][y] = true;

            solved_path.push(current_path);
            current_path = (x, y + 1);
            continue;
        }

        // Can I go up?
        if path_up(&maze, x, y, &visited_square) {
            check_visit_later(
                &maze,
                &visited_square,
                current_path,
                &mut visit_later,
                Direction::UP,
            );

            visited_square[x][y] = true;

            solved_path.push(current_path);
            current_path = (x - 1, y);
            continue;
        }

        // Dead end, mark as visited
        visited_square[x][y] = true;
        solved_path.push(current_path);

        match visit_later.pop() {
            Some(path) => current_path = path,
            None => break,
        }
    }

    solved_path.push(end);

    solved_path
}

fn check_visit_later(
    maze: &[Vec<u8>],
    visited_square: &[Vec<bool>],
    current_path: (usize, usize),
    visit_later: &mut Stack<(usize, usize)>,
    direction: Direction,
) {
    let (x, y) = current_path;

    if direction != Direction::LEFT && path_left(&maze, x, y, &visited_square) {
        visit_later.push((x, y - 1));
    }
    if direction != Direction::DOWN && path_down(&maze, x, y, &visited_square) {
        visit_later.push((x + 1, y));
    }
    if direction != Direction::RIGHT && path_right(&maze, x, y, &visited_square) {
        visit_later.push((x, y + 1));
    }
    if direction != Direction::UP && path_up(&maze, x, y, &visited_square) {
        visit_later.push((x - 1, y));
    }
}

// ----------------------
//  DEPTH FIRST SEARCH
// ----------------------
pub fn depth_first<S: BuildHasher>(
    maze: &[Vec<u8>],
    adj_list: &HashMap<(usize, usize), Vec<(usize, usize)>, S>,
) -> Vec<(usize, usize)> {
    let mut marked = vec![vec![false; maze[0].len()]; maze.len()];
    let mut stack = Stack::new();

    let maze_start = get_start(&maze);
    let maze_end = get_end(&maze);

    let mut results = Vec::with_capacity(adj_list.len());

    stack.push(maze_start);

    while !stack.empty() {
        let square = stack.pop().unwrap();
        let nodes = adj_list.get(&square).unwrap();
        if square == maze_end {
            results.push(square);
            break;
        }

        if !marked[square.0][square.1] {
            results.push(square);
            marked[square.0][square.1] = true;

            for node in nodes.iter().rev() {
                stack.push(*node);
            }
        }
    }

    results
}

// ----------------------
//  BREADTH FIRST SEARCH
// ----------------------
pub fn breadth_first<S: BuildHasher>(
    maze: &[Vec<u8>],
    adj_list: &HashMap<(usize, usize), Vec<(usize, usize)>, S>,
) -> Vec<(usize, usize)> {
    let mut marked = vec![vec![false; maze[0].len()]; maze.len()];

    let mut queue = Queue::new();

    let mut results = Vec::with_capacity(adj_list.len());

    let maze_start = get_start(&maze);
    let maze_end = get_end(&maze);

    queue.enqueue(maze_start);
    marked[maze_start.0][maze_start.1] = true;

    while !queue.empty() {
        let square = queue.dequeue();
        let nodes = adj_list.get(&square).unwrap();

        results.push(square);

        // Stop if we reached the end
        if square == maze_end {
            break;
        }

        for node in nodes.iter() {
            // If they weren't visited, visit them
            if !marked[node.0][node.1] {
                queue.enqueue(*node);
                marked[node.0][node.1] = true;
            }
        }
        // After the whole layer has been visited,
        //  visit the next layer or a node from same layer
    }

    results
}

// -----------------------------------
//  BACKTRACKING BREADTH FIRST SEARCH
// -----------------------------------
pub fn backtracking_breadth_first<S: BuildHasher>(
    maze: &[Vec<u8>],
    adj_list: &HashMap<(usize, usize), Vec<(usize, usize)>, S>,
) -> Vec<(usize, usize)> {
    let mut marked = vec![vec![false; maze[0].len()]; maze.len()];

    let mut queue = Queue::new();

    let maze_start = get_start(&maze);
    let maze_end = get_end(&maze);

    let mut parents: HashMap<(usize, usize), (usize, usize)> =
        HashMap::with_capacity(adj_list.len());

    let mut results = Vec::with_capacity(adj_list.len());
    results.push(maze_start);

    queue.enqueue(maze_start);
    marked[maze_start.0][maze_start.1] = true;

    while !queue.empty() {
        let square = queue.dequeue();
        let nodes = adj_list.get(&square).unwrap();

        // Stop if we reached the end
        if square == maze_end {
            break;
        }

        for node in nodes.iter() {
            // If they weren't visited, visit them
            if !marked[node.0][node.1] {
                queue.enqueue(*node);
                parents.insert(*node, square);
                marked[node.0][node.1] = true;
            }
        }
        // After the whole layer has been visited,
        //  visit the next layer or a node from same layer
    }

    // We now backtrack from the end to the beginning
    //  to get the best path
    let mut current = maze_end;

    while current != maze_start {
        results.push(current);
        current = *parents.get(&current).unwrap();
    }

    results
}
