use crate::FuncParser;

impl FuncParser<'_> {
    pub fn accept_more_param(&self) -> bool {
        match self.func.max_param_count() {
            -1 => true,
            0 => false,
            cnt => (self.params.len() as i64) < cnt,
        }
    }
}
