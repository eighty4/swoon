pub use liquid::object as template_object;

use crate::api::task;

pub struct Template {
    tmpl: liquid::Template,
}

impl Template {
    pub fn new(tmpl_bytes: &[u8]) -> task::Result<Self> {
        let tmpl_str = String::from_utf8_lossy(tmpl_bytes).to_string();
        let parser = liquid::ParserBuilder::with_stdlib().build()
            .expect("liquid parser init failed");
        let template = parser.parse(tmpl_str.as_ref());
        Ok(Template {
            tmpl: template.unwrap(),
        })
    }

    pub fn render(&self, globals: &dyn liquid::ObjectView) -> task::Result<String> {
        match self.tmpl.render(globals) {
            Ok(r) => Ok(r),
            Err(e) => task::Error::result(e.to_string().as_ref()),
        }
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
