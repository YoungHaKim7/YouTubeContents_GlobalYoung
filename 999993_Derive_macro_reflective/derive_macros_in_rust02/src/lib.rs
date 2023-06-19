pub use reflective_derive::Reflective;
pub trait Reflective {
    fn name(&self) -> &'static str;
}
