use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    flower: AnimatedFlower,
}

fn model(app: &App) -> Model {
    app.new_window().size(512, 512).view(view).build().unwrap();

    Model {
        flower: AnimatedFlower::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.flower.update();
}

fn draw_lancealoid_petal(draw: &Draw, x: f32, y: f32, width: f32, height: f32, rotation: f32) {
    let tip = pt2(x, y + height / 2.0).rotate(rotation);
    let base = pt2(x, y - height / 2.0).rotate(rotation);
    let control1 = pt2(x + width / 2.0, y + height / 4.0).rotate(rotation);
    let control2 = pt2(x + width / 2.0, y - height / 4.0).rotate(rotation);

    draw.quad().points(base, control1, tip, control2).color(BLACK);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    model.flower.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

struct AnimatedFlower {
    petal_rotation: f32,
    petal_rotation_speed: f32,
    elapsed_time: f32,
    petal_width: f32,
    petal_height: f32,
}

impl AnimatedFlower {
    pub fn new() -> Self {
        let golden_ratio = 1.618;
        let petal_width = 100.0 * golden_ratio;
        let petal_height = 100.0;

        AnimatedFlower {
            // adjust this number to change the initial rotation of the petals
            // 130 seems to give us back a closed bloom to start with
            petal_rotation: 130.0,
            petal_rotation_speed: 0.01,
            elapsed_time: 0.0,
            petal_width,
            petal_height,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        // Test values for x and y positions
        let x_pos = 0.0;
        let y_pos = 0.0;

        // Test values for petal width and height
        let petal_width = 50.0;
        let petal_height = 100.0;

        // Test value for rotation
        let petal_rotation = 0.0;

        let x_center = 0.0; // Center x position of the flower
        let y_center = 0.0; // Center y position of the flower

        for i in 0..6 {
            let angle = ((i as f32) * 60.0).to_radians(); // 60-degree separation between petals
            let x_pos = x_center + 75.0 * angle.cos(); // Adjust the offset as needed
            let y_pos = y_center + 75.0 * angle.sin(); // Adjust the offset as needed

            draw_lancealoid_petal(
                &draw,
                x_pos,
                y_pos,
                self.petal_width,
                self.petal_height,
                angle // Use angle for rotation
            );
        }
    }

    fn update(&mut self) {
        if self.elapsed_time < 5.0 {
            self.petal_rotation += self.petal_rotation_speed;
            // make this number bigger and smaller to achieve different bloom effect
            self.petal_rotation_speed *= 0.9975; // Gradual slowdown
            self.elapsed_time += 0.016; // Approximate time for 60fps
        }
    }
}
