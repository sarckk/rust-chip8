use minifb::Key;

pub struct Keypad {
    keys: u16, // bit field of which keys were pressed
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: 0,
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

    pub fn register_keypresses(&mut self, keys: Vec<Key>) {
        self.keys = 0;

        for key in keys.iter() {
            let exponent = match key {
                Key::Key1 => Some(0x1),
                Key::Key2 => Some(0x2),
                Key::Key3 => Some(0x3),
                Key::Key4 => Some(0xC),
                Key::Q =>    Some(0x4),
                Key::W =>    Some(0x5),
                Key::E =>    Some(0x6),
                Key::R =>    Some(0xD),
                Key::A =>    Some(0x7),
                Key::S =>    Some(0x8),
                Key::D =>    Some(0x9),
                Key::F =>    Some(0xE),
                Key::Z =>    Some(0xA),
                Key::X =>    Some(0x0),
                Key::C =>    Some(0xB),
                Key::V =>    Some(0xF),
                _ => None
            };

            
            if let Some(exp) = exponent {
                self.keys = 1 << exp;
                break;
            } 
        }
    }


}
