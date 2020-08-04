pub trait Boxed
where
    Self: std::marker::Sized,
{
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
