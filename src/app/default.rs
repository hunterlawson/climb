use crate::__command::{__CArg, __COArg, __COpt, __Command};

pub struct DefaultCommand {
    pub(crate) args: Vec<__CArg>,
    pub(crate) opts: Vec<__COpt>,
    pub(crate) opt_args: Vec<__COArg>,
}

impl __Command for DefaultCommand {
    fn alias(&self) -> &'static str {
        "help"
    }
    fn alias_short(&self) -> Option<&'static str> {
        None
    }
    fn args(&self) -> &Vec<__CArg> {
        &self.args
    }
    fn desc(&self) -> &'static str {
        "default help command"
    }
    fn execute(&self) -> crate::__command::__types::__FunctionResult {
        Ok(None)
    }
    fn optional_args(&self) -> &Vec<__COArg> {
        &self.opt_args
    }
    fn options(&self) -> &Vec<__COpt> {
        &self.opts
    }
    // Never used
    fn send_input(
        &self,
        _: Vec<String>,
        _: Vec<bool>,
        _: Vec<Option<String>>,
    ) {
    }
}
