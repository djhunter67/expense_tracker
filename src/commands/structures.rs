use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

pub struct Cmd {
    pub command: Option<Command>,
}

impl Cmd {
    pub fn new() -> Self {
        Self { command: None }
    }
}

impl Default for Cmd {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Command {
    Add(Add),
    List(List),
    Delete(Delete),
    Summary(Summary),
    Help,
    Version,
    None,
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Add(add) => write!(f, "{}", add),
            Command::List(list) => write!(f, "{}", list),
            Command::Delete(delete) => write!(f, "{}", delete),
            Command::Summary(summary) => write!(f, "{}", summary),
            Command::None => write!(f, "None"),
            Command::Help => write!(f, "Help"),
            Command::Version => write!(f, "Version"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Add {
    pub description: String,
    pub amount: f32,
}

impl Add {
    pub fn new(description: String, amount: f32) -> Self {
        Self {
            description,
            amount,
        }
    }
}

pub struct Delete {
    pub id: i32,
}

impl Delete {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

pub struct List {
    pub print: String,
}

impl List {
    pub fn new(list_item: Vec<String>) -> Self {
        Self {
            print: list_item.join("\n"),
        }
    }
}

pub struct Summary {
    pub month: i32,
}

impl Summary {
    pub fn new(month: i32) -> Self {
        Self { month }
    }
}

impl Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Add")
    }
}

impl Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List")
    }
}

impl Display for Delete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Delete")
    }
}

impl Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Summary")
    }
}

pub fn show_help() {
    println!(
        r#"HELP:
Usage: expense_tracker [COMMAND] 

Commands:
  add --description <DESCRIPTION> -d <DESCRIPTION> --amount <AMOUNT> -a <AMOUNT>
  list     
  delete   
  summary --month <MONTH> -m <MONTH>
  help     Print this message or the help of the given subcommand(s)

Options:
  -a, --output <DESCRIPTION>  
  -l, --list <LIST>           
  -v, --verbose               
  -h, --help                  Print help
  -V, --version               Print version
"#
    );
}
