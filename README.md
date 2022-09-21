# climb

Climb is a simple Rust crate for creating CLI applications. Allows for functions to accept inputs, options, and optional inputs. Climb handles all input argument validation and parsing and guarantees that only the correct number of inputs and only valid options are passed into your commands.

- [climb](#climb)
  - [Commands](#commands)
    - [The ClimbFunction Signature](#the-climbfunction-signature)
    - [The CommandInput Type](#the-commandinput-type)
    - [The CommandOptions Type](#the-commandoptions-type)
    - [The CommandResult Type](#the-commandresult-type)
  - [Creating and Running an Application](#creating-and-running-an-application)
  
## Commands
Climb commands follow this signature:
```rust
pub struct Command<'a> {
    pub function: ClimbFunction,
    pub name: &'a str,
    pub alias: &'a str,
    pub options: HashSet<&'a str>,
    pub num_inputs: usize,
}
```
`function`: A pointer to the function that follows the `ClimbFunction` signature. This is the function that is executed when the command is called.
`name`: The name of the function  
`alias`: The alias used to call the command in the command line  
`options`: A `HashSet` of options that the command can take. Each option is indicated by a string slice of its name. Appending a '?' at the end of an option indicates that it accepts an input. 
`num_inputs`: The number of required inputs to the command

### The ClimbFunction Signature
The `ClimbFunction` definition looks like this:
```rust
pub type ClimbFunction = fn(CommandInput, CommandOptions) -> CommandResult;
pub type CommandInput = Option<Vec<String>>;
pub type CommandOptions = Option<Vec<CommandOption>>;
pub struct CommandOption(pub String, pub Option<String>);
pub type CommandResult = Result<Option<String>, String>;
```
The CommandInput, CommandOptions, and CommandResult types are the required types that every Climb function needs to use.

### The CommandInput Type
The `CommandInput` type is populated with the non-optional inputs for the function. If a function takes 0 inputs, this will be `None`.

### The CommandOptions Type
The `CommandOptions` type is populated with the options for the function. `CommandOption` is a tuple struct containing the `String` name of the function and an optional input value.

### The CommandResult Type
All Climb commands return the `CommandResult` type. If a command wants to print its results to the console, then it returns a `Some(String)` type.

## Creating and Running an Application
Here is an example application that acts as a simple calculator

This is the math function that the calculator will use. You can see an example of how you can parse the options and arguments that are passed into the function.
```rust
fn add(input: CommandInput, options: CommandOptions) -> CommandResult {
    let input = input.unwrap();

    let a = input.get(0).unwrap().parse::<i32>().unwrap();
    let b = input.get(1).unwrap().parse::<i32>().unwrap();

    if let Some(options_vec) = options {
        for option in options_vec {
            match option.0.as_str() {
                "a" => return Ok(Some(format!("{a} + {b} = {}", a + b))),
                "s" => return Ok(Some(format!("{a} - {b} = {}", a - b))),
                "m" => return Ok(Some(format!("{a} * {b} = {}", a * b))),
                "d" => return Ok(Some(format!("{a} / {b} = {}", a / b))),
                _ => (),
            }
        }
    }

    Ok(None)
}
```
All Climb applications need a default function. This function is called when no command is input in the command line after calling the application. Since the calculator requires us to give a command, you can make an empty function:
```rust
fn default(input: CommandInput, options: CommandOptions) -> CommandResult {
    Ok(None)
}
```

In the main function, we need to now create two `ClimbFunction` structs to represent our functions. These will be passed into the `ClimbApp` struct.
```rust
let default_command = Command {
    function: default,
    name: "Default",
    alias: "",
    options: hashset![],
    num_inputs: 0,
};

let add_command = Command {
    function: math,
    name: "math",
    alias: "math",
    options: hashset!["a", "s", "m", "d"],
    num_inputs: 2,
};
```

Now to create the application and add the functions:
```rust
let result = ClimbApp::new(default_command)?
    .add_command(add_command)?
    .run(env::args().collect());
```

We store the result of the run function so that we can use it later. In this case, we will print it:
```rust
match result {
    Ok(r) => match r {
        Some(s) => println!("{}", s),
        None => (),
    },
    Err(e) => println!("{}", e),
}
```

Now the application works as intended:
```
climb-test-app math -a 4 6
4 + 6 = 10
climb-test-app math -s 2022 10
2022 - 10 = 2012
```