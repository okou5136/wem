use clap::{
    Parser,
    Args,
    Subcommand
};


#[derive(Debug, Parser)]
pub struct Arguments {
    #[clap(subcommand)]
    pub act: Move,

    #[clap(short='m', long="mode")]
    pub mode: Option<String>,

    #[clap(short='c', long="config")]
    pub conf_path: Option<String>
}

#[derive(Debug,Subcommand)]
pub enum Move {
    ///load wem script and generate files
    Make(MakeCommand),

    ///list up all available wem scripts
    List(ListCommand),
}

#[derive(Debug, Args)]
pub struct MakeCommand {
    ///name of the wem script you want to load
    pub reference_name: String,

    ///name of your project, will be value for %NAME%
    pub project_name: String,

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
