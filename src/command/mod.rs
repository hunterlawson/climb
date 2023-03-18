use self::__types::__FunctionResult;

#[path = "types.rs"]
pub mod __types;

pub struct __CArg {
    pub(crate) name: &'static str,
}

impl __CArg {
    pub fn new(name: &'static str) -> Self {
        __CArg { name }
    }
}

pub struct __COpt {
    pub(crate) alias: &'static str,
    pub(crate) alias_short: Option<&'static str>,
    pub(crate) desc: &'static str,
}

impl __COpt {
    pub fn new(alias: &'static str, alias_short: Option<&'static str>, desc: &'static str) -> Self {
        __COpt { alias, alias_short, desc }
    }
}

pub struct __COArg {
    pub(crate) alias: &'static str,
    pub(crate) alias_short: Option<&'static str>,
    pub(crate) desc: &'static str,
    pub(crate) arg_name: &'static str,
}

impl __COArg {
    pub fn new(alias: &'static str, alias_short: Option<&'static str>, desc: &'static str, arg_name: &'static str) -> Self {
        __COArg { alias, alias_short, desc, arg_name }
    }
}

pub trait __Command {
    fn send_input(
        &self,
        args: Vec<String>,
        options: Vec<bool>,
        oargs: Vec<Option<String>>,
    );
    fn execute(&self) -> __FunctionResult;
    fn alias(&self) -> &'static str;
    fn alias_short(&self) -> Option<&'static str>;
    fn desc(&self) -> &'static str;
    fn args(&self) -> &Vec<__CArg>;
    fn options(&self) -> &Vec<__COpt>;
    fn optional_args(&self) -> &Vec<__COArg>;
}