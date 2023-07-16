#[derive(Debug)]
pub struct Data {}

impl Data {
    pub fn crunch(&self) -> i32 {
        32
    }
}

#[derive(Debug, Default)]
pub struct Widget(Option<Box<Data>>);

impl Widget {
    pub fn data_a(&self) -> &Option<Box<Data>> {
        &self.0
    }

    pub fn data_b(&self) -> Option<&Data> {
        self.0.as_deref()
    }
}

fn do_something_a(_: &Option<Data>) {}

fn do_something_b(_: Option<&Data>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let widget = Widget::default();
        let a: &Option<Box<Data>> = widget.data_a();
        let b: Option<&Data> = widget.data_b();

        assert_eq!(a.is_some(), b.is_some());

        let crunch_a = a.as_ref().map(|data| data.crunch());
        let crunch_b = b.map(|data| data.crunch());

        let data: &Data = &Data {};
        // do_something_a(&Some(data));
        do_something_b((Some(data));
    }
}
