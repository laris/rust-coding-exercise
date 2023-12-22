use crate::web::ctx;
//use thiserror::Error;
use handlebars;

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("rendering error: {0}")]
    Render(#[from] handlebars::RenderError)
}

pub struct Renderer<'a>(handlebars::Handlebars<'a>);

impl<'a> Renderer<'a> {
    pub fn new(template_dir: std::path::PathBuf) -> Self {
        let mut renderer = handlebars::Handlebars::new();
        renderer
            .register_templates_directory(".hbs", &template_dir)
            .expect("failed to register handlebars templates");
        Self(renderer)
    }
    fn convert_to_value<S>(serilizable: &S) -> serde_json::Value
        where
            S: serde::Serialize + std::fmt::Debug,
        {
            serde_json::to_value(&serilizable)
                .expect("failed to convert structure to value")
        }
    pub fn render<P>(&self, context: P, errors: &[&str]) -> String
        where
            P: ctx::PageContext + serde::Serialize + std::fmt::Debug,
        {
            let mut value = Self::convert_to_value(&context);
            if let Some(value) = value.as_object_mut() {
                value.insert("_errors".into(), errors.into());
                value.insert("_title".into(), context.title().into());
                value.insert("_base".into(), context.parent().into());
            }
            self.do_render(context.template_path(), value)
        }
     pub fn render_with_data<P, D>(
         &self, 
         context: P, 
         data: (&str, D),
         errors: &[&str])
         -> String
        where
            P: ctx::PageContext + serde::Serialize + std::fmt::Debug,
            D: serde::Serialize + std::fmt::Debug,
        {
            use handlebars::to_json;
            let mut value = Self::convert_to_value(&context);
            if let Some(value) = value.as_object_mut() {
                value.insert("_errors".into(), errors.into());
                value.insert("_title".into(), context.title().into());
                value.insert("_base".into(), context.parent().into());
                value.insert(data.0.into(), to_json(data.1));
            }
            self.do_render(context.template_path(), value)
        }

    fn do_render(&self, path: &str, ctx: serde_json::Value) -> String {
        self.0.render(path, &ctx).expect("error rendering template")
    }
}
