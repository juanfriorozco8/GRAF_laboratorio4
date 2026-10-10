use raylib::prelude::{Color, Image, Vector2};

pub fn point(image: &mut Image, position: Vector2) {
    image.draw_pixel(
        position.x.round() as i32,
        position.y.round() as i32,
        Color::WHITE,
    );
}

pub fn line(image: &mut Image, start: Vector2, end: Vector2) {
    let mut x1 = start.x.round() as i32;
    let mut y1 = start.y.round() as i32;
    let x2 = end.x.round() as i32;
    let y2 = end.y.round() as i32;

    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        for offset_y in -1..=2 {
            for offset_x in -1..=2 {
                point(
                    image,
                    Vector2::new((x1 + offset_x) as f32, (y1 + offset_y) as f32),
                );
            }
        }
        if x1 == x2 && y1 == y2 {
            break;
        }
        let doubled_error = 2 * error;
        if doubled_error >= dy {
            error += dy;
            x1 += sx;
        }
        if doubled_error <= dx {
            error += dx;
            y1 += sy;
        }
    }
}

pub fn triangle(image: &mut Image, a: Vector2, b: Vector2, c: Vector2) {
    line(image, a, b);
    line(image, b, c);
    line(image, c, a);
}
