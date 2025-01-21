use super::Transform;

pub fn transform<F>(transform: F) -> FnTransform<F>
    where F: Fn(&str) -> String
{
    FnTransform { transform }
}

pub struct FnTransform<F>
    where F: Fn(&str) -> String
{
    transform: F,
}

impl<F> Transform for FnTransform<F>
    where F: Fn(&str) -> String
{
    fn transform(&self, value: &str) -> String {
        (self.transform)(value)
    }
}