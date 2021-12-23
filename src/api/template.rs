pub use liquid::object as template_object;

use crate::{swoon_error_result, SwoonError};

pub struct Template {
    tmpl: liquid::Template,
}

impl Template {
    pub fn new(tmpl_bytes: &[u8]) -> Result<Self, SwoonError> {
        let tmpl_str = String::from_utf8_lossy(tmpl_bytes).to_string();
        let parser = liquid::ParserBuilder::with_stdlib().build().unwrap();
        let template = parser.parse(tmpl_str.as_ref());
        return Result::Ok(Template {
            tmpl: template.unwrap(),
        });
    }

    pub fn render(&self, globals: &dyn liquid::ObjectView) -> Result<String, SwoonError> {
        let result = self.tmpl.render(globals);
        return if result.is_ok() {
            Result::Ok(result.unwrap())
        } else {
            swoon_error_result(result.err().unwrap().to_string().as_ref())
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template() {
        let tmpl = Template::new("{{ foo }} {{ bar }}".as_bytes()).unwrap();
        let result = tmpl.render(&template_object!({
            "foo": "foo",
            "bar": "bar",
        })).expect("no error");
        assert_eq!(result, "foo bar");
    }
}
