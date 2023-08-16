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
    petal_widths: [f32; 5],
    petal_heights: [f32; 5],
}

impl AnimatedFlower {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let petal_widths: [f32; 5] = [
            rng.gen_range(60.0..120.0),
            rng.gen_range(60.0..120.0),
            rng.gen_range(60.0..120.0),
            rng.gen_range(60.0..120.0),
            rng.gen_range(60.0..120.0),
        ];
        let petal_heights: [f32; 5] = [
            rng.gen_range(100.0..200.0),
            rng.gen_range(100.0..200.0),
            rng.gen_range(100.0..200.0),
            rng.gen_range(100.0..200.0),
            rng.gen_range(100.0..200.0),
        ];

        AnimatedFlower {
            petal_rotation: 0.0,
            petal_widths,
            petal_heights,
        }
    }

    pub fn draw(&self, draw: &Draw) {
        let colors = [RED, BLUE, YELLOW, GREEN, CYAN];

        for i in 0..5 {
            let x_pos = (i as f32 * 72.0).to_radians().cos() * 75.0;
            let y_pos = (i as f32 * 72.0).to_radians().sin() * 75.0;

            draw.ellipse()
                .color(colors[i])
                .w_h(self.petal_widths[i], self.petal_heights[i])
                .rotate(self.petal_rotation + (i as f32 * 72.0).to_radians())
                .x_y(x_pos, y_pos);
        }

        draw.ellipse()
            .color(DARKORANGE)
            .w_h(50.0, 50.0)
            .finish();
    }

    pub fn update(&mut self) {
        self.petal_rotation += 0.01;
    }
}
