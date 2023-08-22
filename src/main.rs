extern crate piston_window;
use std::time::{Duration, Instant};

use piston_window::*;

type Point2 = [f64; 2];

fn tessellate_triangle(triangle: [Point2; 3], iterations: usize) -> Vec<Point2> {
    if iterations == 0 {
        return triangle.to_vec();
    }

    let mid_points = [
        [(triangle[0][0] + triangle[1][0]) / 2.0, (triangle[0][1] + triangle[1][1]) / 2.0],
        [(triangle[1][0] + triangle[2][0]) / 2.0, (triangle[1][1] + triangle[2][1]) / 2.0],
        [(triangle[0][0] + triangle[2][0]) / 2.0, (triangle[0][1] + triangle[2][1]) / 2.0],
    ];

    let mut triangles = Vec::new();
    triangles.extend(
        tessellate_triangle([triangle[0], mid_points[0], mid_points[2]], iterations - 1)
    );
    triangles.extend(
        tessellate_triangle([mid_points[0], triangle[1], mid_points[1]], iterations - 1)
    );
    triangles.extend(
        tessellate_triangle([mid_points[2], mid_points[1], triangle[2]], iterations - 1)
    );
    triangles.extend(
        tessellate_triangle([mid_points[0], mid_points[1], mid_points[2]], iterations - 1)
    );

    triangles
}

fn main() {
    let width = 800;
    let height = 800;
    let mut window: PistonWindow = WindowSettings::new("8-Pointed Star", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let colors = [
        [0.581, 0.067, 0.0, 1.0],
        [1.0, 0.149, 0.0, 1.0],
        [1.0, 0.493, 0.474, 1.0],
    ];

    let color_change_rate: usize = 20; // Change color every 20 frames
    let mut frame_count = 0;

    let mut last_color_change = Instant::now();
    let color_change_interval = Duration::from_millis(250); // Change color every 5 seconds

    while let Some(e) = window.next() {
        let triangle = [
            [-100.0, -100.0],
            [100.0, -100.0],
            [0.0, 100.0],
        ];
        let tessellated = tessellate_triangle(triangle, 3);
    
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g); // Clear the screen
    
            for (index, chunk) in tessellated.chunks_exact(3).enumerate() {
                if Instant::now() - last_color_change >= color_change_interval {
                    last_color_change = Instant::now();
                    frame_count = (frame_count + 1) % color_change_rate;
                }
    
                let color_index = (index + frame_count) % colors.len();
                let color = colors[color_index];
    
                polygon(
                    color,
                    chunk,
                    c.transform.trans((width as f64) * 0.5, (height as f64) * 0.5),
                    g
                );
            }
        });
    }
    
    
}
