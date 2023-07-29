use std::collections::HashMap;

pub struct Config {
    pub(crate) id: u32,
    pub(crate) subtask_id: Option<u32>,
    pub(crate) values: HashMap<String, Value>,
}

#[derive(Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

impl Config {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            subtask_id: None,
            values: HashMap::new(),
        }
    }

    pub fn with_subtask(subtask_id: u32, id: u32) -> Self {
        Self {
            id,
            subtask_id: Some(subtask_id),
            values: HashMap::new(),
        }
    }

    pub fn insert<S, V>(mut self, key: S, value: V) -> Self
    where
        S: ToString,
        V: Into<Value>,
    {
        self.values.insert(key.to_string(), value.into());
        self
    }

    pub fn get<S: AsRef<str>>(&self, key: S) -> Value {
        self.values.get(key.as_ref()).cloned().unwrap()
    }

    pub fn generate_name<S1, S2>(&self, name: S1, extension: S2) -> String
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let name = name.as_ref();
        let extension = extension.as_ref();

        match self.subtask_id {
            Some(subtask_id) => format!("{}{}_{}.{}", name, subtask_id, self.id, extension),
            None => format!("{}{}.{}", name, self.id, extension),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn subtask_id(&self) -> Option<u32> {
        self.subtask_id
    }
}

impl Value {
    pub fn unwrap_integer(self) -> i64 {
        match self {
            Self::Integer(value) => value,
            _ => panic!("Error inner type"),
        }
    }

    pub fn unwrap_float(self) -> f64 {
        match self {
            Self::Float(value) => value,
            _ => panic!("Error inner type"),
        }
    }

    pub fn unwrap_string(self) -> String {
        match self {
            Self::String(value) => value,
            _ => panic!("Error inner type"),
        }
    }

    pub fn unwrap_bool(self) -> bool {
        match self {
            Self::Bool(value) => value,
            _ => panic!("Error inner type"),
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
