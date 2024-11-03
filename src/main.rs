/*
expense-tracker add --description "Lunch" --amount 20
# Expense added successfully (ID: 1)

expense-tracker add --description "Dinner" --amount 10
# Expense added successfully (ID: 2)

expense-tracker list
# ID  Date       Description  Amount
# 1   2024-08-06  Lunch        $20
# 2   2024-08-06  Dinner       $10

expense-tracker summary
# Total expenses: $30

expense-tracker delete --id 1
# Expense deleted successfully

expense-tracker summary
# Total expenses: $20

expense-tracker summary --month 8
# Total expenses for August: $20
 */

use std::{
    fmt::{self, Display},
    process::exit,
};

struct Cmd {
    command: Option<Command>,
}

impl Cmd {
    fn new() -> Self {
        Self { command: None }
    }
}

enum Command {
    Add(Add),
    List(List),
    Delete(Delete),
    Summary(Summary),
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Add(add) => write!(f, "{}", add),
            Command::List(list) => write!(f, "{}", list),
            Command::Delete(delete) => write!(f, "{}", delete),
            Command::Summary(summary) => write!(f, "{}", summary),
        }
    }
}

struct Add {
    description: String,
    amount: f32,
}

impl Add {
    fn new(description: String, amount: f32) -> Self {
        Self {
            description,
            amount,
        }
    }
}

struct Delete {
    id: i32,
}

impl Delete {
    fn new(id: i32) -> Self {
        Self { id }
    }
}

struct List {
    print: String,
}

impl List {
    fn new(list_item: Vec<String>) -> Self {
        Self {
            print: list_item.join("\n"),
        }
    }
}

struct Summary {
    month: i32,
}

impl Summary {
    fn new(month: i32) -> Self {
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

fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut commands: Cmd = Cmd::new();

    println!("\n\nArgs: {:?}\n\n", args);
    for (i, arg) in args.iter().enumerate() {
        println!("Arg {}: {}", i, arg);
    }

    if args.len() == 1 {
        show_help();
        exit(1);
    }

    if args[1].as_str().eq("--help") || args[1].as_str().eq("-h") {
        show_help();
    } else if args[1].as_str().eq("--version")
        || args[1].as_str().eq("-V")
        || args[1].as_str().eq("-v")
    {
        get_version();
    } else if args[1].as_str().eq("add") {
        let add_obj = match args.get(2) {
            Some(s) if s == "--description" => args[3].clone(),
            Some(s) if s == "-d" => args[3].clone(),
            _ => {
                show_help();
                exit(1);
            }
        };
        let amount_obj = match args.get(4) {
            Some(s) if s == "--amount" => args[5].parse::<f32>().unwrap(),
            Some(s) if s == "-a" => args[5].parse::<f32>().unwrap(),
            _ => {
                show_help();
                exit(1);
            }
        };

        let add_obj = Add::new(add_obj, amount_obj);

        println!("Add request: {}", add_obj.description);
        println!("Add request: {}", add_obj.amount);
        commands.command = Some(Command::Add(add_obj));
    } else if args[1].as_str().eq("list") {
        let list_obj = List::new(vec!["1".to_string(), "2".to_string()]);
        println!("List obj: {}", list_obj.print);
        commands.command = Some(Command::List(list_obj));
    } else if args[1].as_str().eq("delete") {
        if args.get(2).is_some() {
            let del_obj = Delete::new(args[3].parse::<i32>().unwrap());
            println!("Del obj: {}", del_obj.id);
            commands.command = Some(Command::Delete(del_obj));
        }
    } else if args[1].as_str().eq("summary") {
        let summary_obj = match args.get(2) {
            // Some(s) if s == "--month" => args[3].parse::<i32>().unwrap(),
            Some(s) if s == "--month" => Summary::new(args[3].parse::<i32>().unwrap()),
            Some(s) if s == "-m" => Summary::new(args[3].parse::<i32>().unwrap()),
            _ => {
                show_help();
                exit(1);
            }
        };

        println!("Summary obj: {}", summary_obj.month);
        commands.command = Some(Command::Summary(summary_obj));
    } else {
        show_help();
        exit(1);
    }

    println!("\n\nCommand: {}", commands.command.unwrap());

    Ok(())
}

fn show_help() {
    println!(
        r#"COMMANDS:
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
