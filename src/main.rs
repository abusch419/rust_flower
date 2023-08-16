use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    flower: AnimatedFlower,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(512, 512)
        .view(view)
        .build()
        .unwrap();

    Model {
        flower: AnimatedFlower::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.flower.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    model.flower.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

struct AnimatedFlower {
    petal_rotation: f32,
    oscillation_angle: f32, // New field to control the squirming effect
}

impl AnimatedFlower {
    pub fn new() -> Self {
        AnimatedFlower {
            petal_rotation: 0.0,
            oscillation_angle: 0.0, // Initialize to 0
        }
    }

    pub fn draw(&self, draw: &Draw) {
        // Draw the center of the flower
        draw.ellipse()
            .color(ORANGE) // Corrected the color constant
            .w_h(30.0, 30.0)
            .finish(); // Added finish to complete the drawing

        for i in 0..5 {
            let oscillation = self.oscillation_angle.sin() * 10.0;

            // For simplicity, let's revert to the ellipse method for petals for now
            draw.ellipse()
                .x_y(
                    (i as f32 * 72.0).to_radians().cos() * (50.0 + oscillation),
                    (i as f32 * 72.0).to_radians().sin() * (50.0 + oscillation),
                )
                .w_h(40.0, 80.0)
                .rotate((i as f32 * 72.0).to_radians())
                .color(GREEN)
                .finish(); // Added finish to complete the drawing
        }
    }

    pub fn update(&mut self) {
        self.petal_rotation += 1.0;
        self.oscillation_angle += 0.1; // Increase the angle for the oscillation effect
    }
}

