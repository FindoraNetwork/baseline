use super::Context;

pub trait Module {
    type Context: Context;

    fn set_ctx(&mut self, context: Self::Context);
}
