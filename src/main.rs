#![feature(conservative_impl_trait)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json as json;
extern crate chrono;
extern crate app_dirs;
extern crate rand;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;


mod handlers;
mod storage;
mod error;
mod types;

use types::*;
use error::*;

use clap::{Arg, ArgMatches, App, SubCommand};

pub const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {
    name: "Karma Slap",
    author: "Mike Lubinets <lubinetsm@yandex.ru>",
};

fn parse_args() -> Result<Mode> {
    let matches = App::new(APP_INFO.name)
        .author(APP_INFO.author)
        .subcommand(SubCommand::with_name("add")
            .about("Add a new karma entity")
            .subcommand(SubCommand::with_name("fault")
                .about("Add a new fault")
                .arg(Arg::from_usage("<FAULT>")))
            .subcommand(SubCommand::with_name("punishment")
                .about("Add a new punishment")
                .arg(Arg::from_usage("<PUNISHMENT>"))
                .arg(Arg::with_name("amount")
                        .short("a")
                        .takes_value(true)))
            .subcommand(SubCommand::with_name("user")
                .about("Add a new user")
                .arg(Arg::from_usage("<USERNAME>"))))
        .subcommand(SubCommand::with_name("list")
            .about("List karma entities")
            .subcommand(SubCommand::with_name("faults")
                .about("List faults"))
            .subcommand(SubCommand::with_name("punishments")
                .about("List faults"))
            .subcommand(SubCommand::with_name("users")
                .about("List faults"))
            .subcommand(SubCommand::with_name("history")
                .arg(Arg::from_usage("<USERNAME> Username who's history to show (or 'All')"))))
        .subcommand(SubCommand::with_name("set")
            .about("Set variables")
            .subcommand(SubCommand::with_name("coefficient")
                .about("Set severity coefficient")
                .arg(Arg::from_usage("<COEFFICIENT>"))))
        .subcommand(SubCommand::with_name("punish")
            .about("Make 'em suffer")
            .arg(Arg::from_usage("<USERNAME> Name of the guilty user"))
            .arg(Arg::from_usage("<FAULT> Name of the fault registered in the Fitness Karma"))
            .arg(Arg::from_usage("<SEVERITY> Severity of the fault, e.g calories")))
        .get_matches();

    match matches.subcommand() {
        ("add", Some(matches)) => {
            match matches.subcommand() {
                ("fault", Some(matches)) => {
                    let fault = matches.value_of("FAULT").unwrap()
                        .to_string();
                    Ok(Mode::Add(Add::Fault(fault)))
                },
                ("punishment", Some(matches)) => {
                    let punishment = matches.value_of("PUNISHMENT").unwrap()
                        .to_string();
                    let amount = value_t!(matches.value_of("amount"), u64)
                        .unwrap_or(1);
                    let punishment = Punishment {
                        name: punishment,
                        amount: amount
                    };
                    Ok(Mode::Add(Add::Punishment(punishment)))
                },
                ("user", Some(matches)) => {
                    let user = matches.value_of("USERNAME").unwrap()
                        .to_string();
                    let user = User::new(user);
                    Ok(Mode::Add(Add::User(user)))
                },
                _ => print_usage_and_exit(&matches)
            }
        },
        ("list", Some(matches)) => {
            match matches.subcommand() {
                ("faults", Some(_)) => {
                    Ok(Mode::List(List::Faults))
                },
                ("punishments", Some(_)) => {
                    Ok(Mode::List(List::Punishments))
                },
                ("users", Some(_)) => {
                    Ok(Mode::List(List::Users))
                },
                ("history", Some(matches)) => {
                    let user = matches.value_of("USERNAME").unwrap()
                        .to_string();
                    Ok(Mode::List(List::History(user)))
                },
                _ => print_usage_and_exit(&matches)
            }
        },
        ("set", Some(matches)) => {
            match matches.subcommand() {
                ("coefficient", Some(matches)) => {
                    let coeff = value_t!(matches.value_of("COEFFICIENT"), f64).unwrap();
                    Ok(Mode::Set(Set::Coefficient(coeff)))
                }
                _ => print_usage_and_exit(&matches)
            }
        },
        ("punish", Some(matches)) => {
            let user = matches.value_of("USERNAME").unwrap()
                .to_string();
            let fault = matches.value_of("FAULT").unwrap()
                .to_string();
            let severity = value_t!(matches.value_of("SEVERITY"), u16).unwrap();
            Ok(Mode::Punish(Punish {
                user,
                fault,
                severity
            }))
        }
        _ => print_usage_and_exit(&matches)
    }
}

fn print_usage_and_exit(matches: &ArgMatches) -> ! {
    println!("{}", matches.usage());
    std::process::exit(1);
}

use storage::Storage;

fn main() {
    let mode = parse_args().unwrap();

    let storage = Storage::load_default_path().unwrap_or_default();

    handlers::Executor::new(storage)
        .execute(mode)
        .unwrap();
}