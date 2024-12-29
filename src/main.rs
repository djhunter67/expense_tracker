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

use colored::Colorize;
use expense_tracker::{
    commands::structures::{show_help, Add, Cmd, Command, Delete, List, Summary},
    models::helpers::connect_db,
    settings,
};

fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let settings = match settings::get() {
        Ok(settings) => settings,
        Err(err) => {
            println!("Failed to load settings: {err}");
            panic!("Failed to load settings");
        }
    };

    let _client = match connect_db(&settings.mongo.connection_string) {
        Ok(c) => c,
        Err(e) => {
            println!("Database connection: {}", e);
            return Err("Error".into());
        }
    };

    let args = std::env::args().collect::<Vec<String>>();
    let mut commands: Cmd = Cmd::new();

    if args.len() <= 1 {
        show_help();
        return Err("No input detected".into());
    }

    for (i, arg) in args.iter().enumerate() {
        println!("Arg {}: {}", i, arg);
    }
    println!("\n\n");

    // Check if the help is entered anywhere in the user input; not just at the beginning
    for arg in args.iter() {
        if arg.trim().eq("--help") || arg.trim().eq("-h") {
            commands.command = Some(Command::Help);
            show_help();
            return Ok(());
        } else if arg.trim().eq("--version") || arg.trim().eq("-V") || arg.trim().eq("-v") {
            println!("Version: {}", get_version());
            commands.command = Some(Command::Version);
            return Ok(());
        }
    }

    if args[1].as_str().eq("add") {
        let add_obj = match args.get(2) {
            Some(s) if s == "--description" => args[3].clone(),
            Some(s) if s == "-d" => args[3].clone(),
            _ => {
                show_help();
                println!(
                    "\n{}\n-- Arg 3: {}",
                    "Error: Description not found".red(),
                    args[3],
                );
                "Error: Description not found".red().to_string()
            }
        };

        let amount_obj = match args.get(4) {
            Some(s) if s == "--amount" => args[5].parse::<f32>().unwrap(),
            Some(s) if s == "-a" => args[5].parse::<f32>().unwrap(),
            _ => {
                show_help();
                println!(
                    "\n{}\n-- Arg 5: {}\n",
                    "Error: Description not found".red(),
                    args[5],
                );

                return Err("Error".into());
            }
        };

        let add_obj = Add::new(add_obj, amount_obj);

        println!("Add request: {}", add_obj.description);
        println!("Add request: {}", add_obj.amount);
        commands.command = Some(Command::Add(add_obj));
    } else if args[1].trim().eq("list") {
        let list_obj = List::new(vec!["1".to_string(), "2".to_string()]);
        println!("List obj: {}", list_obj.print);
        commands.command = Some(Command::List(list_obj));
    } else if args[1].trim().eq("delete") {
        if args.get(2).is_some() {
            let del_obj = Delete::new(args[3].trim().parse::<i32>().unwrap());
            println!("Del obj: {}", del_obj.id);
            commands.command = Some(Command::Delete(del_obj));
        }
    } else if args[1].as_str().eq("summary") {
        let summary_obj = match args.get(2) {
            Some(s) if s == "--month" => Summary::new(args[3].trim().parse::<i32>().unwrap()),
            Some(s) if s == "-m" => Summary::new(args[3].trim().parse::<i32>().unwrap()),
            _ => {
                show_help();
                println!(
                    "\n{}\n-- Arg 2: {}\n",
                    "Error: Month not found".red(),
                    args.get(2).unwrap_or(&"None".to_string()),
                );
                return Err("Error".into());
            }
        };

        println!("Summary obj: {}", summary_obj.month);
        commands.command = Some(Command::Summary(summary_obj));
    } else {
        show_help();
        return Err("No input recognized".into());
    }

    println!("\n\nCommand: {}", commands.command.unwrap_or(Command::None));

    Ok(())
}
