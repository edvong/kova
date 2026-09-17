use crate::types::Database;

mod dsl;
mod query;

pub use query::SelectContext;

pub trait Render<C> {
    fn render(&self, ctx: &mut C);
}

pub trait PlatformRender<T, C> {
    fn render(t: &T, ctx: &mut C);
}

// Dispatch platform-specific rendering for Standard Implementation
pub(crate) trait RenderDispatch<T, C> {
    // This method is only available if the current Extension implements the corresponding PlatformRender.
    fn dispatch(t: &T, ctx: &mut C);
}

impl<Ext, T, C> RenderDispatch<T, C> for Ext
    where
        Ext: PlatformRender<T, C> {
    fn dispatch(t: &T, ctx: &mut C) {
        // Direct passthrough to the underlying platform renderer
        <Self as PlatformRender<T, C>>::render(t, ctx)
    }
}

pub struct RenderData<'a, DB: Database> {
    pub sql: String,
    pub params: Vec<DB::Value<'a>>,
}

impl<'a, DB: Database> Default for RenderData<'a, DB> {
    fn default() -> Self {
        Self { sql: String::new(), params: Vec::new() }
    }
}

impl<'a, DB: Database> RenderData<'a, DB> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(sql_capacity: usize, params_capacity: usize) -> Self {
        Self {
            sql: String::with_capacity(sql_capacity),
            params: Vec::with_capacity(params_capacity),
        }
    }
}

impl<C> Render<C> for () {
    fn render(&self, _: &mut C) {
        // Do nothing
    }
}
