use image::{Rgb, RgbImage};
use rand::Rng;

fn main() {
    // Create a new RGB image canvas
    let mut canvas: RgbImage = RgbImage::new(800, 800);

    // Generate a refined interconnected flower-like generative art representation
    generate_refined_interconnected_flower_art(&mut canvas);

    // Save the canvas as an image file
    let output_path = "refined_interconnected_flower.png";
    canvas.save(output_path).expect("Failed to save image");

    println!("Image saved as: {}", output_path);
}

fn generate_refined_interconnected_flower_art(canvas: &mut RgbImage) {
    let mut rng = rand::thread_rng();

    // Center of the flower
    let center_x = canvas.width() / 2;
    let center_y = canvas.height() / 2;

    // Number of petals and nodes per petal
    let num_petals = 12;
    let num_nodes_per_petal = 15;

    // Generate flower petals
    let mut petal_endpoints = Vec::new();
    for i in 0..num_petals {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / (num_petals as f64);
        let petal_x = (center_x as f64 + angle.cos() * 150.0) as u32;
        let petal_y = (center_y as f64 + angle.sin() * 150.0) as u32;

        // Create a color gradient for the petal
        let gradient_color = Rgb([200, 150, 200]);
        let petal_color = create_gradient_color(petal_x, petal_y, gradient_color);

        // Draw nodes for the petal
        let mut nodes = Vec::new();
        for _ in 0..num_nodes_per_petal {
            let node_distance = rng.gen_range(30..80) as f64;
            let node_angle = angle + rng.gen_range(-0.3..0.3);
            let node_x = (petal_x as f64 + node_distance * node_angle.cos()) as u32;
            let node_y = (petal_y as f64 + node_distance * node_angle.sin()) as u32;
            nodes.push((node_x, node_y));
        }

        // Store the endpoint of the current petal
        petal_endpoints.push(nodes[num_nodes_per_petal - 1]);

        // Connect some nodes to form the petal shape
        let num_connections = rng.gen_range(5..=nodes.len());
        for _ in 0..num_connections {
            let node_index = rng.gen_range(0..nodes.len());
            let (node_x, node_y) = nodes[node_index];
            draw_line(canvas, center_x, center_y, node_x, node_y, petal_color);
        }
    }

    // Connect petal endpoints using hypergraph-like connections
    for i in 0..num_petals {
        let (x1, y1) = petal_endpoints[i];
        let (x2, y2) = petal_endpoints[(i + 1) % num_petals];
        draw_line(canvas, x1, y1, x2, y2, Rgb([0, 0, 0])); // Connecting lines in black
    }

    // Add circular patterns to the center of the flower
    for _ in 0..20 {
        let pattern_radius = rng.gen_range(50..150);
        let pattern_angle = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
        let pattern_x = (center_x as f64 + pattern_radius as f64 * pattern_angle.cos()) as u32;
        let pattern_y = (center_y as f64 + pattern_radius as f64 * pattern_angle.sin()) as u32;
        draw_circle(canvas, pattern_x, pattern_y, pattern_radius, Rgb([100, 100, 100]));
    }
}

fn create_gradient_color(x: u32, y: u32, base_color: Rgb<u8>) -> Rgb<u8> {
    let gradient_factor = (x + y) / 2; // Adjust the gradient based on position
    let gradient_color = Rgb([
        base_color.0[0].saturating_sub(gradient_factor as u8),
        base_color.0[1].saturating_sub(gradient_factor as u8),
        base_color.0[2].saturating_sub(gradient_factor as u8),
    ]);
    gradient_color
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

fn draw_circle(canvas: &mut RgbImage, center_x: u32, center_y: u32, radius: u32, color: Rgb<u8>) {
    if radius == 0 {
        return; // Return early if the radius is 0
    }
    let mut x = radius as i32 - 1;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 1;
    let err_calculation = dx as i64 - ((radius as i64) << 1);
    let mut err = if err_calculation >= i32::MIN as i64 && err_calculation <= i32::MAX as i64 {
        err_calculation as i32
    } else {
        panic!("Error calculation out of bounds!");
    };




    while x >= y {
        draw_pixel(canvas, center_x + x as u32, center_y + y as u32, color);
        draw_pixel(canvas, center_x + y as u32, center_y + x as u32, color);
        draw_pixel(canvas, center_x - y as u32, center_y + x as u32, color);
        draw_pixel(canvas, center_x - x as u32, center_y + y as u32, color);
        draw_pixel(canvas, center_x - x as u32, center_y - y as u32, color);
        draw_pixel(canvas, center_x - y as u32, center_y - x as u32, color);
        draw_pixel(canvas, center_x + y as u32, center_y - x as u32, color);
        draw_pixel(canvas, center_x + x as u32, center_y - y as u32, color);

        if err <= 0 {
            y += 1;
            err += dy;
            dy += 2;
        }
        if err > 0 {
            x -= 1;
            dx += 2;
            err += (dx as i64 - ((radius as i64) << 1)) as i32;
        }
    }
}

fn draw_pixel(canvas: &mut RgbImage, x: u32, y: u32, color: Rgb<u8>) {
    if x < canvas.width() && y < canvas.height() {
        canvas.put_pixel(x, y, color);
    }
}
