#[derive(Debug)]
pub struct Data {}

impl Data {
    pub fn crunch(&self) -> i32 {
        32
    }
}

#[derive(Debug, Default)]
pub struct Widget(Option<Data>);

impl Widget {
    pub fn data_a(&mut self) -> &mut Option<Data> {
        &mut self.0
    }

    pub fn data_b(&mut self) -> Option<&mut Data> {
        self.0.as_mut()
    }
}

fn main() {
    println!("Hello, world!");
}
