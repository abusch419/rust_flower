use nannou::prelude::*;
use palette::Srgb;

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
            petal_rotation: 300.0,
            petal_rotation_speed: 0.002,
            elapsed_time: 0.0,
            petal_width,
            petal_height,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        let color_1 = Srgb::new(98.78 / 255.0, 105.38 / 255.0, 179.3 / 255.0);
        let color_2 = Srgb::new(132.33 / 255.0, 114.29 / 255.0, 178.75 / 255.0);
        let color_3 = Srgb::new(165.88 / 255.0, 123.2 / 255.0, 178.2 / 255.0);
        let color_4 = Srgb::new(195.8 / 255.0, 128.48 / 255.0, 174.02 / 255.0);
        let color_5 = Srgb::new(223.666 / 255.0, 131.707 / 255.0, 167.787 / 255.0);
        let color_6 = Srgb::new(251.534 / 255.0, 134.933 / 255.0, 161.553 / 255.0);
        let color_7 = Srgb::new(1.0 / 1.0, 1.0 / 1.0, 1.0 / 1.0);

        let colors = [color_1, color_2, color_3, color_4, color_5, color_6, color_7];

        for i in 0..6 {
            let x_pos = ((i as f32) * 60.0).to_radians().cos() * 75.0;
            let y_pos = ((i as f32) * 60.0).to_radians().sin() * 75.0;

            draw.ellipse()
                .color(colors[i])
                .w_h(self.petal_width, self.petal_height)
                .rotate(self.petal_rotation + ((i as f32) * 60.0 + 30.0).to_radians())
                .x_y(x_pos, y_pos);
        }

        draw.ellipse().color(color_7).w_h(75.0, 75.0).finish();
    }

    pub fn update(&mut self) {
        if self.elapsed_time < 51.0 {
            self.petal_rotation += self.petal_rotation_speed;
            // make this number bigger and smaller to achieve different bloom effect
            self.petal_rotation_speed *= 0.99995; // Gradual slowdown
            self.elapsed_time += 0.016; // Approximate time for 60fps
        }
    }
}
