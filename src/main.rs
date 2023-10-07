mod wem_structure;
mod search_org;

use wem_structure::*;

use anyhow::Context;
use chrono;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
use std::collections::*;
use std::time::Instant;
use clap::Parser;


#[derive(Parser)]
struct Arguments {
    reference_name: String,

    project_name: String,

    #[clap(short='s', long="source", default_value="/home/normie/documents/program/rs/workinprogress/wem/assets")]

    reference_source: String,

    #[clap(short='t', long="time-format", default_value="%Y-%m-%d")]
    time_format: String,

    #[clap(short='o', long="output")]
    output: Option<String>,

    #[clap(short='m', long="mode")]
    mode: Option<String>,
}

//read lexed lines and extract the variable datas
fn hash_maker(original: &Vec<String>) -> anyhow::Result<HashMap<String, String>> {
    let mut vars: HashMap<String, String> = HashMap::new();
    let mut varinfo = VarInfo::new();
    let mut i = 0usize;

    while i < original.len() {
        if original[i] == "def" {

            i += 1;
            if original[i] == ":" {

                i += 1;
                if original[i] == "{" {

                    i += 1;
                    while original[i] != "}" {
                        varinfo.name.push_str(&original[i]);

                        i += 1;
                        if original[i] != "=" {
                            return Err(anyhow::anyhow!("Syntax error occurred during parsing varibales:\n
                                                       varible name must be followed by \"=\" operator"));
                        }

                        i += 1;
                        if original[i] == "\"" {
                            i += 1;
                            while original[i] != "\"" {
                                varinfo.var.push_str(&format!("{}\n", original[i]));
                                i += 1;
                            }
                        } else {
                            varinfo.var.push_str(&original[i]);
                        }

                        vars.insert(varinfo.name, varinfo.var);
                        varinfo = VarInfo::new();

                        i += 1;
                    }
                } else {
                    varinfo.name.push_str(&original[i]);
                    i += 1;
                    if original[i] != "=" {
                        return Err(anyhow::anyhow!("Syntax error occurred during parsing varibales:\n
                                                       varible name must be followed by \"=\" operator"));
                    }

                    i += 1;
                    if original[i] == "\"" {
                        i += 1;
                        while original[i] != "\"" {
                            varinfo.var.push_str(&format!("{}\n", original[i]));
                            i += 1;
                        }
                    } else {
                        varinfo.var.push_str(&original[i]);
                    }

                    vars.insert(varinfo.name, varinfo.var);
                    varinfo = VarInfo::new();
                }
            } else {
                return Err(anyhow::anyhow!("Syntax error occurred during parsing variables:
                                           \nthe command \"def\" could not detect \":\""));               
            }

        }
        i += 1;
    }

    Ok(vars)
}

//parse the lexed lines and substitute the strings
fn val_parser(original: &Vec<String>, val_hash: &HashMap<String, String>) -> anyhow::Result<Vec<String>> {
    let arg = Arguments::parse();
    let mut result: Vec<String> = Vec::new();
    let mut line = String::new();
    let mut i = 0usize;
    let mut temp_card = String::new();

    for string in original {
        let substring: Vec<char> = string.chars().collect();
        while i < substring.len() {
            if substring[i] == '%' {
                i += 1;
                while substring[i] != '%' {
                    temp_card.push(substring[i]);
                    i += 1;
                }
                if temp_card == "NAME".to_string() {
                    line.push_str(arg.project_name.as_str());
                    temp_card = String::new();
                } 
                else if temp_card == "DATE".to_string() {
                    line.push_str(format!("{}", chrono::Local::now().format(&arg.time_format)).as_str());
                    temp_card = String::new();
                }
                else if temp_card == "DQ".to_string() {
                    line.push('"');
                    temp_card = String::new();

                } else {
                    if let Some(var) = val_hash.get(&temp_card) {
                        line.push_str(var);
                    } else {
                        return Err(anyhow::anyhow!("variable name not declared"));
                    }
                    temp_card = String::new();
                }
            } else {
                line.push(substring[i]);
            }
            i += 1;
        }
        i = 0;
        result.push(line.clone());
        line = String::new();
    }

    Ok(result)
}

