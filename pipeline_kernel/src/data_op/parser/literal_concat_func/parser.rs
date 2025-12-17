use crate::{FuncDataPathParam, ParserInnerState};

pub struct LiteralConcatFuncParser<'a> {
    pub inner: ParserInnerState<'a>,
    pub params: Vec<FuncDataPathParam>,
}

