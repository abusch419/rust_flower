use nannou::prelude::*;
use rand::Rng;

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
            petal_rotation: 0.0,
            petal_rotation_speed: 0.01,
            elapsed_time: 0.0,
            petal_width,
            petal_height,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        let colors = [RED, BLUE, YELLOW, GREEN, CYAN, MAGENTA];

        for i in 0..6 {
            let x_pos = (i as f32 * 60.0).to_radians().cos() * 75.0;
            let y_pos = (i as f32 * 60.0).to_radians().sin() * 75.0;

            draw.ellipse()
                .color(colors[i])
                .w_h(self.petal_width, self.petal_height)
                .rotate(self.petal_rotation + (i as f32 * 60.0 + 30.0).to_radians())
                .x_y(x_pos, y_pos);
        }

        draw.ellipse()
            .color(DARKORANGE)
            .w_h(75.0, 75.0)
            .finish();
    }

    pub fn update(&mut self) {
        if self.elapsed_time < 5.0 {
            self.petal_rotation += self.petal_rotation_speed;
            self.petal_rotation_speed *= 0.995; // Gradual slowdown
            self.elapsed_time += 0.016; // Approximate time for 60fps
        }
    }
   
}
