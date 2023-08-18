struct LSystem {
    alphabet: String,
    axiom: String,
    rules: std::collections::HashMap<char, String>,
    angle: f32,
    length: f32,
}

impl LSystem {
    fn interpret_rules(&self, state: &str) -> String {
        let mut next_state = String::new();
        for symbol in state.chars() {
            next_state.push_str(self.rules.get(&symbol).unwrap_or(&symbol.to_string()));
        }
        next_state
    }

    // Draw a line forward based on the current position, angle, and length
    fn draw_line(&self, position: &mut (f32, f32), angle: &mut f32, draw: &Draw) {
        let (x, y) = *position;
        *position = (
            x + self.length * angle.to_radians().cos(),
            y + self.length * angle.to_radians().sin(),
        );
        draw.line()
            .start(pt2(x, y))
            .end(pt2(position.0, position.1))
            .stroke_weight(1.0)
            .color(BLACK);
    }
}

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
    petal_rotation_speed: f32,
    elapsed_time: f32,
    petal_width: f32,
    petal_height: f32,
    l_system: LSystem,
}

impl AnimatedFlower {
    pub fn new() -> Self {
        let golden_ratio = 1.618;
        let petal_width = 100.0 * golden_ratio;
        let petal_height = 100.0;

        let l_system = LSystem {
            alphabet: "F+-[]".to_string(),
            axiom: "F".to_string(),
            rules: {
                let mut rules = std::collections::HashMap::new();
                rules.insert('F', "FF+[+F-F-F]-[-F+F+F]".to_string());
                rules
            },
            angle: 15.0,
            length: 5.0,
        };

        AnimatedFlower {
            petal_rotation: 130.0,
            petal_rotation_speed: 0.01,
            elapsed_time: 0.0,
            petal_width,
            petal_height,
            l_system,
        }
    }

    pub fn update(&mut self) {
        if self.elapsed_time < 5.0 {
            self.petal_rotation += self.petal_rotation_speed;
            // make this number bigger and smaller to achieve different bloom effect
            self.petal_rotation_speed *= 0.9975; // Gradual slowdown
            self.elapsed_time += 0.016; // Approximate time for 60fps
        }
    }
    pub fn draw(&self, draw: &Draw) {
        let colors = [RED, BLUE, YELLOW, GREEN, CYAN, MAGENTA];
    
        for i in 0..6 {
            let angle_offset = (i as f32 * 60.0).to_radians();
            let x_pos = angle_offset.cos() * 75.0;
            let y_pos = angle_offset.sin() * 75.0;
            let petal_angle = self.petal_rotation + angle_offset;
    
            // Draw the petal shape
            draw.ellipse()
                .color(colors[i])
                .w_h(self.petal_width, self.petal_height)
                .rotate(petal_angle)
                .x_y(x_pos, y_pos);
    
            // Draw the L-system within the petal using a local coordinate system
            let mut local_pos = (0.0, 0.0);
            let mut local_angle = 0.0;
            let mut state = self.l_system.axiom.clone();
            for _ in 0..3 { // Fewer iterations for a looser pattern
                state = self.l_system.interpret_rules(&state);
            }
            for symbol in state.chars() {
                match symbol {
                    'F' => {
                        let l_system_length = self.l_system.length; // Adjust length for desired density
                        let global_x = x_pos + local_pos.0 * petal_angle.cos() - local_pos.1 * petal_angle.sin();
                        let global_y = y_pos + local_pos.0 * petal_angle.sin() + local_pos.1 * petal_angle.cos();
                        let next_x = global_x + l_system_length * local_angle.cos();
                        let next_y = global_y + l_system_length * local_angle.sin();
                        draw.line()
                            .start(pt2(global_x, global_y))
                            .end(pt2(next_x, next_y))
                            .stroke_weight(1.0)
                            .color(BLACK);
                        local_pos = (next_x - x_pos, next_y - y_pos);
                    }
                    '+' => local_angle += self.l_system.angle.to_radians(),
                    '-' => local_angle -= self.l_system.angle.to_radians(),
                    _ => {}
                }
            }
        }
    
        // Draw the center of the flower
        draw.ellipse()
            .color(DARKORANGE)
            .w_h(75.0, 75.0)
            .finish();
    }
    
    
    
}
