use super::Context;

pub trait Module: Clone + Send + Sync + 'static {
    type Context: Context;

    fn set_ctx(&mut self, context: Self::Context);
}
