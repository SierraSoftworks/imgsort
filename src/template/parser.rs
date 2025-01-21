use crate::errors;

#[derive(Debug, PartialEq)]
pub enum Segment<'a> {
    Text(&'a str),
    Interpolation(&'a str),
    Transform(&'a str, Vec<&'a str>),
}

pub struct Parser<'a> {
    template: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(template: &'a str) -> Self {
        Parser { template, pos: 0 }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Segment<'a>, errors::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.pos;

        match self.template.chars().nth(self.pos) {
            Some('{') => {
                let end = match self.template[self.pos..].find('}') {
                    Some(end) => end,
                    None => return Some(Err(errors::user(
                        &format!("Expected '}}' to close interpolation, but found the end of the template instead (pos: {start})."),
                        "Ensure that your template interpolations are correctly closed.")))
                };

                self.pos += end + 1;
                
                let interpolation = &self.template[start + 1..start + end];

                let mut parts = interpolation.split('|');
                let field = match parts.next() {
                    Some(field) => field,
                    None => return Some(Err(errors::user(
                        "Expected an expression within your interpolation braces, but found an empty string instead.",
                        "Make sure that your template expressions include a '{field}' or '{field|transform}'.")))
                };
                
                let transforms: Vec<_> = parts.collect();
                if transforms.is_empty() {
                    return Some(Ok(Segment::Interpolation(field)));
                }

                return Some(Ok(Segment::Transform(field, transforms)));
            },
            Some(_) => {
                let end = self.template[self.pos..].find('{').map(|i| i + self.pos).unwrap_or_else(|| self.template.len());
                self.pos = end;

                if end > start {
                    Some(Ok(Segment::Text(&self.template[start..end])))
                } else {
                    None
                }
            },
            None => None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let template = "Hello, {name}! {message|uppercase}";
        let parser = Parser { template, pos: 0 };

        let sequence = vec![
            Segment::Text("Hello, "),
            Segment::Interpolation("name"),
            Segment::Text("! "),
            Segment::Transform("message", vec!["uppercase"]),
        ];

        sequence.into_iter().zip(parser).for_each(|(expected, actual)| {
            assert_eq!(actual.expect("no parser error"), expected);
        });
    }
}