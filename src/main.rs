use std::i32;

use image::RgbImage;
use rand::Rng;

#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

const IMG_WIDTH: u32 = 5000;
const IMG_HEIGHT: u32 = 5000;

const POINT1: Point = Point {
    x: (IMG_HEIGHT / 2) as i64,
    y: 0,
};
const POINT2: Point = Point { x: 0, y: IMG_HEIGHT as i64 };
const POINT3: Point = Point { x: IMG_WIDTH as i64, y: IMG_HEIGHT as i64 };

fn main() {
    let iterations = 100000000;
    let mut image = RgbImage::new(IMG_WIDTH + 1, IMG_HEIGHT + 1);

    //for x in 0..1000 {
    //    for y in 0..1000 {
    //        println!("Putting pixel ({}, {})", x, y);
    //        image.put_pixel(x, y, image::Rgb([255, 255, 255]));
    //    }
    //}

    draw_line(&mut image, POINT1.x, POINT1.y, POINT2.x, POINT2.y);
    draw_line(&mut image, POINT2.x, POINT2.y, POINT3.x, POINT3.y);
    draw_line(&mut image, POINT3.x, POINT3.y, POINT1.x, POINT1.y);

    let mut current_iter = 0;

    let mut start_point = get_random_triangle_point();
    loop {
        let vertex = get_random_vertex();
        let midpoint = get_midpoint(start_point.clone(), vertex.clone());

        start_point = midpoint.clone();
        draw_point(&mut image, midpoint.clone());

        if current_iter >= iterations {
            break;
        }
        println!("Current iter {}", current_iter);
        current_iter += 1;
    }

    image.save("test.png").unwrap();
}

fn get_random_triangle_point() -> Point {
    let mut rng = rand::thread_rng();

    let mut x;
    let y = rng.gen_range(0..=IMG_HEIGHT);

    loop {
        x = rng.gen_range(0..=IMG_WIDTH);

        if point_in_triangle(Point {
            x: x as i64,
            y: y as i64,
        }) {
            break;
        }
    }

    Point { x: x as i64, y: y as i64 }
}

fn get_random_vertex() -> Point {
    let mut rng = rand::thread_rng();
    let vertex = rng.gen_range(0..=2);

    match vertex {
        0 => POINT1.clone(),
        1 => POINT2.clone(),
        2 => POINT3.clone(),
        _ => panic!("Invalid vertex"),
    }
}

fn get_midpoint(p1: Point, p2: Point) -> Point {
    let part_1 = (p1.x + p2.x) / 2;
    let part_2 = (p1.y + p2.y) / 2;

    Point {
        x: part_1,
        y: part_2,
    }
}

fn draw_point(img: &mut RgbImage, p: Point) {
    img.get_pixel_mut(p.x as u32, p.y as u32).0 = [255, 255, 255];
}

fn draw_line(img: &mut RgbImage, x0: i64, y0: i64, x1: i64, y1: i64) {
    let mut x0 = x0;
    let mut y0 = y0;

    let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
    let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut err2;

    loop {
        img.get_pixel_mut(x0 as u32, y0 as u32).0 = [255, 255, 255];

        if x0 == x1 && y0 == y1 {
            break;
        };

        err2 = 2 * err;

        if err2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if err2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}

fn sign(p1: Point, p2: Point, p3: Point) -> i64 {
    return (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y);
}

fn point_in_triangle(pt: Point) -> bool {
    let d1 = sign(pt.clone(), POINT1, POINT2);
    let d2 = sign(pt.clone(), POINT2, POINT3);
    let d3 = sign(pt.clone(), POINT3, POINT1);

    let has_neg = (d1 < 0) || (d2 < 0) || (d3 < 0);
    let has_pos = (d1 > 0) || (d2 > 0) || (d3 > 0);

    return !(has_neg && has_pos);
}
