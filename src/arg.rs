use clap::{
    Parser,
    Args,
    Subcommand
};


#[derive(Debug, Parser)]
#[clap (about="Wirease improved",
long_about="A program that automates the process of generating files",
author="D.A.",
version="1.0.2 HIV")]
pub struct Arguments {
    #[clap(subcommand)]
    pub act: Move,

    #[clap(short='c', long="config")]
    pub conf_path: Option<String>
}

#[derive(Debug,Subcommand)]
pub enum Move {
    ///load wem script and generate files
    Make(MakeCommand),

    ///list up all available wem scripts
    List(ListCommand),

    ///read filesystem recursively and generate a wem script
    Read(ReadCommand),

    ///modify a selected wem script
    Mod(ModCommand),
}

#[derive(Debug, Args)]
pub struct MakeCommand {
    ///name of the wem script you want to load
    pub reference_name: String,

    ///name of your project, will be value for %NAME%
    pub project_name: Option<String>,

    ///path to the the file in which wem script is stored(default value is stored in config file)
    #[clap(short='s', long="source")]
    pub reference_source: Option<String>,

    ///determine how %DATE% is formatted (this value will overwrite the value in config)
    #[clap(short='t', long="time-format")]
    pub time_format: Option<String>,

    ///path where you want to generate the files for
    #[clap(short='o', long="output")]
    pub output: Option<String>,
}

#[derive(Debug,Args)]
pub struct ListCommand {
    ///path to the the file in which wem script is stored(default value is stored in config file)
    pub reference_path: Option<String>,
}

#[derive(Debug, Args)]
pub struct ReadCommand {
    pub ref_name: String,

    pub output: Option<String>,

    #[clap(short='s', long="source")]
    pub ref_src: Option<String>,

    #[clap(short='d', long="description")]
    pub desc: Option<String>,

}

#[derive(Debug, Args)]
pub struct ModCommand {
    pub ref_name: String,
    
    #[clap(short='e', long="editor")]
    pub editor: Option<String>,

    #[clap(short='p', long="path")]
    pub ref_path: Option<String>,
}
