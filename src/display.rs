pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    pub memory: [[u8; WIDTH]; HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            memory: [[0; WIDTH]; HEIGHT],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.memory[y][x] = on as u8;
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        self.memory[y][x] == 1
    }

    pub fn cls(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.set_pixel(x, y, false);
            }
        }
    }
}
