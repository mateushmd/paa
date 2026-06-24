use crate::input;

pub fn solve() {
    let size = input!("board size: ", usize);
    let x = input!("cell x position: ", usize); 
    let y = input!("cell y position: ", usize);
    
    let mut board = vec![vec![0usize; size]; size];
    
    let mut current_id = 0;

    tromino(&mut board, 0, 0, size, x, y, &mut current_id);
}

fn tromino(
    b: &mut Vec<Vec<usize>>, 
    sx: usize, 
    sy: usize, 
    size: usize, 
    dx: usize, 
    dy: usize,
    id: &mut usize
) {
    if size == 1 {
        return;
    }

    let half = size / 2;
    let mid_x = sx + half;
    let mid_y = sy + half;

    *id += 1;
    let t_id = *id;

    let mut next_dx = [0; 4];
    let mut next_dy = [0; 4];

    
    if dx < mid_x && dy < mid_y {
        next_dx[0] = dx; next_dy[0] = dy;
    } else {
        b[mid_y - 1][mid_x - 1] = t_id;
        next_dx[0] = mid_x - 1; next_dy[0] = mid_y - 1;
    }

    if dx >= mid_x && dy < mid_y {
        next_dx[1] = dx; next_dy[1] = dy;
    } else {
        b[mid_y - 1][mid_x] = t_id;
        next_dx[1] = mid_x; next_dy[1] = mid_y - 1;
    }

    if dx < mid_x && dy >= mid_y {
        next_dx[2] = dx; next_dy[2] = dy;
    } else {
        b[mid_y][mid_x - 1] = t_id;
        next_dx[2] = mid_x - 1; next_dy[2] = mid_y;
    }

    if dx >= mid_x && dy >= mid_y {
        next_dx[3] = dx; next_dy[3] = dy;
    } else {
        b[mid_y][mid_x] = t_id;
        next_dx[3] = mid_x; next_dy[3] = mid_y;
    }

    tromino(b, sx, sy, half, next_dx[0], next_dy[0], id);
    tromino(b, mid_x, sy, half, next_dx[1], next_dy[1], id);
    tromino(b, sx, mid_y, half, next_dx[2], next_dy[2], id);
    tromino(b, mid_x, mid_y, half, next_dx[3], next_dy[3], id);
}
