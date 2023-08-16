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
    model.flower.update(_app.mouse.position());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    model.flower.draw(&draw, app.mouse.position());
    draw.to_frame(app, &frame).unwrap();
}

struct AnimatedFlower {
    petal_rotation: f32,
    oscillation_angle: f32,
    breathing_count: u32,
    hovered_petal: Option<usize>,
}

impl AnimatedFlower {
    pub fn new() -> Self {
        AnimatedFlower {
            petal_rotation: 0.0,
            oscillation_angle: 0.0,
            breathing_count: 0,
            hovered_petal: None,
        }
    }

    pub fn draw(&self, draw: &Draw, mouse_pos: Point2) {
        // Draw the center of the flower
        draw.ellipse()
            .color(DARKORANGE)
            .w_h(150.0, 150.0)
            .finish();

        for i in 0..5 {
            let oscillation = self.oscillation_angle.sin() * 15.0;
            let is_hovered = self.hovered_petal == Some(i);
            let scale_factor = if is_hovered { 1.5 } else { 1.0 };

            // Define a shorter ovular daisy petal shape using Bezier curves
            let petal_points = vec![
                pt2(0.0, 0.0) * scale_factor,
                pt2(60.0, 50.0) * scale_factor,
                pt2(40.0, 125.0) * scale_factor,
                pt2(0.0, 187.5) * scale_factor,
                pt2(-40.0, 125.0) * scale_factor,
                pt2(-60.0, 50.0) * scale_factor,
                pt2(0.0, 0.0) * scale_factor,
            ];

            // Draw the petal
            draw.polyline()
                .points(petal_points.iter().cloned())
                .color(LIGHTGREEN)
                .rotate((i as f32 * 72.0).to_radians());
        }
    }

    pub fn update(&mut self, mouse_pos: Point2) {
        if self.breathing_count < 3 {
            self.petal_rotation += 0.5;
            self.oscillation_angle += 0.05;

            if self.oscillation_angle > 2.0 * PI {
                self.breathing_count += 1;
                self.oscillation_angle = 0.0;
            }
        }

        self.hovered_petal = None;
        for i in 0..5 {
            let petal_pos = pt2(
                (i as f32 * 72.0).to_radians().cos() * 50.0,
                (i as f32 * 72.0).to_radians().sin() * 50.0,
            );
            if petal_pos.distance(mouse_pos) < 40.0 {
                self.hovered_petal = Some(i);
                break;
            }
        }
    }
}
