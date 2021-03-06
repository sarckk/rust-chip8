pub struct Keypad {
    keys: u16, // bit field of which keys were pressed
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: 0,
        }
    }

    pub fn reset_keys(&mut self) {
        self.keys = 0;
    }

    pub fn set_key(&mut self, key: u8, pressed: bool) {
        // key is between 0 and 16
        if pressed {
            self.keys |= 1 << key;
        } else {
            // turn it off
            self.keys &= !(1 << key);
        }
    }

    pub fn is_pressed(&self, key_index: u8) -> bool {
        (self.keys & 1 << key_index) > 0
    }

    pub fn no_keys_pressed(&self) -> bool {
        self.keys == 0
    }

    pub fn get_first_key_pressed(&self) -> u8 {
        let mut i: u8 = 0;
        let mut val = self.keys;

        while val > 0 {
            if (val & 0x1) == 1 { break; }
            val >>= 1;
            i += 1;
        }

        i
    }
}
