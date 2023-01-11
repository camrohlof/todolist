use std::process;

use clap::ArgMatches;
use rusqlite::Connection;
use text_io::read;

#[derive(Debug)]
struct TodoItem {
    id: u32,
    title: String,
}

pub fn run(matches: ArgMatches, connection: Connection) {
    let action = handle_input(&matches);
    match action {
        Ok(action) => action(connection),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1)
        }
    }
}

fn handle_input(matches: &ArgMatches) -> Result<fn(Connection), &'static str> {
    match matches.subcommand_name() {
        Some("add") => Ok(add_item),
        Some("remove") => Ok(remove_item),
        Some("update") => Ok(update_item),
        Some("list") => Ok(list_items),
        _ => Err("No matching command found."),
    }
}

fn add_item(connection: Connection) {
    println!("Enter a title for the Todo:");
    let title: String = read!("{}\n");
    if !title.trim().is_empty() {
        let result = connection.execute("INSERT INTO todo (title) values (?1)", [&*title]);
        match result {
            Ok(_) => println!("Created todo named: {}", title),
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1)
            }
        }
    } else {
        eprintln!("Empty string is an invalid title");
        process::exit(1)
    }
}

fn remove_item(connection: Connection) {
    println!("Enter id of Todo to remove:");
    let id: u32 = read!();
    println!("{}", &id);
    let mut statement = connection.prepare("SELECT * FROM todo WHERE id=?").unwrap();
    let item = statement
        .query_row([&id], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        })
        .unwrap();

    println!(
        "Would you like to remove: [{}: {}]? (Y,n)",
        item.id,
        item.title.trim_end(),
    );
    let response: String = read!();
    if response.to_lowercase() == "n" || response.to_lowercase() == "no" {
        println!("Exiting.");
        process::exit(1)
    } else {
        let mut statement = connection.prepare("DELETE FROM todo WHERE id=?").unwrap();
        let result = statement.execute([item.id.to_string()]).unwrap();
    }
}

fn update_item(connection: Connection) {
    println!("Enter id of Todo to modify:");
    let id: u32 = read!();
    println!("{}", &id);
    let mut statement = connection.prepare("SELECT * FROM todo WHERE id=?").unwrap();
    let item = statement
        .query_row([&id], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        })
        .unwrap();

    println!(
        "Change the title of: [{}: {}]? (Y,n)",
        item.id,
        item.title.trim_end(),
    );
    let response: String = read!();
    if response.to_lowercase() == "n" || response.to_lowercase() == "no" {
        println!("Exiting.");
        process::exit(1)
    } else {
        let mut statement = connection
            .prepare("UPDATE todo SET title=? WHERE id=?")
            .unwrap();
        let result = statement
            .execute([item.title, item.id.to_string()])
            .unwrap();
    }
}

fn list_items(connection: Connection) {
    let mut statement = connection.prepare("SELECT * FROM todo").unwrap();
    let results = statement
        .query_map([], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        })
        .unwrap();
    for result in results {
        let data = result.unwrap();
        println!("{}: {:?}", data.id, data.title.trim_end())
    }
}
