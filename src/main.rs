extern crate piston_window;

use piston_window::*;

fn draw_star(c: Context, g: &mut G2d, x: f64, y: f64, size: f64, points: usize) {
    let angle_step = 360.0 / points as f64;
    let mut polygon_points = Vec::new();
    for i in 0..points {
        let angle_deg = angle_step * i as f64;
        let angle_rad = std::f64::consts::PI / 180.0 * angle_deg;
        polygon_points.push([x + angle_rad.cos() * size, y + angle_rad.sin() * size]);
    }
    polygon([0.0, 0.0, 0.0, 1.0], &polygon_points, c.transform, g);
}

fn draw_petals(c: Context, g: &mut G2d, x: f64, y: f64, size: f64, count: usize) {
    let angle_step = 360.0 / count as f64;
    for i in 0..count {
        let angle_deg = angle_step * i as f64;
        let angle_rad = std::f64::consts::PI / 180.0 * angle_deg;
        let x1 = x + angle_rad.cos() * size * 1.3;
        let y1 = y + angle_rad.sin() * size * 1.3;
        let x2 = x + angle_rad.cos() * size * 0.7;
        let y2 = y + angle_rad.sin() * size * 0.7;
        line([0.0, 0.0, 0.0, 1.0], 2.0, [x1, y1, x2, y2], c.transform, g);
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Flower Pattern", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            draw_star(c, g, 400.0, 300.0, 100.0, 8);
            draw_petals(c, g, 400.0, 300.0, 100.0, 8);
        });
    }
}