fn parser(original: Vec<String>, strt_path: Option<String>) -> anyhow::Result<Vec<ExecInfo>> {
    let mut i = 0usize;
    let mut indent = 0usize;
    let mut prexec = ExecInfo::new();
    let mut res: Vec<ExecInfo> = Vec::new();
    let mut path: Vec<String> = match strt_path {
        Some(x) => vec![format!("{}", x)],
        None => vec![format!("{}", env::current_dir()?.display())]
    };
    while i < original.len() {
        if original[i] == "dir" {
            i += 1;

            prexec.location = path.join("/");
            prexec.action = Actions::DIR;
            if original[i] == ":" {
                i += 1;
                prexec.name.push_str(&original[i]);

                if original[i + 1] == "{" {
                    i += 1;
                    indent += 1;
                    path.push(prexec.name.clone());        
                }
            } else {
                return Err(anyhow::anyhow!("\"dir\" should be followed by ':'"));
            }
            res.push(prexec);
            prexec = ExecInfo::new();
        }
        else if original[i] == "file" {
            i += 1;

            prexec.location = path.join("/");
            prexec.action = Actions::FILE;
            if original[i] == "(" {
                i += 1;
                loop {
                    if original[i] == "pre" {
                        i += 1;

                        if original[i] != ":" {
                            return Err(anyhow::anyhow!("pre subcommand should be followed by ':'"));
                        }
                        i += 1;

                        if original[i] == "\""{
                            i += 1;
                            while original[i] != "\"" {
                                prexec.pretext.push_str(&original[i]);
                                prexec.pretext.push('\n');
                                i += 1;
                            }
                        } else {
                            prexec.pretext.push_str(&original[i]);
                        }
                    } else {
                        return Err(anyhow::anyhow!("file command does not support that subcommand"));
                    }
                    i += 1;

                    if original[i] == ")" {
                        i += 1;
                        break;
                    }
                    else if original[i] == "," {
                        i += 1;
                        continue;
                    } else {
                        return Err(anyhow::anyhow!("every subcommand must be followed by:\n
                                                   ')' to end the subcommand, or\n
                                                   ',' to continue the subcommand"));
                    }

                }
            }

            if original[i] == ":" {
                i += 1;
                if original[i] == "\"" {
                    return Err(anyhow::anyhow!("you cannot set long strings to file name"));
                }
                prexec.name.push_str(&original[i]);
            } else {
                return Err(anyhow::anyhow!("\"file\" should be followed by ':'"));
            }
            res.push(prexec);
            prexec =  ExecInfo::new();
        }
        else if original[i] == "}" && indent != 0 {
            path.pop();
            indent -= 1;
        }
        i += 1;
    }

    Ok(res)
}

//lex the lines and remove the excessive new lines
fn lexer(unlexed: Vec<String>) -> anyhow::Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    let mut cut_lex: Vec<String> = Vec::new();
    let mut dq_ind = false;
    let mut esc_ind = false;
    let mut res_pos: usize = 0;

    cut_lex.push(String::new());
    for line in unlexed {
        for each_char in line.chars() {
            if each_char == '"' && esc_ind == false {
                dq_ind = dq_ind ^ true;
            }

            if search_org::does_contain_vec(each_char.to_string(), vec!['"', ':', '{', '}', '(', ')', '=']) 
                && dq_ind == false 
                    && esc_ind == false {
                cut_lex.push(String::new());
                res_pos += 1;
                cut_lex[res_pos].push(each_char);
                cut_lex.push(String::new());
                res_pos += 1;
            }
            else if each_char == '"' && esc_ind == true {
                cut_lex[res_pos].pop();
                cut_lex[res_pos].push_str(&"%DQ%");
                esc_ind = false;
            }
            else if each_char == '"' && dq_ind == true && esc_ind == false {
                cut_lex.push(String::new());
                res_pos += 1;
                cut_lex[res_pos].push(each_char);
                cut_lex.push(String::new());
                res_pos += 1;
            } else {
                cut_lex[res_pos].push(each_char);
            }

            if each_char == '\\' {
                esc_ind = true;
            }
        }
        cut_lex.push(String::new());
        res_pos += 1;
    }

    res_pos = 0usize;
    dq_ind = false;
    result.push(String::new());
    for string in cut_lex {
        if string.contains('"') {
            dq_ind = dq_ind ^ true;
        }

        if string.trim() != "" && dq_ind == false {
            result.push(string.trim().to_string());

        }
        else if dq_ind == true {
            result.push(string);
        }
    }

    Ok(result)
}

fn exec(commands: Vec<ExecInfo>) -> anyhow::Result<()> {
    for command in commands {
        match command.action {
            Actions::DIR =>{
                fs::create_dir(format!("{}/{}",command.location, command.name))
                    .with_context(|| format!("faled to create directory"))?;
            },

            Actions::FILE => {
                let  file = File::create(format!("{}/{}", command.location, command.name))
                    .with_context(|| format!("failed to create new file"))?;
                if command.pretext != "" {
                    let mut filebuf = BufWriter::new(file);
                    filebuf.write(command.pretext.as_bytes())
                        .with_context(|| format!("failed to write pretext to the specified file"))?;
                }
            },
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    //arguments containing reference name, project name, and debug information
    let arg = Arguments::parse();
    let mut lex: Vec<String> = Vec::new();
    let now = Instant::now();
    let mode = match arg.mode {
        Some(x) => x,
        None => String::new(),
    };


    if let Ok(lines) = read_file(format!("{}/{}",arg.reference_source ,arg.reference_name)) {
        for line in lines {
            if let Ok(string_line) = line {
                lex.push(string_line);
            }
        }
    } else {
        return Err(anyhow::anyhow!("failed to read the path"));
    }

    println!("\nlines: ");
    for lex_line in &lex {
        println!("{}", lex_line);
    }

    let lexed = lexer(lex)?;

    let variables = hash_maker(&lexed)?;

    if mode.contains("debug") {
        println!("\nvariables: ");
        for (name, val) in &variables {
            println!("{}: {}", name, val);
        }
    }
    if mode.contains("debug") {
        println!("\nlexed lines: ");
        for lexed_line in &lexed {
            println!("{}", lexed_line);
        }
    }

    let lexed = val_parser(&lexed, &variables)?;

    if mode.contains("debug") {
        println!("\nlexed lines(variable substituted): ");
        for lexed_line in &lexed {
            println!("{}", lexed_line);
        }
    }

    let parsed = parser(lexed, arg.output)?;

    let parsed = parsed.iter()
        .map(|x| x.from_pre(
                if let Some(y) = val_parser(&vec![x.pretext.clone()], &variables).ok() {
                    y
                } else {
                    vec![String::from("Error pretext")]
                }
                ))
        .collect::<Vec<ExecInfo>>();

    if mode.contains("debug") {
        println!("\nexec info:");
        for content in &parsed {
            println!("action: {}\nname: {}\nloc: {}\npre: {}\n",
                     match &content.action {
                             Actions::DIR => String::from("Dir"),
                             Actions::FILE => String::from("File"),
                     }, 
                     content.name, 
                     content.location, 
                     content.pretext);
        }
    }

    if mode.contains("time") {
        let elapsed = now.elapsed();
        println!("elapsed time: {:.2?}", elapsed);
    }

    if !mode.contains("test") {
        exec(parsed)?;
    }
    Ok(())
}

fn read_file<P>(file_path: P) -> anyhow::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>
{
    let file =  File::open(file_path).with_context(|| format!("failed to open the file"))?;
    Ok(io::BufReader::new(file).lines())
}