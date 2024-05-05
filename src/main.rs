mod structure;
mod general_func;
mod arg;

use arg::*;
use structure::*;

use walkdir::WalkDir;
use serde_yaml::{ self };
use anyhow::Context;
use chrono;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::collections::*;
use std::time::Instant;
use std::process::Command;
use clap::Parser;

//read lexed lines and extract the variable datas
fn hash_maker(original: &Vec<String>, outsidefile: &Vec<String>) -> anyhow::Result<HashMap<String, String>> {
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
    i = 0;
    while i < outsidefile.len() {
        if outsidefile[i] == "def" {

            i += 1;
            if outsidefile[i] == ":" {

                i += 1;
                if outsidefile[i] == "{" {

                    i += 1;
                    while outsidefile[i] != "}" {
                        varinfo.name.push_str(&outsidefile[i]);

                        i += 1;
                        if outsidefile[i] != "=" {
                            return Err(anyhow::anyhow!("Syntax error occurred during parsing varibales:\n
                                                       varible name must be followed by \"=\" operator"));
                        }

                        i += 1;
                        if outsidefile[i] == "\"" {
                            i += 1;
                            while outsidefile[i] != "\"" {
                                varinfo.var.push_str(&format!("{}\n", outsidefile[i]));
                                i += 1;
                            }
                        } else {
                            varinfo.var.push_str(&outsidefile[i]);
                        }

                        vars.insert(varinfo.name, varinfo.var);
                        varinfo = VarInfo::new();

                        i += 1;
                    }
                } else {
                    varinfo.name.push_str(&outsidefile[i]);
                    i += 1;
                    if outsidefile[i] != "=" {
                        return Err(anyhow::anyhow!("Syntax error occurred during parsing varibales:\n
                                                       varible name must be followed by \"=\" operator"));
                    }

                    i += 1;
                    if outsidefile[i] == "\"" {
                        i += 1;
                        while outsidefile[i] != "\"" {
                            varinfo.var.push_str(&format!("{}\n", outsidefile[i]));
                            i += 1;
                        }
                    } else {
                        varinfo.var.push_str(&outsidefile[i]);
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
fn val_parser(original: &Vec<String>, val_hash: &HashMap<String, String>, arg: &MakeArg) -> anyhow::Result<Vec<String>> {
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
                    line.push_str(arg.pro_name.as_str());
                    temp_card = String::new();
                } 
                else if temp_card == "DATE".to_string() {
                    line.push_str(format!("{}", chrono::Local::now().format(&arg.time_fmt)).as_str());
                    temp_card = String::new();
                }
                else if temp_card == "PERC".to_string() {
                    line.push('%');
                    temp_card = String::new();
                }
                else if temp_card == "DQ".to_string() {
                    line.push('"');
                    temp_card = String::new();
                }
                else if temp_card == "SRC".to_string() {
                    line.push_str(arg.ref_src.as_str());
                    temp_card = String::new();
                    println!("source!");
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
           // if original[i] == ":" {
           //     i += 1;
           //     prexec.name.push_str(&original[i]);

           //     if original[i + 1] == "{" {
           //         i += 1;
           //         indent += 1;
           //         path.push(prexec.name.clone());        
           //     }
           // } else {
           //     return Err(anyhow::anyhow!("\"dir\" should be followed by ':'"));
           // }
            if original[i] != ":" {
                return Err(anyhow::anyhow!("\"dir\" should be followed by ':'"));
            }

            i += 1;
            prexec.name.push_str(&original[i]);

            // see if the index is out of vector's bound.
            if i + 1 >= original.len() {
                res.push(prexec);
                prexec = ExecInfo::new();
                continue;
            }

            //leden ses om dett nastat innehallet ar "{".
            if original[i + 1] == "{" {
                i += 1;
                indent += 1;
                path.push(prexec.name.clone());
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

            if general_func::does_contain_vec(each_char.to_string(), vec!['"', ':', '{', '}', '(', ')', '=']) 
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
            else if each_char == '%' && esc_ind == true {
                cut_lex[res_pos].pop();
                cut_lex[res_pos].push_str(&"%PERC%");
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

    //res_pos = 0usize;
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

    println!("{:?}", &result);

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

fn desc_parser(lex: Vec<String>) -> anyhow::Result<String> {
    let mut desc = String::new();
    let mut i = 0usize;

    while lex.len() > i {
        if lex[i] == "desc".to_string() {
            i += 1;
            if lex[i] == ":" {
                i += 1;
                if lex[i] == "\"" {
                    i += 1;
                    while lex[i] != "\"".to_string() {
                        desc.push_str(&lex[i]);
                        desc.push('\n');
                        i += 1;
                    }
                    desc = desc.trim_end().to_string();
                    i += 1;
                } else {
                    desc.push_str(&lex[i]);
                    i += 1;
                }
            } 
            else if lex[i] == "(" {
                return Err(anyhow::anyhow!("command \"desc\" does't support subcommand"));
            } else {
                return Err(anyhow::anyhow!("invalid syntax:\ncommand \"desc\" needs\':\' after it"));
            }
        }
        i += 1;
    }

    return Ok(desc);
}

fn display_reference(ref_path: &String) -> anyhow::Result<()> {
    let path_crst = fs::read_dir(ref_path)?;
    let mut filenames: Vec<String> = Vec::new();
    let mut file_content: Vec<String> = Vec::new();
    let mut desc: Vec<String> = Vec::new();
    let mut spaces: usize = 0;
    let mut i: usize = 0;

    for path in path_crst {
        filenames.push(path.unwrap().path().display().to_string());
    }

    
    for file in &filenames {
        if let Ok(lines) = general_func::read_file(file) {
            for line in lines {
                if let Ok(code) = line {
                    file_content.push(code);
                }
            }
        } else {
            return Err(anyhow::anyhow!("failed to find the specified file"));
        }
        file_content = lexer(file_content)?;
        desc.push(desc_parser(file_content)?.trim().to_string());
        file_content = Vec::new();
    }

    for file in &filenames {
        if file.len() > spaces {
            spaces = file.len();
        }
    }

    println!("from {}", ref_path);
    for file in &filenames {
        print!("{}:", Path::new(file).file_name().and_then(|s| s.to_str())
               .with_context(|| format!("failed to obtain file name"))?);
        for _ in 0..(spaces + 2 - file.len()) {
            print!(" ");
        }

        println!("{}", desc[i]);
        i += 1;
    }
    Ok(())
}

fn try_open_files(pathvec: Vec<String>) -> anyhow::Result<File> {
    for path in pathvec {
        if let Ok(content) = File::open(path) {
            return Ok(content);
        }
    }

    return Err(anyhow::anyhow!("could not find any config files"));
}

fn dref_parser(lex: &Vec<String>, ref_src: &String) -> anyhow::Result<Vec<String>> {
    let mut i = 0usize;
    let mut result: Vec<String> = Vec::new();

    if !lex.contains(&"dref".to_string()) {
        return Ok(Vec::new());
    }

    while i < lex.len() {
        let mut onetime: Vec<String> = Vec::new();
        if lex[i] == "dref" {
            i += 1;
            if lex[i] != ":" {
                return Err(anyhow::anyhow!("dref needs ':' as a separator"));
            }

            i += 1;
            if lex[i] == "{" {
                i += 1;
                while lex[i] != "}" {
                    if let Ok(lines) = general_func::read_file(format!("{}/{}", ref_src, &lex[i])) {
                        for line in lines {
                            if let Ok(string_line) = line {
                                onetime.push(string_line);
                            }
                        }
                    } else {
                        return Err(anyhow::anyhow!("failed to read files in dref_parser"));
                    }
                    i += 1;
                    if lex[i] == "," {
                        i += 1;
                    }
                }
            } else {
                if let Ok(lines) = general_func::read_file(format!("{}/{}", ref_src, &lex[i])) {
                    for line in lines {
                        if let Ok(string_line) = line {
                            onetime.push(string_line);
                        }
                    }
                } 
            }
        }


        for ot_cont in lexer(onetime)? {
            result.push(ot_cont.to_string());
        }
        i += 1;
    }

   return Ok(result); 
}

fn del_filename(path: String) -> anyhow::Result<String> {
    let mut path: Vec<char> = path.chars().collect::<Vec<char>>();
    let _res: String = String::new();
    let mut i = path.len() - 1;

    while i > 0 {
        if path[i] == '/' {
            path.pop();
            return Ok(path.into_iter().collect::<String>());
        }
        path.pop();
        i -= 1;
    }

    Err(anyhow::anyhow!("could not get valid path"))
}

//read the information about files, which is defined as ExecInfo,
//and generate a wem script from it
fn create_wem(wem_script: &Vec<ExecInfo>, desc: Option<String>) -> anyhow::Result<String> {

    //quick note: how to indent
    //the first this came up to my mind was creating a hash map that stores the information of
    //indentations for each directory as a String type variable along with how many
    //indentation is needed.
    //
    //however, I realized that that would be too complicated and has a potential risk that slows
    //this program down.
    //
    //so, keeping the idea of storing information as a hash map, 
    //I decided to resolve how many indentation is needed for each directory
    //AND THEN create a wem script with indentation based on the hash map, each in the separate
    //loop.

    let mut description = String::new();
    let mut variables: HashMap<String, String> = HashMap::new();
    let mut varstr = String::new();
    let mut filesystem = String::new();
    let mut result = String::new();
    let mut indent_map: HashMap<String, i32> = HashMap::new();
    let _indent = 0i32;
    let mut varnum = 0usize;
    let mut before_location; 

    let base_slash = String::from(format!("{}", env::current_dir()?.display())).matches("/").count();

    for ws in wem_script {
        println!("{:?}", ws);
    }

    //when the contents of indent_map are parent's name and the indentation depth
    for (_i, component) in wem_script.iter().enumerate() {
        indent_map.insert(component.location.clone(), (component.location.matches("/").count() - base_slash).try_into().unwrap());
        before_location = component.location.clone();
    }

    if let Some(line) = desc {
        description.push_str(&format!("desc: \"{}\"\n", line));
    }

    println!("{:?}", indent_map);


    before_location = wem_script[0].location.clone();
    for (i, component) in wem_script.iter().enumerate() {

        if let Some(ind) = indent_map.get(&component.location) {
            let before_ind = indent_map.get(&before_location).with_context(|| format!("failed to get indentation data"))?;
            if ind < before_ind {
                for ii in 0..(before_ind - ind) {
                    for _ in 1..(before_ind - ii) {
                        filesystem.push_str("   ");
                    }
                    filesystem.push_str("}\n");
                }
            }
            for _ in 0..*ind {
                filesystem.push_str("   ");
            }
        }
        match component.action {
            Actions::DIR => {
                filesystem.push_str("dir");
                filesystem.push(':');
                filesystem.push_str(&component.name);
                if let Some(_) = indent_map.get(&vec![component.location.clone(), component.name.clone()].join("/")) {
                    filesystem.push_str(" {");
                }
            },

            Actions::FILE => {
                filesystem.push_str("file");
                if component.pretext != "".to_string() {
                    if component.pretext.as_str().lines().count() >= 1 {
                        variables.insert(format!("{}", varnum), component.pretext.clone());
                        filesystem.push_str(&format!("(pre: \"%{}%\")", varnum));
                        varnum += 1;
                    } else {
                        filesystem.push_str(&format!("(pre: \"{}\")", component.pretext));
                    }
                }
                filesystem.push(':');
                filesystem.push_str(&component.name);
            },
        }

        if i == wem_script.len() - 1 {
            filesystem.push('\n');
            if let Some(ind) = indent_map.get(&component.location) {
                for ii in 0..*ind {
                    for _ in 1..(ind - ii) {
                        filesystem.push_str("   ");
                    }
                    filesystem.push_str("}\n");
                }
            }
        }

        filesystem.push('\n');
        before_location = component.location.clone();
    }

    if !variables.is_empty() {
        varstr.push_str("def: {\n");
        for (num, var) in variables {
            varstr.push_str(&format!("{} = \"{}\"\n",num, var.replace("\"","\\\"").replace("%", "\\%") ));
        }
        varstr.push_str("}\n\n");
    }

    result.push_str(&description);
    result.push_str(&varstr);
    result.push_str(&filesystem);


    println!("{}", &result);
    Ok(result)
}

fn read_dir(name: String, strt_loc: Option<String>) -> anyhow::Result<Vec<ExecInfo>> {
    let mut wem_script: Vec<ExecInfo> = Vec::new();
    let mut i = 0usize;
    let path: Vec<String> = match strt_loc {
        Some(x) => vec![format!("{}", x)],
        None => vec![format!("{}", env::current_dir()?.display())],
    };

    wem_script.push(ExecInfo::new());

    for entry in WalkDir::new(format!("{}/{}", path.join("/"), name)).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            wem_script[i].action = Actions::FILE;
            if let Ok(lines) = general_func::read_file(entry.path()) {
                for line in lines {
                    if let Ok(string_line) = line {
                        if string_line != "".to_string() {
                            wem_script[i].pretext.push_str(&string_line);
                            wem_script[i].pretext.push('\n');
                        }
                    }
                }
            }
        }
        else if entry.path().is_dir() {
            wem_script[i].action = Actions::DIR;
        }

        if i == 0 {
            wem_script[i].location.push_str(&path.join("/"));
        } else {
            wem_script[i].location.push_str(&del_filename(entry.path().display().to_string())?);
        }

        wem_script[i].name.push_str(entry.file_name().to_str().with_context(|| format!("failed to convevrt entryr to stinrg"))?);
        wem_script.push(ExecInfo::new());
        i += 1;
    }

    wem_script.pop();

    println!("test purposed");
    for ws in &wem_script {
        println!("{}", ws.pretext);
    }

    Ok(wem_script)
}


// How this program works
//
// 1. Read config file
// 2. Determine the mode to use
//  2.1 make
//   2.1.1 If certain options are not given, read them from the config file
//   2.1.2 Read from the given wem file
//   2.1.3 Lex the file input
//   2.1.4 
//  2.2 

fn main() -> anyhow::Result<()> {
    //arguments containing reference name, project name, and debug information
    let arg = Arguments::parse();
    let home = env::var("HOME")?;
    let make_arg: MakeArg;
    let mut lex: Vec<String> = Vec::new();
    let _now = Instant::now();
    let conf_path: Vec<String> = match arg.conf_path {
        Some(x) => vec![x],
        None => vec!["/home/normie/documents/program/rs/completed/wem/config.yml".to_string(),
                          format!("{}/.wenconf.yml", home),
                          format!("{}/.config/wem/config.yml", home)],
    };

    let config:Config = serde_yaml::from_reader(try_open_files(conf_path)?)
        .with_context(|| format!("failed to read config file"))?;

    match arg.act {
        Move::Make(command) => {
            make_arg = MakeArg::from(command.reference_name.clone(),
            if let Some(pro_name) = command.project_name {
                pro_name
            } else {
                command.reference_name
            },
            if let Some(ref_path) = command.reference_source {
                ref_path
            } else {
                config.reference_path
            },
            if let Some(format) = command.time_format {
                format
            } else {
                config.time_format
            },
            command.output
            );

            if let Ok(lines) = general_func::read_file(format!("{}/{}", make_arg.ref_src, make_arg.ref_name)) {
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

            

            // ykw? i feel like hash_maker and dref part and all should go inside val_parser
            let lexed = lexer(lex)?;

            let dref_files: Vec<String> = dref_parser(&lexed, &make_arg.ref_src)?;

            let mut variables = hash_maker(&lexed, &dref_files)?;

            let lexed = val_parser(&lexed, &variables, &make_arg)?;

            let parsed = parser(lexed, make_arg.clone().output)?;

            //let parsed = parsed.iter()
            //    .map(|x| x.from_pre(
            //            if let Some(y) = val_parser(&vec![x.pretext.clone()], &variables, &make_arg).ok() {
            //                y
            //            } else {
            //                vec![String::from("Error pretext")]
            //            }
            //            ))
            //    .collect::<Vec<ExecInfo>>();

                exec(parsed)?;
        },

        Move::List(command) => {
            let reference = if let Some(ref_path) = command.reference_path {
                ref_path
            } else {
                config.reference_path
            };
            display_reference(&reference)?;
            return Ok(());
        },
        
        Move::Read(command) => {
            let  file = File::create(match command.output {
                Some(x) => format!("{}/{}", config.reference_path, x),
                None => format!("{}/{}", config.reference_path, &command.ref_name),
            })
            .with_context(|| format!("failed to create new file"))?;
            let dir_information = read_dir(command.ref_name, command.ref_src)?;
            let result =  create_wem(&dir_information, command.desc)?;


            let mut filebuf = BufWriter::new(file);
            filebuf.write(result.as_bytes())
                .with_context(|| format!("failed to write pretext to the specified file"))?;

            return Ok(());
        },
        Move::Mod(command) => {
            let editor = match command.editor {
                Some(x) => x,
                None => config.editor,
            };
            
            let path = match command.ref_path {
                Some(x) => x,
                None => format!("{}/{}", config.reference_path, command.ref_name),
            };

            match editor.as_str() {
                "nvim" => {
                    Command::new("nvim")
                        .arg(path)
                        .spawn()
                        .with_context(|| format!("failed to execute nvim"))?;
                },

                "vim" => {
                    Command::new("vim")
                        .arg(path)
                        .spawn()
                        .with_context(|| format!("failed to execute nvim"))?;
                },

                _ => {
                    return Err(anyhow::anyhow!("Error: unsupported text editor"));
                },
            }

            return Ok(())
        },
    }
    Ok(())
}

