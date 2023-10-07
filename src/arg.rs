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
    Make(MakeCommand),
    List(ListCommand),
}

#[derive(Debug, Args)]
pub struct MakeCommand {
    pub reference_name: String,

    pub project_name: String,

    #[clap(short='s', long="source")]
    pub reference_source: Option<String>,

    #[clap(short='t', long="time-format")]
    pub time_format: Option<String>,

    #[clap(short='o', long="output")]
    pub output: Option<String>,
}

#[derive(Debug,Args)]
pub struct ListCommand {
    pub ref_dir: Option<String>,
}
