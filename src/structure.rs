use serde::{ Serialize, Deserialize };

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

    //pub fn from(act_src: Actions,
    //            name_src: String,
    //            loc_src: String,
    //            pre_src: String) -> ExecInfo 
    //{
    //    ExecInfo {
    //        action: act_src,
    //        name: name_src, 
    //        location: loc_src,
    //        pretext: pre_src,
    //    }
    //}
    pub fn from_pre(&self, prefrom: Vec<String>) -> ExecInfo {
        ExecInfo {
            action: self.action.clone(),
            name: self.name.clone(),
            location: self.location.clone(),
            pretext: prefrom.join("").to_string(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MakeArg {
    pub ref_name: String,
    pub pro_name: String,
    pub ref_src: String,
    pub time_fmt: String,
    pub output: Option<String>,
}

impl MakeArg {
    // this function would come in handy later on.
    // but i don't use for now, therefore commented out.
    // ( sorry for my bad code :< )
//    pub fn new() -> MakeArg {
//        MakeArg {
//            ref_name: String::new(),
//            pro_name: String::new(),
//            ref_src:  String::new(),
//            time_fmt: String::new(),
//            output:   None,
//        }
//    }

    pub fn from(reference_name: String,
                project_name: String,
                reference_src:  String,
                time_format: String,
                out:   Option<String>,
                ) -> MakeArg 
    {
        MakeArg {
            ref_name: reference_name,
            pro_name: project_name,
            ref_src:  reference_src,
            time_fmt: time_format,
            output:   out,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub reference_path: String,
    pub time_format: String,
}

    // this function would come in handy later on.
    // but i don't use for now, therefore commented out.
    // ( sorry for my bad code :< )
//impl Config {
//    pub fn new() -> Config {
//        Config {
//            reference_path: String::new(),
//            time_format: String::new(),
//        }
//    }
//
//    pub fn from(reference_src:  String,
//                time_fmt: String,) -> Config
//    {
//        Config {
//            reference_path: reference_src,
//            time_format: time_fmt,
//        }
//    }
//}

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
