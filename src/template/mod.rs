use std::collections::HashMap;

mod parser;
mod transforms;
mod value;

pub use transforms::transform;
pub use value::Value;

pub trait DataSource {
    fn get(&self, key: &str) -> Option<Value<'_>>;
}

pub trait Transform {
    fn transform(&self, value: &str) -> String;
}

pub struct TemplateContext {
    template: String,
    transforms: HashMap<&'static str, Box<dyn Transform>>
}

impl TemplateContext {
    pub fn new<S: ToString>(template: S) -> Self {
        TemplateContext {
            template: template.to_string(),
            transforms: HashMap::new()
        }
    }

    pub fn with_transform<T: Transform + 'static>(mut self, name: &'static str, transform: T) -> Self {
        self.transforms.insert(name, Box::new(transform));
        self
    }

    pub fn render<S: DataSource>(&self, data: &S) -> String {
        let mut output = String::new();
        let parser = parser::Parser::new(&self.template);

        for segment in parser {
            match segment {
                Ok(parser::Segment::Text(text)) => output.push_str(text),
                Ok(parser::Segment::Interpolation(field)) => {
                    let value = data.get(field).unwrap_or_default();
                    output.push_str(value.as_str());
                },
                Ok(parser::Segment::Transform(field, transforms)) => {
                    let value = data.get(field).unwrap_or_default();
                    let mut value = value.to_string();

                    for transform in transforms {
                        if let Some(transform) = self.transforms.get(transform) {
                            value = transform.transform(&value).to_string();
                        }
                    }

                    output.push_str(&value);
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestDataSource<'a> {
        data: HashMap<&'a str, &'a str>
    }

    impl<'a> DataSource for TestDataSource<'a> {
        fn get(&self, key: &str) -> Option<Value> {
            self.data.get(key).map(|value| Value::Borrowed(value))
        }
    }

    struct TestTransform;

    impl Transform for TestTransform {
        fn transform(&self, value: &str) -> String {
            value.to_uppercase()
        }
    }

    #[test]
    fn test_template_context() {
        let context = TemplateContext::new("Hello, {name|uppercase}!")
            .with_transform("uppercase", TestTransform);

        let data = TestDataSource {
            data: {
                let mut data = HashMap::new();
                data.insert("name", "world");
                data
            }
        };

        assert_eq!(context.render(&data), "Hello, WORLD!");
    }
}