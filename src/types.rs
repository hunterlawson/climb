pub type FunctionInput = Vec<String>;
pub type FunctionOptions = Vec<FunctionOption>;
pub type FunctionResult = Result<Option<String>, String>;
pub type CommandFunction = fn(FunctionInput, FunctionOptions) -> FunctionResult;

pub struct FunctionOption(pub String, pub Option<String>);