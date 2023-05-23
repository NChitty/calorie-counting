pub struct Elf {
    food: Vec<u32>,
    total: u32
}

impl Elf {
    pub fn new() -> Self {
        Elf {
            food: Vec::new(),
            total: 0
        }
    }

    pub fn add_food_item(&mut self, cal: u32) {
        self.food.push(cal);
        self.update_total();
    }

    fn update_total(&mut self) {
        self.total = self.food.iter().sum();
    }

    pub fn get_total(&self) -> u32 {
        self.total
    }
}
