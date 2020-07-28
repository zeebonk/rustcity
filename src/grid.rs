use rand::{thread_rng, Rng};

pub struct Grid {
    fields: [[i32; u8::MAX as usize]; u8::MAX as usize],
    width: u8,
    height: u8,
}

impl Grid {
    pub fn new(width: u8, height: u8) -> Grid {
        let mut fields = [[0; u8::MAX as usize]; u8::MAX as usize];

        let mut random = thread_rng();
        for y in 0..fields.len() {
            for x in 0..fields[y].len() {
                fields[y][x] = random.gen_range(0, 3);
            }
        }

        Grid {
            width: width,
            height: height,
            fields: fields,
        }
    }

    pub fn get(&self, x: u8, y: u8) -> i32 {
        self.fields[y as usize][x as usize]
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }
}
