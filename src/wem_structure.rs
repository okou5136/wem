#[derive(Clone)]
pub enum Actions {
    DIR,
    FILE,
}

pub struct ExecInfo {
    pub action: Actions,
    pub name: String,
    pub location: String,
    pub pretext: String,
}

impl ExecInfo {
    pub fn new() -> ExecInfo {
        ExecInfo {
            action: Actions::DIR,
            name: String::new(),
            location: String::new(),
            pretext: String::new(),

        }
    }

    pub fn from_pre(&self, prefrom: Vec<String>) -> ExecInfo {
        ExecInfo {
            action: self.action.clone(),
            name: self.name.clone(),
            location: self.location.clone(),
            pretext: prefrom.join("").to_string(),
        }
    }
}

pub struct VarInfo {
    pub name: String,
    pub var: String,
}

impl VarInfo {
    pub fn new() -> VarInfo {
        VarInfo {
            name: String::new(),
            var: String::new(),
        }
    }
}
