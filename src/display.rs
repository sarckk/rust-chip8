pub const DISPLAY_WIDTH: usize = 64; 
pub const DISPLAY_HEIGHT: usize = 32; 

pub struct Display {
    pub pixels: [u8; DISPLAY_WIDTH*DISPLAY_HEIGHT], // display graphics
}

impl Display {
    pub fn new() -> Display {
        Display {
            pixels: [0; DISPLAY_WIDTH*DISPLAY_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite_height: usize, sprite: &[u8]) -> u8 {
        let mut collide_flag: u8 = 0;

        for row in 0..sprite_height {
            let mut sprite: u8 = sprite[row as usize];

            for col in (0..8).rev() {
                let vx_w = (x + col) % DISPLAY_WIDTH; 
                let vy_w = (y + row) % DISPLAY_HEIGHT;

                let i = DISPLAY_WIDTH * (vy_w) + (vx_w);
                if (self.pixels[i] == 0x1) && (sprite & 0x1) == 0x1 {
                    collide_flag = 1;
                }
                self.pixels[i] ^= sprite & 0x1; 
                sprite >>= 1;
            }
        }

        collide_flag
    }
}

