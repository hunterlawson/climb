# climb

[![Build](https://github.com/hunterlawson/climb/actions/workflows/tests.yml/badge.svg)](https://github.com/hunterlawson/climb/actions/workflows/tests.yml)
[![Crate](https://img.shields.io/crates/v/climb)](https://crates.io/crates/climb)
[![License](https://img.shields.io/crates/l/climb)](https://github.com/hunterlawson/climb/blob/master/LICENSE)

Climb is a simple Rust crate for creating CLI applications. Allows creating commands that accept inputs, options, and optional inputs. Climb handles all input argument validation and parsing and guarantees that only the correct number of inputs and only valid options are passed into your commands. Climb will also generate help menus for your application and commands.

Climb follows the [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html) for creating applications.

- [climb](#climb)
- [Example Application - Calculator](#example-application---calculator)
  - [Creating the application](#creating-the-application)
  - [Adding functionality with commands](#adding-functionality-with-commands)
  - [Creating and using functions in climb commands](#creating-and-using-functions-in-climb-commands)
  - [Using the application](#using-the-application)

# Example Application - Calculator

## Creating the application

Let's say you want to make a simple calculator application with two commands:

* `add` - Takes in two numbers, adds them, and returns the result
* `div` - Takes in two numbers, divides them, and returns the result

In your `main.rs` file, first create a basic Climb application with the `create_app` macro:

```rust
use climb::*;

fn main() {
    let calc_app = create_app!();

    let _ = calc_app.run();
}
```

Call the `run` function to run your application with the command line arguments. Don't
worry about the return value of this for now.
The `create_app` macro will create a default application with its name, description,
and version pulled from your crate's cargo.toml file.

If you wanted to change these values you can:

```rust
let mut calc_app = create_app!();

calc_app = calc_app.name("cool_calc");
calc_app = calc_app.desc("This app does some cool math");
calc_app = calc_app.version("1.0.0");
```

Or chain the commands to make everything easier to read:

```rust
let _ = create_app!()
    .name("cool_calc")
    .desc("This app does some cool math")
    .version("1.0.0")
    .run();
```

If you run our code right now, this is what you get:

```
$ cool_calc

This app does some cool math

USAGE:
        cool_calc [OPTIONS] [COMMAND]

OPTIONS:
        -h, --help                   Print help information
        -v, --version                Print version

COMMANDS:

Run `cool_calc [COMMAND] --help` to see help information for a specific command
```

Climb created the application and added a few default options: `help` and `version`. To add more functionality to the application, you need to make use of Climb commands.

## Adding functionality with commands

To add the two commands to the app, you can use the `app::command` API. When you call the `app::command` function, you have to pass in a command struct. You can use `Command::new`:

```rust
let _ = create_app!()
    .name("cool_calc")
    .desc("This app does some cool math")
    .version("1.0.0")
    .command(
        Command::new(
            "add",
            "Add two numbers",
            add_fn
        )
        .arg("number_a")
        .arg("number_b")
    )
    .command(
        Command::new(
            "div",
            "Divide two numbers",
            div_fn
        )
        .arg("number_a")
        .arg("number_b")
        .option(
            CommandOption::new(
                "round",
                "Round the result"
            )
        )
    )
    .run();
```

The `Command::new` function takes in the command name and description as arguments. It also takes in a function that will be called when the command is executed. We haven't created definitions for these functions yet; the next section describes how to create these functions.

We're also using the `Command` API to add arguments and options to the commands. We've added two arguments to each command `number_a` and `number_b`. These will be the two numbers that the commands will operate on. We also added an option to the `div` command: `round`. If this option is added, the command should return a rounded result.

## Creating and using functions in climb commands

To add the `add` and `div` commands to the calculator application, you need to make the functions that these commands will actually execute. It's inside these functions that you can put the logic of the command.

Climb functions follow the `CommandFunction` signature. It looks like this:

```rust
type CommandFunction = fn(FunctionInput, FunctionOptions) -> FunctionResult;
```

The arguments and return type are defined as:

```rust
type FunctionInput = Vec<String>;
type FunctionOptions = Vec<FunctionOption>;
type FunctionResult = Result<Option<String>, String>;
```

It might look intimidating, but it's actually pretty simple. `FunctionInput` stores the input to our function (these are called arguments), `FunctionOptions` stores any options that our command takes. Options can also have arguments. `FunctionResult` is the standard result type that all Climb commands must return. The results of running your functions are always returned from the `App::run()` command that was used earlier.

Let's start with making the function for the `add` command. Climb functions are guaranteed to have the correct number of arguments passed into them, so you can safely assume that there are the 2 arguments passed in. If you added any options to your command, only these valid options can ever be passed into your functions:

```rust
fn add_fn(input: FunctionInput, _: FunctionOptions) -> FunctionResult {
    let num_a: i32 = input.get(0).unwrap().parse().unwrap();
    let num_b: i32 = input.get(1).unwrap().parse().unwrap();

    let result = num_a + num_b;

    println!("{}", result);

    Ok(None)
}
```

We first unwrap the two inputs and convert them to integers. Then add them and print the result. You could return the result from the function wrapped as `Ok(Some(<result>))`, but instead we're just gonna return `Ok(None)` and just print the result from inside the function.

For the `div` function: 

```rust
fn div_fn(input: FunctionInput, options: FunctionOptions) -> FunctionResult {
    let num_a: f32 = input.get(0).unwrap().parse().unwrap();
    let num_b: f32 = input.get(1).unwrap().parse().unwrap();

    let mut result = num_a / num_b;

    if options.contains(&FunctionOption(String::from("--round"), None)) {
        result = result.round();
    }

    println!("{}", result);

    Ok(None)
}
```

Just like before, we can unwrap the first two inputs. The division result is stored in the `result` variable and is printed to the console. We also check if the `--round` option is passed into the command and round the result if it is. There are many ways of doing this, but this is just one example.

## Using the application

If you run the application now with no arguments, this is what you get:

```
$ cool_calc

This app does some cool math

USAGE:
        cool_calc [OPTIONS] [COMMAND]

OPTIONS:
        -h, --help                   Print help information
        -v, --version                Print version

COMMANDS:
        add        Add two numbers
        div        Divide two numbers

Run `cool_calc [COMMAND] --help` to see help information for a specific command
```

The two commands are listed there: `add` and `div`. You can try running the command: `div --help` to see a help menu for just the `div` command:

```
$ cool_calc div --help

Divide two numbers

USAGE:
        cool_calc div [OPTIONS] <NUMBER_A> <NUMBER_B>

ARGS:
        <NUMBER_A>
        <NUMBER_B>

OPTIONS:
        -h, --help                   Print help information
            --round                  Round the result
```

Finally, we can run the commands to test that they work:

```
$ cool_calc add 45 22

67
```

```
$ cool_calc div 45 22

2.0454545
```

```
$ cool_calc div --round 45 22

2
```
