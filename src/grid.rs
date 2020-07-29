use rand::{thread_rng, Rng};

pub struct Grid {
    fields: Vec<i32>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let mut fields = vec![0; width * height];

        let mut random = thread_rng();
        for i in 0..fields.len() {
            fields[i] = random.gen_range(0, 3);
        }

        Grid {
            width: width,
            height: height,
            fields: fields,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> i32 {
        self.fields[x + self.width * y]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
