//this functions as a very simplistic database
use std::collections::HashMap;
use std::fs;
use std::fmt;
pub enum Action{
    Read{key: u32},
    Write{key: u32, value: u32},
    Delete{key: u32},
}
impl fmt::Debug for Action{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Read {key} => write!(f, "Read({})", key),
            Action::Write {key, value} => write!(f, "Write({},{})", key, value),
            Action::Delete {key} => write!(f, "Delete({})", key)
        }
    }
}
pub struct KV{
    log: Vec<(Action, Option<u32>)>,
    path: String,
    log_path: String,
    map: HashMap<u32, u32>
}
impl KV{
    pub fn new(path: &str, log_path: &str) -> Self {
        KV {
            log: Vec::<(Action, Option<u32>)>::new(),
            path: String::from(path),
            map: HashMap::new(),
            log_path: String::from(log_path)
        }
    }
    //will save in a session file
    pub fn act (&mut self, action: Action) -> Option<u32> {
        match action {
            Action::Read {key} => { 
                let res = self.map.get(&key).copied();
                let tup = (action, res);
                self.log.push(tup);
                res
            },
            Action::Write {key, value} => {
                let res = self.map.insert(key, value);
                let tup = (action, res);
                self.log.push(tup);
                res
            }
            Action::Delete {key} => {
                let res = self.map.remove(&key);
                let tup = (action, res);
                self.log.push(tup);
                res
            }
        }
    }
    pub fn load(&mut self) {
        if let Ok(data) = fs::read_to_string(&self.path) {
            self.map = data.trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .filter_map(|entry| {
                    let mut parts = entry.split(':');
                    if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                        if let (Ok(key), Ok(value)) = (key.parse::<u32>(), value.parse::<u32>()) {
                            return Some((key, value));
                        }
                    }
                    None
                })
            .collect();
        }
    }
    pub fn save(&self) {
        let data:String = self.map.iter()
            .map(|(key, value)| format!("{}:{}", key, value))
            .collect::<Vec<String>>()
            .join(",");
        fs::write(&self.path, format!("{{{}}}", data)).expect("DB Write Failed");
        let log:String = self.log.iter()
            .map(|(act, res)| format!("{:?}:{:?}", act, res))
            .collect::<Vec<String>>()
            .join(",");
        fs::write(&self.log_path, format!("{{{}}}",log)).expect("Log Write Failed");
    }
}
