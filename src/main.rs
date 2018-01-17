#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand};

fn parse_args() -> Result<Mode> {
    let matches = App::new("Fitness Karma")
        .author("Mike Lubinets <lubinetsm@yandex.ru>")
        .subcommand(SubCommand::with_name("add")
            .about("Add a new karma entity")
            .subcommand(SubCommand::with_name("fault")
                .about("Add a new fault")
                .arg(Arg::from_usage("<FAULT>")))
            .subcommand(SubCommand::with_name("punishment")
                .about("Add a new punishment")
                .arg(Arg::from_usage("<PUNISHMENT>"))
                .arg(Arg::from_usage("-a, --amount [AMOUNT]")))
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
            .subcommand(SubCommand::with_name("severity_coeff")
                .about("Set severity coefficient")
                .arg(Arg::from_usage("<SEV_COEFF>"))))
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
                    let amount = value_t!(matches.value_of("AMOUNT"), u64)
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
                    Ok(Mode::Add(Add::User(user)))
                },
                _ => bail!(ErrorKind::UnknownSubcommand)
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
                _ => bail!(ErrorKind::UnknownSubcommand)
            }
        },
        ("set", Some(matches)) => {
            match matches.subcommand() {
                ("severity_coeff", Some(matches)) => {
                    let coeff = value_t!(matches.value_of("SEV_COEFF"), f64).unwrap();
                    Ok(Mode::Set(Set::Coefficient(coeff)))
                }
                _ => bail!(ErrorKind::UnknownSubcommand)
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
        _ => bail!(ErrorKind::UnknownSubcommand)
    }
}

#[derive(Debug, Clone)]
enum Mode {
    Add(Add),
    List(List),
    Set(Set),
    Punish(Punish)
}

#[derive(Debug, Clone)]
enum Add {
    Fault(Fault),
    Punishment(Punishment),
    User(User)
}

#[derive(Debug, Clone)]
enum List {
    Faults,
    Punishments,
    Users,
    History(Username)
}

#[derive(Debug, Clone)]
enum Set {
    Coefficient(f64)
}

#[derive(Debug, Clone)]
struct Punish {
    user: String,
    fault: String,
    severity: u16
}

type Fault = String;

#[derive(Debug, Clone)]
struct Punishment {
    name: String,
    amount: u64,
}

type Username = String;
type User = Username;

fn main() {
    let mode = parse_args().unwrap();
    println!("{:#?}", mode);
}

error_chain! {
    errors {
        UnknownSubcommand {
            description("Unknown subcommand or insufficient parameters\nPlease refer to --help")
            display("Unknown subcommand or insufficient parameters\nPlease refer to --help")
        }
    }
}