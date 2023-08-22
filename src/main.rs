extern crate piston_window;

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
    triangles.extend(tessellate_triangle([triangle[0], mid_points[0], mid_points[2]], iterations - 1));
    triangles.extend(tessellate_triangle([mid_points[0], triangle[1], mid_points[1]], iterations - 1));
    triangles.extend(tessellate_triangle([mid_points[2], mid_points[1], triangle[2]], iterations - 1));
    triangles.extend(tessellate_triangle([mid_points[0], mid_points[1], mid_points[2]], iterations - 1));

    triangles
}

fn main() {
    let width = 800;
    let height = 800;
    let square_size = width as f64 * 0.4;
    let mut window: PistonWindow = WindowSettings::new("8-Pointed Star", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        let triangle = [
            [-100.0, -100.0],
            [100.0, -100.0],
            [0.0, 100.0],
        ];
        let tessellated = tessellate_triangle(triangle, 3);

        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g); // Clear the screen

            // Draw tessellated triangles
            for chunk in tessellated.chunks(3) {
                polygon(
                    [rand::random(), rand::random(), rand::random(), 1.0],
                    &chunk,
                    c.transform.trans(width as f64 * 0.5, height as f64 * 0.5),
                    g,
                );
            }

            // // Define the points for the two squares
            // let square1 = rectangle::square(-square_size / 2.0, -square_size / 2.0, square_size);
            // let square2 = rectangle::square(-square_size / 2.0, -square_size / 2.0, square_size);

            // // Draw the first square
            // rectangle(
            //     [0.0, 0.0, 0.0, 1.0], // black color
            //     square1,
            //     c.transform.trans(width as f64 * 0.5, height as f64 * 0.5),
            //     g,
            // );

            // // Draw the second square (rotated 45 degrees)
            // rectangle(
            //     [0.0, 0.0, 0.0, 1.0], // black color
            //     square2,
            //     c.transform
            //         .trans(width as f64 * 0.5, height as f64 * 0.5)
            //         .rot_deg(45.0),
            //     g,
            // );
        });
    }
}