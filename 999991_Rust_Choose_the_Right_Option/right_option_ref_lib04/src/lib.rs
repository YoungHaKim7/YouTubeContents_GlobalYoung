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
    pub fn data_a(&self) -> &Option<Data> {
        &self.0
    }

    pub fn data_b(&self) -> Option<&Data> {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let widget = Widget::default();
        let a: &Option<Data> = widget.data_a();
        let b: Option<&Data> = widget.data_b();

        assert_eq!(a.is_some(), b.is_some());
    }
}
