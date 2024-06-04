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
    pub act: Action,

    /// path to the config file
    #[clap(short='c', long="config", global = true)]
    pub conf_path: Option<String>
}

#[derive(Debug,Subcommand)]
pub enum Action {
    ///load wem script and generate files
    Make(MakeCommand),

    ///list up all available wem scripts
    List(ListCommand),

    ///read filesystem recursively and generate a wem script
    Read(ReadCommand),

    ///modify a selected wem script
    Mod(ModCommand),

    ///remove file 
    Rm(RmCommand),
}

#[derive(Debug, Args)]
pub struct MakeCommand {
    ///name of the wem script you want to load
    pub reference_name: String,

    ///name of your project, will be value for %NAME%
    pub project_name: Option<String>,

    ///path to the the file in which wem script is stored(default value is stored in config file)
    #[clap(short='s', long="source", global=true)]
    pub reference_source: Option<String>,

    ///determine how %DATE% is formatted (this value will overwrite the value in config)
    #[clap(short='t', long="time-format", global=true)]
    pub time_format: Option<String>,

    ///path where you want to generate the files for
    #[clap(short='o', long="output", global=true)]
    pub output: Option<String>,
}

#[derive(Debug,Args)]
pub struct ListCommand {
    ///path to the the file in which wem script is stored(default value is stored in config file)
    pub reference_path: Option<String>,
}

#[derive(Debug, Args)]
pub struct ReadCommand {

    ///name of the file/directory you want to read from
    pub ref_name: String,

    ///name of the output file
    pub output: Option<String>,

    ///path at which you want to save your wem file
    #[clap(short='s', long="source", global=true)]
    pub ref_src: Option<String>,

    ///description
    #[clap(short='d', long="description", global=true)]
    pub desc: Option<String>,

}

#[derive(Debug, Args)]
pub struct ModCommand {
    ///name of the file
    pub ref_name: String,
    
    ///name of the text editor to use
    #[clap(short='e', long="editor", global=true)]
    pub editor: Option<String>,

    ///path to the directory in which the file resides
    #[clap(short='s', long="source", global=true)]
    pub ref_path: Option<String>,
}

#[derive(Debug, Args)]
pub struct RmCommand {

    ///
    pub ref_name: String,
    
    #[clap(short='p', long="path", global=true)]
    pub ref_path: Option<String>,

    #[clap(short='y', long="yes-to-all", global=true)]
    pub confirm_every_question: bool,

    #[clap(long="not-keep-backup", global=true)]
    pub not_keep_backup: bool,
}
