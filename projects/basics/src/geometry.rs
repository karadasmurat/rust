pub struct Rectangle {
    pub w: u32,
    pub h: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.w * self.h
    }

    // modify self
    // &mut self: Borrows the struct mutably
    pub fn scale(&mut self, factor: u32) {
        self.w *= factor;
        self.h *= factor;
    }
}
