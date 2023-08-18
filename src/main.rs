use image::{Rgb, RgbImage};
use rand::Rng;

fn main() {
    // Create a new RGB image canvas
    let mut canvas: RgbImage = RgbImage::new(800, 800);

    // Generate a generative art representation of a flower
    generate_flower_hypergraph_art(&mut canvas);

    // Save the canvas as an image file
    let output_path = "generative_flower_hypergraph.png";
    canvas.save(output_path).expect("Failed to save image");

    println!("Image saved as: {}", output_path);
}

fn generate_flower_hypergraph_art(canvas: &mut RgbImage) {
    let mut rng = rand::thread_rng();

    // Center of the flower
    let center_x = canvas.width() / 2;
    let center_y = canvas.height() / 2;

    // Number of petals and nodes per petal
    let num_petals = 10;
    let num_nodes_per_petal = 20;

    // Generate petals
    for i in 0..num_petals {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / (num_petals as f64);
        let petal_x = (center_x as f64 + angle.cos() * 150.0) as u32;
        let petal_y = (center_y as f64 + angle.sin() * 150.0) as u32;

        // Randomly select a color for the petal
        let petal_color = Rgb([
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(100..255),
        ]);

        // Draw nodes for the petal
        let mut nodes = Vec::new();
        for _ in 0..num_nodes_per_petal {
            let node_x = rng.gen_range(petal_x - 10..petal_x + 10);
            let node_y = rng.gen_range(petal_y - 10..petal_y + 10);
            nodes.push((node_x, node_y));
        }

        // Connect nodes to the central node
        for node in &nodes {
            draw_line(canvas, center_x, center_y, node.0, node.1, petal_color);
        }
    }
}

fn draw_line(canvas: &mut RgbImage, x1: u32, y1: u32, x2: u32, y2: u32, color: Rgb<u8>) {
    // Bresenham's line algorithm
    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = (y2 as i32 - y1 as i32).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1 as i32;
    let mut y = y1 as i32;

    loop {
        if x >= 0 && y >= 0 && x < canvas.width() as i32 && y < canvas.height() as i32 {
            canvas.put_pixel(x as u32, y as u32, color);
        }

        if x == x2 as i32 && y == y2 as i32 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
