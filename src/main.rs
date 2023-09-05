use nannou::prelude::*;
//use palette::Srgb;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    flower: AnimatedFlower,
}

fn model(app: &App) -> Model {
    app.new_window().size(2253, 1550).view(view).build().unwrap();

    Model {
        flower: AnimatedFlower::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.flower.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let dark_mode = Srgb::new(
        (54.0 * 0.7) / 255.0,
        (69.0 * 0.7) / 255.0,
        (79.0 * 0.7) / 255.0
    );
    // draw.background().color(best_slate_ever);
    let light_mode = Srgb::new(230.0 / 250.0, 230.0 / 250.0, 230.0 / 250.0);
    draw.background().color(miles_color);

    // draw the flowers
    model.flower.draw(app, &draw, [326.0, -309.0], 0.47);
    model.flower.draw(app, &draw, [-680.0, 154.0], 0.75); // y: 35
    model.flower.draw(app, &draw, [-605.0, -305.0], 1.9);
    model.flower.draw(app, &draw, [650.0, 200.0], 2.5);

    draw.to_frame(app, &frame).unwrap();
}

struct AnimatedFlower {
    petal_rotation: f32,
    petal_rotation_speed: f32,
    petal_length: f32,
    petal_thickness: f32,
    growing: i8,
    spacing: f32,
    elapsed_time: f32,
}

impl AnimatedFlower {
    pub fn new() -> Self {
        let golden_ratio = 1.618;
        // let petal_length = 100.0 * golden_ratio;
        // let petal_thickness = 100.0;

        let petal_length = 100.0 * golden_ratio;
        let petal_thickness = 100.0 / (golden_ratio * golden_ratio); // 38.19

        AnimatedFlower {
            // adjust this number to change the initial rotation of the petals
            petal_rotation: 120.3, // started at 300
            petal_rotation_speed: 0.0014,
            petal_length,
            petal_thickness,
            growing: 1,
            spacing: 80.0, //75 previously
            elapsed_time: 0.0,
        }
    }

    // Inside AnimatedFlower's draw method, replace the .ellipse() call with the following
    // this function draws the flower
    pub fn draw(&self, app: &App, draw: &Draw, pos: [f32; 2], scale_factor: f32) {
        let t = app.duration.since_start.secs() as f32; // Add this line

        let color_1 = Srgb::new(98.78 / 255.0, 105.38 / 255.0, 179.3 / 255.0);
        let color_2 = Srgb::new(132.33 / 255.0, 114.29 / 255.0, 178.75 / 255.0);
        let color_3 = Srgb::new(165.88 / 255.0, 123.2 / 255.0, 178.2 / 255.0);
        let color_4 = Srgb::new(195.8 / 255.0, 128.48 / 255.0, 174.02 / 255.0);
        let color_5 = Srgb::new(223.666 / 255.0, 131.707 / 255.0, 167.787 / 255.0);
        let color_6 = Srgb::new(251.534 / 255.0, 134.933 / 255.0, 161.553 / 255.0);

        let colors = [color_1, color_2, color_3, color_4, color_5, color_6];
        let predefined_hues = [0.25, 0.35, 0.45, 0.55, 0.65, 0.75];

        for i in 0..6 {
            let x_pos = ((i as f32) * 60.0).to_radians().cos() * self.spacing * scale_factor;
            let y_pos = ((i as f32) * 60.0).to_radians().sin() * self.spacing * scale_factor;

            let hue_speed = 0.1; // Adjust this to make it faster or slower
            let hue = t * hue_speed + predefined_hues[i];
            println!("{}", hue);

            let sat = 0.5;
            let lum = 0.5;
            let a = 1.0;

            draw.ellipse()
                .color(colors[i])
                .w_h(self.petal_length * scale_factor, self.petal_thickness * scale_factor)
                .rotate(self.petal_rotation + ((i as f32) * 60.0 + 30.0).to_radians())
                .x_y(pos[0] + x_pos, pos[1] + y_pos)
                .hsla(hue, sat, lum, a)
                .finish();
        }
    }

    pub fn update(&mut self) {
        self.elapsed_time += 0.002;
        self.petal_rotation += self.petal_rotation_speed;
        self.petal_rotation_speed *= 0.99995; // Gradual slowdown
        // growing the flower
        if self.growing == 1 {
            if self.petal_thickness < 72.0 {
                self.petal_thickness += 0.01;
                self.spacing += 0.01;
            } else {
                self.growing = -1;
            }
        }
        // srinking the flower
        if self.growing == -1 {
            if self.petal_thickness > 38.19 {
                self.petal_thickness -= 0.01;
                self.spacing -= 0.01;
            } else {
                self.growing = 1;
            }
        }
    }
}
