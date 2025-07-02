use alloc::format;
use alloc::string::{String, ToString};
use core::fmt;
use core::fmt::Debug;
use core::hash::{Hash, Hasher};

use hashbrown::HashSet;
use spin::{Lazy, Mutex};

#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    content: String,
}

static ENVIRONMENT: Lazy<Mutex<HashSet<Variable>>> = Lazy::new(|| Mutex::new(HashSet::new()));

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Variable {}

impl Hash for Variable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state); // nur nach name hashen
    }
}

impl Variable {
    pub fn new(name: &str, content: &str) -> Self {
        return Variable {
            name: name.to_string(),
            content: content.to_string(),
        };
    }

    pub fn get_content(&self) -> String {
        return self.content.to_string();
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{} : {}\"", self.name, self.content)
    }
}

pub fn env_contains(name: &str) -> bool {
    let variables = ENVIRONMENT.lock();
    let contains = variables.contains(&Variable::new(name, ""));
    return contains;
}

pub fn env_insert(name: &str, content: &str) {
    let mut variables = ENVIRONMENT.lock();
    variables.replace(Variable::new(name, content));
}

pub fn env_remove(name: &str) {
    let mut variables = ENVIRONMENT.lock();
    variables.remove(&Variable::new(name, ""));
}

pub fn env_get(name: &str) -> Option<String> {
    let variables = ENVIRONMENT.lock();
    let env_var_option = variables
        .get(&Variable::new(name, ""))
        .map(|var| var.content.clone());
    return env_var_option;
}

pub fn env_get_all() -> String {
    let variables = ENVIRONMENT.lock();
    // Leeren String erzeugen
    let mut content: String = String::new();
    // Anfangswert schreiben
    content.push_str("[\n");
    // Durch alle Variablen durchgehen
    for var in variables.iter() {

        content.push_str(format!("    - {:}\n",var).as_str());
    }
    // Schönes Ende als Abschluss
    content.push_str("]");
    return content;
}
