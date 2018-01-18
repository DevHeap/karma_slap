use std::cmp::{Ordering, PartialOrd, Ord, Eq, PartialEq};
use chrono::{DateTime, Local};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Mode {
    Add(Add),
    List(List),
    Set(Set),
    Punish(Punish)
}

#[derive(Debug, Clone)]
pub enum Add {
    Fault(Fault),
    Punishment(Punishment),
    User(User)
}

#[derive(Debug, Clone)]
pub enum List {
    Faults,
    Punishments,
    Users,
    History(Username)
}

#[derive(Debug, Clone)]
pub enum Set {
    Coefficient(f64)
}

#[derive(Debug, Clone)]
pub struct Punish {
    pub user: String,
    pub fault: String,
    pub severity: u16
}

pub type Fault = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Punishment {
    pub name: String,
    pub amount: u64,
}

impl PartialEq for Punishment {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Punishment {}

impl PartialOrd for Punishment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for Punishment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}


impl fmt::Display for Punishment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.name, self.amount)
    }
}

pub type Username = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub name: Username,
    // Overal faults
    pub faults_overall: u32,
    // Faults in the current period
    pub faults_current: u32,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for User {}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl User {
    pub fn new(name: String) -> Self {
        User {
            name: name,
            faults_overall: 0,
            faults_current: 0,
        }
    }

    pub fn increment_faults(&mut self) {
        self.faults_current += 1;
        self.faults_overall += 1;
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} faults | {} in the current period", self.name, self.faults_overall, self.faults_current)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub datetime: DateTime<Local>,
    pub username: String,
    pub fault: String,
    pub punishment: Punishment,
}

impl fmt::Display for HistoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {} | {} | {}", self.datetime, self.username, self.fault, self.punishment)
    }
}
