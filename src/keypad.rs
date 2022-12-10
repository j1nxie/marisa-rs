pub struct Keypad {
    pub keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad { keys: [false; 16] }
    }

    pub fn key_down(&mut self, i: usize) {
        self.keys[i] = true;
    }

    pub fn key_up(&mut self, i: usize) {
        self.keys[i] = false;
    }

    pub fn is_key_down(&mut self, i: usize) -> bool {
        self.keys[i]
    }
}
