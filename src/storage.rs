use std::collections::{HashMap, BTreeSet};
use json;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::fs;

use APP_INFO;
use app_dirs::AppDataType;
use app_dirs::get_app_root;

use types::*;
use error::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Storage {
    users: BTreeSet<User>,
    faults: BTreeSet<Fault>,
    punishments: BTreeSet<Punishment>,
    values: HashMap<String, String>,
    history: Vec<HistoryEntry>,
}

impl Storage {
    pub fn default_path() -> Result<PathBuf> {
        let path = get_app_root(AppDataType::UserConfig, &APP_INFO)?;
        fs::create_dir(&path).ok();
        let path = path.join("config.json");
        Ok(path)
    }

    pub fn load_default_path() -> Result<Self> {
        Self::load(Self::default_path()?)
    }

    pub fn store_default_path(&self) -> Result<()> {
        self.store(Self::default_path()?)
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        Ok(
            json::from_reader(file)?
        )
    }

    pub fn store<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;

        let serialized = json::to_string_pretty(self)?;

        file.write_all(serialized.as_bytes())?;

        Ok(())
    }

    pub fn users(&self) -> impl Iterator<Item=&User> {
        self.users.iter()
    }

    pub fn get_user_by_name(&self, name: &str) -> Option<&User> {
        self.users.get(&User::new(name.to_string()))
    }

    pub fn add_user(&mut self, u: User) {
        println!("Added a new user {}", u);
        self.users.insert(u);
    }

    pub fn increment_faults(&mut self, name: &str) {
        let mut user = self.users.take(&User::new(name.to_owned())).unwrap();
        user.increment_faults();
        self.users.insert(user);
    }

    pub fn faults(&self) -> impl Iterator<Item=&Fault> {
        self.faults.iter()
    }

    pub fn add_fault(&mut self, f: Fault) {
        if !self.faults.contains(&f) {
            println!("Added a new fault {}", f);
            self.faults.insert(f);
        }
    }

    pub fn punishments(&self) -> impl Iterator<Item=&Punishment> {
        self.punishments.iter()
    }

    pub fn add_punishment(&mut self, p: Punishment) {
        println!("Added a new punishment {}", p);
        self.punishments.insert(p);
    }

    pub fn value(&self, name: &str) -> Option<&String> {
        self.values.get(name)
    }

    pub fn set_value<T: ToString>(&mut self, name: T, value: T) {
        let name = name.to_string();
        let value = value.to_string();
        println!("Set value {} as {}", name, value);
        self.values.insert(name, value);
    }

    pub fn history(&self) -> &[HistoryEntry] {
        self.history.as_ref()
    }

    pub fn log(&mut self, entry: HistoryEntry) {
        self.history.push(entry)
    }
}