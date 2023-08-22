extern crate piston_window;

use piston_window::*;

fn main() {
    let width = 800;
    let height = 800;
    let square_size = width as f64 * 0.4;
    let mut window: PistonWindow = WindowSettings::new("8-Pointed Star", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g); // Clear the screen

            // Define the points for the two squares
            let square1 = rectangle::square(-square_size / 2.0, -square_size / 2.0, square_size);
            let square2 = rectangle::square(-square_size / 2.0, -square_size / 2.0, square_size);

            // Draw the first square
            rectangle(
                [0.0, 0.0, 0.0, 1.0], // black color
                square1,
                c.transform.trans(width as f64 * 0.5, height as f64 * 0.5),
                g,
            );

            // Draw the second square (rotated 45 degrees)
            rectangle(
                [0.0, 0.0, 0.0, 1.0], // black color
                square2,
                c.transform
                    .trans(width as f64 * 0.5, height as f64 * 0.5)
                    .rot_deg(45.0),
                g,
            );
        });
    }
}
