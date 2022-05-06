pub use liquid::object as template_object;

use crate::api::output::file::{Directory, File};
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

    pub fn render(tmpl_bytes: &[u8], model: &dyn liquid::ObjectView) -> task::Result<String> {
        Self::new(tmpl_bytes)?.render_model(model)
    }

    pub fn render_model(&self, model: &dyn liquid::ObjectView) -> task::Result<String> {
        match self.tmpl.render(model) {
            Ok(r) => Ok(r),
            Err(e) => task::Error::result(e.to_string()),
        }
    }
}

pub trait TemplateFile: File {
    fn data(&self) -> task::Result<liquid::Object>;

    fn template(&self) -> task::Result<Template>;

    fn template_output_path(&self) -> (Directory, String);
}

impl<T> File for T where T: TemplateFile {
    fn content(&self) -> task::Result<Vec<u8>> {
        let template = self.template()?;
        let data = self.data()?;
        let content = template.render_model(&data)?;
        Ok(content.as_bytes().to_vec())
    }

    fn output_path(&self) -> (Directory, String) {
        self.template_output_path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template() {
        let tmpl = Template::new("{{ foo }} {{ bar }}".as_bytes()).unwrap();
        let result = tmpl.render_model(&template_object!({
            "foo": "foo",
            "bar": "bar",
        })).expect("no error");
        assert_eq!(result, "foo bar");
    }
}
