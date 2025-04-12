pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    pub memory: [[u8; WIDTH]; HEIGHT],
    pub draw_flag: bool,
}

impl Display {
    pub fn new() -> Display {
        Display {
            memory: [[0; WIDTH]; HEIGHT],
            draw_flag: false,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.memory[y][x] = on as u8;
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        self.memory[y][x] == 1
    }

    pub fn cls(&mut self) {
        self.memory = [[0; WIDTH]; HEIGHT];
        self.draw_flag = true;
    }
}
