use nannou::prelude::*;
use palette::Srgb;

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
    let best_slate_ever = Srgb::new(
        (54.0 * 0.7) / 255.0,
        (69.0 * 0.7) / 255.0,
        (79.0 * 0.7) / 255.0
    );
    draw.background().color(best_slate_ever);

    // draw the flowers
    model.flower.draw(&draw, [427.0, -359.0], 0.47);
    model.flower.draw(&draw, [-680.0, 35.0], 0.75);
    model.flower.draw(&draw, [-605.0, -305.0], 1.9);
    model.flower.draw(&draw, [870.0, 280.0], 3.0);

    draw.to_frame(app, &frame).unwrap();
}

struct AnimatedFlower {
    petal_rotation: f32,
    petal_rotation_speed: f32,
    elapsed_time: f32,
    petal_width: f32,
    petal_height: f32,
    growing: i8,
}

impl AnimatedFlower {
    pub fn new() -> Self {
        let golden_ratio = 1.618;
        // let petal_width = 100.0 * golden_ratio;
        // let petal_height = 100.0;

        let petal_width = 100.0 * golden_ratio;
        let petal_height = 100.0 / (golden_ratio * golden_ratio); // 74.086

        AnimatedFlower {
            // adjust this number to change the initial rotation of the petals
            // 130 seems to give us back a closed bloom to start with
            petal_rotation: 120.3, // started at 300
            petal_rotation_speed: 0.002,
            elapsed_time: 0.0,
            petal_width,
            petal_height,
            growing: 1,
        }
    }

    // this function draws the flower
    pub fn draw(&self, draw: &Draw, pos: [f32; 2], scale_factor: f32) {
        let color_1 = Srgb::new(98.78 / 255.0, 105.38 / 255.0, 179.3 / 255.0);
        let color_2 = Srgb::new(132.33 / 255.0, 114.29 / 255.0, 178.75 / 255.0);
        let color_3 = Srgb::new(165.88 / 255.0, 123.2 / 255.0, 178.2 / 255.0);
        let color_4 = Srgb::new(195.8 / 255.0, 128.48 / 255.0, 174.02 / 255.0);
        let color_5 = Srgb::new(223.666 / 255.0, 131.707 / 255.0, 167.787 / 255.0);
        let color_6 = Srgb::new(251.534 / 255.0, 134.933 / 255.0, 161.553 / 255.0);
        // let color_7 = Srgb::new(1.0 / 1.0, 1.0 / 1.0, 1.0 / 1.0);
        let color_7 = Srgb::new((54.0 * 0.7) / 255.0, (69.0 * 0.7) / 255.0, (79.0 * 0.7) / 255.0);

        let colors = [color_1, color_2, color_3, color_4, color_5, color_6, color_7];

        for i in 0..6 {
            let x_pos = ((i as f32) * 60.0).to_radians().cos() * 75.0 * scale_factor;
            let y_pos = ((i as f32) * 60.0).to_radians().sin() * 75.0 * scale_factor;

            // refactor this code so we can pass in a scale factor and adjust the hard coded height and width with it
            // we may need different scale factors depending on mobile vs desktop
            draw.ellipse()
                .color(colors[i])
                .w_h(self.petal_width * scale_factor, self.petal_height * scale_factor)
                .rotate(self.petal_rotation + ((i as f32) * 60.0 + 30.0).to_radians())
                .x_y(pos[0] + x_pos, pos[1] + y_pos)
                .finish();
        }

        draw.ellipse()
            .color(color_7)
            .w_h(75.0 * scale_factor, 75.0 * scale_factor)
            .x_y(pos[0], pos[1])
            .finish();
    }

    // This function runs every frame and updates the petal rotation and speed
    pub fn update(&mut self) {
        // if self.elapsed_time < 51.0 {
        
        // rotation starts at 300
        self.petal_rotation += self.petal_rotation_speed;
        //     // make this number bigger and smaller to achieve different bloom effect
        self.petal_rotation_speed *= 0.99995; // Gradual slowdown
        // self.elapsed_time += 0.016; // Approximate time for 60fps
        // growing the flower
        println!("{}", self.growing);
        if self.growing == 1 {
            println!("hi from top of growing");
            println!("{}", self.growing);
            println!("{}", self.petal_height);
            println!("{}", self.petal_rotation);
            if self.petal_height < 80.0 { // 70 
                self.petal_height += 0.05;
            } else {
                self.growing = -1;
            }
        }
        // srinking the flower
        if self.growing == -1 {
            println!("hi from top of shrinking");
            println!("{}", self.growing);
            println!("{}", self.petal_height);
            println!("{}", self.petal_rotation);
            if self.petal_height > 38.19 {
                self.petal_height -= 0.05;
            } else {
                self.growing = 1;
            }
        }
        // }
    }
}
