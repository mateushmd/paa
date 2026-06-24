use crate::{ input, common::Point };

pub fn solve() {
    let n = input!("Number of points: ", usize);

    let mut p = Vec::<Point>::with_capacity(n);

    for i in 0..n {
        let x = input!(format!("x position of point {i}: "), f64);
        let y = input!(format!("y position of point {i}: "), f64);
        p.push(Point::new(x, y));
    }

    let mut p_x = p.iter().collect::<Vec<&Point>>();
    p_x.sort_by(|p1, p2| p1.x.total_cmp(&p2.x));

    let mut p_y = p.iter().collect::<Vec<&Point>>();
    p_y.sort_by(|p1, p2| p1.y.total_cmp(&p2.y));

    split(&p_x, &p_y);
}

fn split(p_x: &[&Point], p_y: &[&Point]) -> f64 {
    let n = p_x.len();
    
    if n <= 3 {
        return brute_force(p_x);
    }

    let mid = n / 2;
    let mid_point = p_x[mid];

    let mut p_y_left = Vec::with_capacity(mid);
    let mut p_y_right = Vec::with_capacity(n - mid);
    
    for &p in p_y {
        if p.x < mid_point.x || (p.x == mid_point.x && p_y_left.len() < mid) {
            p_y_left.push(p);
        } else {
            p_y_right.push(p);
        }
    }

    let d_left = split(&p_x[..mid], &p_y_left);
    let d_right = split(&p_x[mid..], &p_y_right);
    
    let d = d_left.min(d_right);

    fuse(p_y, mid_point.x, d)
}

fn fuse(p_y: &[&Point], mid_x: f64, d: f64) -> f64 {
    let mut strip = Vec::new();
    for &p in p_y {
        if (p.x - mid_x).abs() < d {
            strip.push(p);
        }
    }

    let mut min_d = d;

    for i in 0..strip.len() {
        for j in (i + 1)..strip.len() {
            if strip[j].y - strip[i].y >= min_d {
                break;
            }
            min_d = min_d.min(strip[i].distance_to(strip[j]));
        }
    }

    min_d
}

fn brute_force(points: &[&Point]) -> f64 {
    let mut min_d = f64::MAX;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            min_d = min_d.min(points[i].distance_to(points[j]));
        }
    }
    min_d
}
