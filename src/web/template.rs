use crate::config::AppMetaData;
use crate::web::error::AppError;
use crate::AppState;
use serde::Serialize;

pub fn render<T: Serialize>(
    template: &str,
    context: &T,
    app_state: &AppState,
) -> Result<String, AppError> {
    let mut ctx = default_context(app_state)?;
    ctx.extend(tera::Context::from_serialize(context)?);
    app_state
        .tera
        .render(template, &ctx)
        .map_err(AppError::from)
}

pub fn render_with<T: Serialize, F>(
    template: &str,
    context: &T,
    app_state: &AppState,
    modifier: F,
) -> Result<String, AppError>
where
    F: FnOnce(&mut tera::Context),
{
    let mut ctx = default_context(app_state)?;
    ctx.extend(tera::Context::from_serialize(context)?);
    modifier(&mut ctx);
    println!("{:?}", ctx);
    app_state
        .tera
        .render(template, &ctx)
        .map_err(AppError::from)
}

#[derive(Serialize)]
struct DefaultContext {
    metadata: AppMetaData,
}

pub fn default_context(app_state: &AppState) -> Result<tera::Context, tera::Error> {
    let ctx = DefaultContext {
        metadata: app_state.metadata.clone(),
    };
    tera::Context::from_serialize(&ctx)
}
