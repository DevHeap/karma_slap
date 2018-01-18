use error::*;
use types::*;
use storage::Storage;
use std::process;
use std::io;
use std::io::Write;
use std::time::Duration;
use std::thread;

use rand;
use rand::Rng;

use chrono::Local;

pub struct Executor {
    storage: Storage
}

impl Executor {
    pub fn new(storage: Storage) -> Self {
        Executor {
            storage
        }
    }

    pub fn execute(&mut self, mode: Mode) -> Result<()> {
        match mode {
            Mode::Add(add) => self.process_add(add),
            Mode::List(list) => self.process_list(list),
            Mode::Punish(punish) => self.process_punish(punish),
            Mode::Set(set) => self.process_set(set),
        }
    }

    fn process_add(&mut self, add: Add) -> Result<()> {
        match add {
            Add::Fault(name)      => self.storage.add_fault(name),
            Add::Punishment(name) => self.storage.add_punishment(name),
            Add::User(name)       => self.storage.add_user(name),
        }

        self.storage.store_default_path()
    }

    fn process_list(&self, l: List) -> Result<()> {
        match l {
            List::Faults => list(self.storage.faults()),
            List::Punishments => list(self.storage.punishments()),
            List::Users => list(self.storage.users()),
            List::History(user) => {
                if user.to_lowercase() == "all" {
                    list(self.storage.history())
                } else {
                    list(self.storage.history()
                            .iter()
                            .filter(|e| e.username.to_lowercase() == user))
                }
            }
        }

        Ok(())
    }

    fn process_punish(&mut self, p: Punish) -> Result<()> {
        let history_entry = {
            // Increment user fault count
            self.storage.increment_faults(&p.user);

            let user = self.storage.get_user_by_name(&p.user)
                .map(Clone::clone)
                .unwrap_or_else(|| {
                    println!("User not found. Please add user {} via 'add user' CLI subcommand.", p.user);
                    process::exit(1)
                });

            let coeff = self.storage.value("coefficient")
                .and_then(|x| x.parse::<f64>().ok())
                .unwrap_or(1.0);

            let severity = p.severity as f64 * coeff;

            let fault = p.fault;

            // Register an entered fault
            self.storage.add_fault(fault.clone());

            let mut punishment = rand::thread_rng().choose(
                &self.storage.punishments()
                    .map(Clone::clone)
                    .collect::<Vec<_>>()
            ).unwrap().clone();

            let amount_coeff = severity * user.faults_current as f64;
            punishment.amount = (punishment.amount as f64 * amount_coeff).ceil() as u64;

            HistoryEntry {
                datetime: Local::now(),
                username: user.name.clone(),
                fault: fault.clone(),
                punishment: punishment
            }
        };

        self.show_punishment(&history_entry)?;
        self.storage.log(history_entry);
        self.storage.store_default_path()
    }

    fn show_punishment(&self, he: &HistoryEntry) -> Result<()> {
        let punishments = self.storage.punishments()
            .map(Clone::clone)
            .collect::<Vec<_>>();

        let mut random = rand::thread_rng();

        let rotations: i32 = random.gen_range(10, 100);
        let start_delay_ms: i32 = 20;
        let finish_delay_ms: i32 = 500;

        let delay_step = (finish_delay_ms - start_delay_ms) as f64 / rotations as f64;

        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        let mut delay_current = start_delay_ms as f64;

        writeln!(stdout, "Choosing punishment for {}", he.username)?;

        for _ in 0..rotations {
            let punishment = random.choose(&punishments).unwrap();

            write!(stdout, "{}                         \r", punishment.name)?;
            stdout.flush()?;

            let delay = delay_current.floor() as u64;
            let delay = Duration::from_millis(delay);
            delay_current += delay_step;
            thread::sleep(delay);
        }

        writeln!(stdout, "{}\n", he.punishment.name)?;
        writeln!(stdout, "{} is punished for '{}' with {} {}",
            he.username, he.fault, he.punishment.amount, he.punishment.name
        )?;
        writeln!(stdout, "")?;
        writeln!(stdout, "Be a good cat next time!")?;

        stdout.flush()?;

        Ok(())
    }

    fn process_set(&mut self, set: Set) -> Result<()> {
        match set {
            Set::Coefficient(coeff) => self.storage.set_value("coefficient", &coeff.to_string())
        }
        self.storage.store_default_path()
    }
}

use std::fmt::Display;

fn list<I: IntoIterator<Item=T>, T: Display>(iter: I) {
    for (i, d) in iter.into_iter().enumerate() {
        println!("{}: {}", i, d);
    }
}