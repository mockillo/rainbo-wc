use colored::Colorize;

pub struct RainbowWriter {
    colors: Vec<(u8, u8, u8)>,
    counter: usize,
}

impl RainbowWriter {
    pub fn new() -> Self {
        RainbowWriter {
            colors: vec![
                (255, 179, 186), // Pastel Red
                (255, 223, 186), // Pastel Orange
                (255, 255, 186), // Pastel Yellow
                (186, 255, 201), // Pastel Green
                (186, 255, 255), // Pastel Cyan
                (186, 201, 255), // Pastel Blue
                (201, 186, 255), // Pastel Indigo
                (255, 186, 255), // Pastel Violet
            ],
            counter: 0,
        }
    }

    pub fn write(&mut self, string: &str) {
        for c in string.chars() {
            let color = self.colors[self.counter % self.colors.len()];
            print!("{}", c.to_string().truecolor(color.0, color.1, color.2));
            self.counter += 1;
        }
    }

    pub fn writeln(&mut self, string: &str) {
        self.write(string);
        println!();
    }
}
