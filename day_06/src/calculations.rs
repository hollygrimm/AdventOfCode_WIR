use ndarray::Array2;
use crate::errors::AppError;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_movement(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

pub fn count_guard_path(mut grid: Array2<char>) -> Result<i32, AppError> {
    // Find starting position and direction
    let (start_pos, direction) = find_start_position(&grid)
        .ok_or(AppError::NoStartPosition)?;
    
    let mut pos = start_pos;
    let mut facing = direction;
    let mut path_count = 0;

    // Mark the starting position with X
    grid[start_pos] = 'X';
    path_count += 1;

    loop {
        // Mark current position
        if grid[pos] == '.' {
            grid[pos] = 'X';
            path_count += 1;
        }

        // Check if we've reached an edge
        if is_at_edge(&grid, pos) {
            break;
        }

        // Get next position
        let (next_pos, new_direction) = get_next_position(&grid, pos, facing);
        pos = next_pos;
        facing = new_direction;
    }

    Ok(path_count)
}

fn find_start_position(grid: &Array2<char>) -> Option<((usize, usize), Direction)> {
    for (i, &cell) in grid.iter().enumerate() {
        if cell == '^' {
            let pos = (i / grid.ncols(), i % grid.ncols());
            return Some((pos, Direction::Up));
        } else if cell == '>' {
            let pos = (i / grid.ncols(), i % grid.ncols());
            return Some((pos, Direction::Right));
        } else if cell == 'v' {
            let pos = (i / grid.ncols(), i % grid.ncols());
            return Some((pos, Direction::Down));
        } else if cell == '<' {
            let pos = (i / grid.ncols(), i % grid.ncols());
            return Some((pos, Direction::Left));
        }
    }
    None
}

fn is_at_edge(grid: &Array2<char>, pos: (usize, usize)) -> bool {
    pos.0 == 0 || pos.0 == grid.nrows() - 1 || 
    pos.1 == 0 || pos.1 == grid.ncols() - 1
}

fn get_next_position(
    grid: &Array2<char>, 
    pos: (usize, usize), 
    facing: Direction
) -> ((usize, usize), Direction) {
    let (dr, dc) = facing.get_movement();
    let next_row = (pos.0 as i32 + dr) as usize;
    let next_col = (pos.1 as i32 + dc) as usize;

    // Check if next position is obstructed
    if next_row >= grid.nrows() || next_col >= grid.ncols() || 
       grid[(next_row, next_col)] == '#' {
        // Turn right and try again
        let new_direction = facing.turn_right();
        let (dr, dc) = new_direction.get_movement();
        let next_row = (pos.0 as i32 + dr) as usize;
        let next_col = (pos.1 as i32 + dc) as usize;
        ((next_row, next_col), new_direction)
    } else {
        // Move forward
        ((next_row, next_col), facing)
    }
}

fn get_possible_obstructions(
    grid: &Array2<char>,
    guard_pos: (usize, usize)
) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    
    // Iterate through all grid positions
    for row in 0..grid.nrows() {
        for col in 0..grid.ncols() {
            let pos = (row, col);
            // Skip if:
            // - It's the guard's position
            // - It's already an obstruction (#)
            if pos != guard_pos && 
               grid[pos] == '.' {
                positions.push(pos);
            }
        }
    }
    
    positions
}

pub fn count_loop_obstructions(grid: Array2<char>) -> Result<usize, AppError> {
    // Find starting position and direction
    let (guard_pos, _) = find_start_position(&grid)
        .ok_or(AppError::NoStartPosition)?;
    
    let possible_obstructions = get_possible_obstructions(&grid, guard_pos);
    let mut loop_count = 0;

    // Try each possible obstruction
    for obs_pos in possible_obstructions {
        let mut test_grid = grid.clone();
        test_grid[obs_pos] = '#';  // Place obstruction

        // Run the guard path and check if it forms a loop
        if let Ok(path_count) = count_guard_path(test_grid) {
            // If the guard hasn't reached an edge (indicated by path_count being > 0)
            // then we've found a loop
            if path_count > 0 {
                loop_count += 1;
            }
        }
    }

    Ok(loop_count)
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;
    
    #[test]
    fn test_guard_path_count() -> Result<(), Box<dyn std::error::Error>> {
        let grid = read_file("data/inputtest")?;
        let path_count = count_guard_path(grid)?;
        assert_eq!(path_count, 41);
        Ok(())
    }

    #[test]
    fn test_possible_obstructions() {
        let mut grid = Array2::from_elem((4, 4), '.');
        grid[(1, 1)] = '^';  // Guard position
        grid[(0, 0)] = '#';  // Existing obstruction

        let obstructions = get_possible_obstructions(&grid, (1, 1));
        
        // Should not include:
        // - Guard position (1,1)
        // - Existing obstruction (0,0)
        assert!(obstructions.contains(&(1, 2)));
        assert!(obstructions.contains(&(2, 1)));
        assert!(!obstructions.contains(&(1, 1))); // Guard position
        assert!(!obstructions.contains(&(0, 0))); // Edge
    }

    #[test]
    fn test_count_loop_obstructions() -> Result<(), Box<dyn std::error::Error>> {
        let grid = read_file("data/inputtest")?;
        let loop_count = count_loop_obstructions(grid)?;
        assert_eq!(loop_count, 6);
        Ok(())
    }
}