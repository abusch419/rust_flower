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

    // Draw multiple flowers
    for i in 0..3 {
        for j in 0..3 {
            model.flower.draw(&draw, i as f32 * 100.0, j as f32 * 100.0, 0.3);
        }
    }

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
            petal_rotation: 130.0,
            petal_rotation_speed: 0.01,
            elapsed_time: 0.0,
            petal_width,
            petal_height,
        }
    }

    pub fn draw(&self, draw: &Draw, x_offset: f32, y_offset: f32, scale: f32) {
        let colors = [RED, BLUE, YELLOW, GREEN, CYAN, MAGENTA];

        for i in 0..6 {
            let x_pos = (i as f32 * 60.0).to_radians().cos() * 75.0 * scale + x_offset;
            let y_pos = (i as f32 * 60.0).to_radians().sin() * 75.0 * scale + y_offset;

            draw.ellipse()
                .color(colors[i])
                .w_h(self.petal_width * scale, self.petal_height * scale)
                .rotate(self.petal_rotation + (i as f32 * 60.0 + 30.0).to_radians())
                .x_y(x_pos, y_pos);
        }

        draw.ellipse()
            .color(DARKORANGE)
            .w_h(75.0 * scale, 75.0 * scale)
            .x_y(x_offset, y_offset)
            .finish();
    }

    pub fn update(&mut self) {
        if self.elapsed_time < 5.0 {
            self.petal_rotation += self.petal_rotation_speed;
            self.petal_rotation_speed *= 0.9975;
            self.elapsed_time += 0.016;
        }
    }
}
