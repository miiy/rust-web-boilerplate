use crate::config::AppMetaData;
use crate::web::error::AppError;
use serde::Serialize;
use tera::Tera;

#[derive(Clone)]
pub struct Template {
    pub tera: Tera,
    pub metadata: MetaData,
}

#[derive(Serialize, Default, Clone)]
pub struct MetaData {
    pub title: String,
    pub keywords: String,
    pub description: String,
}

#[derive(Serialize)]
struct DefaultContext {
    metadata: MetaData,
}

impl Template {
    pub fn new(dir: &str) -> Result<Self, AppError> {
        let tera = Tera::new(dir)?;
        Ok(Self {
            tera,
            metadata: MetaData::default(),
        })
    }

    pub fn set_metadata(&mut self, metadata: MetaData) {
        self.metadata = metadata;
    }

    pub fn render<T: Serialize>(&self, template: &str, context: &T) -> Result<String, AppError> {
        // default context
        let mut ctx = self.default_context()?;

        ctx.extend(tera::Context::from_serialize(context)?);
        self.tera.render(template, &ctx).map_err(AppError::from)
    }

    pub fn default_context(&self) -> Result<tera::Context, tera::Error> {
        let default_ctx = DefaultContext {
            metadata: self.metadata.clone(),
        };
        tera::Context::from_serialize(&default_ctx)
    }

    pub fn render_with<T: Serialize, F>(
        &self,
        template: &str,
        context: &T,
        modifier: F,
    ) -> Result<String, AppError>
    where
        F: FnOnce(&mut tera::Context),
    {
        let mut ctx = self.default_context()?;
        ctx.extend(tera::Context::from_serialize(context)?);
        modifier(&mut ctx);
        self.tera.render(template, &ctx).map_err(AppError::from)
    }
}

impl From<AppMetaData> for MetaData {
    fn from(metadata: AppMetaData) -> Self {
        Self {
            title: metadata.title,
            keywords: metadata.keywords,
            description: metadata.description,
        }
    }
}
