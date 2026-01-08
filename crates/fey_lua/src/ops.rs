use mlua::Function;

#[derive(Default, Clone)]
pub(crate) struct Ops {
    pub index: Option<Function>,
    pub newindex: Option<Function>,
    pub call: Option<Function>,
    pub tostring: Option<Function>,
    pub add: Option<Function>,
    pub sub: Option<Function>,
    pub mul: Option<Function>,
    pub div: Option<Function>,
    pub unm: Option<Function>,
    pub r#mod: Option<Function>,
    pub pow: Option<Function>,
    pub idiv: Option<Function>,
    pub band: Option<Function>,
    pub bor: Option<Function>,
    pub bxor: Option<Function>,
    pub bnot: Option<Function>,
    pub shl: Option<Function>,
    pub shr: Option<Function>,
    pub eq: Option<Function>,
    pub lt: Option<Function>,
    pub le: Option<Function>,
    pub concat: Option<Function>,
    pub len: Option<Function>,
}
