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

fn update(_app: &App, model: &mut Model, update: Update) {
    model.flower.update(update.since_start.as_secs_f32());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    model.flower.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}

struct AnimatedFlower {
    petal_rotation: f32,
    rotation_speed: f32, // Speed at which the flower rotates
    elapsed_time: f32,   // Time since the animation started
}

impl AnimatedFlower {
    pub fn new() -> Self {
        AnimatedFlower {
            petal_rotation: 0.0,
            rotation_speed: 0.02, // Initial rotation speed
            elapsed_time: 0.0,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        let colors = [RED, BLUE, YELLOW, GREEN, CYAN]; // Colors for the petals
    
        for i in 0..5 {
            // Calculate the position for each petal
            let x_pos = (i as f32 * 72.0).to_radians().cos() * 75.0;
            let y_pos = (i as f32 * 72.0).to_radians().sin() * 75.0;
    
            // Draw the petal as an ellipse (oval shape)
            draw.ellipse()
                .color(colors[i])
                .w_h(80.0, 150.0) // Width and height of the oval petal
                .rotate(self.petal_rotation + (i as f32 * 72.0).to_radians())
                .x_y(x_pos, y_pos);
        }
    
        // Draw the center of the flower on top of the petals
        draw.ellipse()
            .color(DARKGREEN)
            .w_h(50.0, 50.0)
            .finish();
    }

    pub fn update(&mut self, elapsed: f32) {
        self.elapsed_time += elapsed;
    
        // Spin at the same speed for the desired duration
        if self.elapsed_time < 5.0 {
            self.petal_rotation += self.rotation_speed;
        } else {
            // After the desired duration, gradually reduce the rotation speed to make the flower stop
            if self.rotation_speed > 0.001 { // Adjust this threshold to control the stopping duration
                self.rotation_speed -= 0.00005;
                self.petal_rotation += self.rotation_speed;
            }
        }
    }
}
