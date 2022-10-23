/// Renaming of type `Vec<String>`
/// 
/// Stores the arguments that are passed into a function.
/// 
/// They are always in the same order that they were declared
/// when creating the command.
pub type FunctionInput = Vec<String>;

/// Renaming of type `Vec<FunctionOption>`
/// 
/// Stores the options that are passed into a function.
/// 
/// They are not guaranteed to be in any order since
/// options can be passed into the command line in any
/// order.
/// 
/// See [FunctionOption] for more information.
pub type FunctionOptions = Vec<FunctionOption>;

/// Renaming of type `Result<Option<String>, String>
/// 
/// This type is returned from all Climb commands.
pub type FunctionResult = Result<Option<String>, String>;

/// Renaming of type `fn(FunctionInput, FunctionOptions) -> FunctionResult`.
/// 
/// This is the general signature for all climb functions. If you
/// create a function with this signature, you can then pass it
/// into a command to be executed.
/// 
/// See [FunctionInput], [FunctionOptions], [FunctionResult] for more
/// details.
pub type CommandFunction = fn(FunctionInput, FunctionOptions) -> FunctionResult;

/// Stores the options that are passed into functions
/// 
/// If an option takes an input, it is stored in
/// the second variable as Some(input), otherwise it is None
#[derive(PartialEq)]
pub struct FunctionOption(pub String, pub Option<String>);
