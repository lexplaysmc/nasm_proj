// #![feature(specialization)]
// #![allow(incomplete_features)]

pub mod errors;
pub mod config;
pub mod build;

use config::parse_config;
use errors::{Expect, error};
use build::build;

use colored::Color;
use colored::Colorize;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len()==0 {
        println!("{}\n\
                {1} new {2} <asm|c|cpp|c++> - makes a new project in the directory {3} (language is optional and defaults to asm)\n\
                {1} build - builds the project in the cwd into an exe\n\
                {1} run - builds and runs the project in the cwd\n\
                {1} clean - cleans up the .\\build directory and the built exe\n\
                {1} cleanlib - cleans up the compiled libraries. use this if you changed a lib file\n\n\
                {4}\n\
                {1} new:\n\
                \t{5} - don't initialize this project with git",
            "Commands:".color(Color::Yellow), "nasm_proj".color(Color::BrightGreen), "<name>".color(Color::BrightBlue), ".\\<name>".color(Color::BrightBlue), "Arguments:".color(Color::Yellow), "--nvcs".color(Color::BrightBlue));
        return;
    }
    if args[0]=="new" || args[0]=="n" {
        if args.len()<2 {
            println!("{} nasm_proj new {}", "Syntax:".color(Color::Yellow), "<project name>".color(Color::Yellow));
            return;
        }
        if fs::read_dir(format!(".\\{}", args[1])).is_ok_and(|x| x.peekable().peek().is_some()) {
            error("project already exists");
        }
        let mut cpp = false;
        fs::create_dir_all(format!(".\\{}\\src", args[1])).expect_np("couldn't create directory tree");
        if args.len()<3 || args[2].to_ascii_lowercase()=="asm" {
            File::create(format!(".\\{}\\src\\main.asm", args[1])).expect_np("couldn't make main.asm").write_all(include_bytes!("default\\main.asm")).expect_np("couldn't write to main.asm");
        } else if args[2].to_ascii_lowercase()=="c" {
            File::create(format!(".\\{}\\src\\main.c", args[1])).expect_np("couldn't make main.c").write_all(include_bytes!("default\\main.c")).expect_np("couldn't write to main.c");
        } else if args[2].to_ascii_lowercase()=="c++" || args[2].to_ascii_lowercase()=="cpp" {
            File::create(format!(".\\{}\\src\\main.cpp", args[1])).expect_np("couldn't make main.cpp").write_all(include_bytes!("default\\main.cpp")).expect_np("couldn't write to main.cpp");
            cpp = true;
        }
        File::create(format!(".\\{}\\nasm_proj.json", args[1])).expect_np("couldn't make project config").write_all(include_str!("nasm_proj.json").replace("$name", &args[1]).replace("$++", if cpp {"++"} else {""}).as_bytes()).expect_np("couldn't write to project config");
        if !args.contains(&"--nvcs".to_string()) {
            env::set_current_dir(format!(".\\{}", args[1])).expect_np("couldn't change cwd");
            File::create(".\\.gitignore").expect_np("couldn't make gitignore").write_all(include_bytes!("default\\gitignore")).expect_np("couldn't write to gitignore");
            run_cmd(execute::command!("git init"));
            run_cmd(execute::command!("git add ."))
        }
    } else if args[0]=="build" || args[0]=="b" {
        build(&parse_config());
    } else if args[0]=="run" || args[0]=="r" {
        let conf = parse_config();
        build(&conf);
        let mut p = env::current_dir().expect_np("cwd error");
        p.push(Path::new(&conf.name));
        let mut c = Command::new(p.as_os_str());
        println!("{}", "running".color(Color::BrightCyan));
        c.spawn().expect_np("command failed").wait().expect_np("couldn't wait for command");
    } else if args[0]=="clean" || args[0]=="c" {
        let name = parse_config().name;
        if fs::read_dir(".\\build").is_ok() {
            fs::remove_dir_all(".\\build").expect_np("couldn't delete build");
        }
        if File::open(format!(".\\{name}.exe")).is_ok() {
            fs::remove_file(format!(".\\{name}.exe")).expect_np("couldn't delete exe");
        }
        println!("{}", "done cleaning".color(Color::Yellow));
    } else if args[0]=="cleanlib" || args[0]=="cl" {
        let mut p = env::current_exe().expect_np("couldn't get exe");
        p.pop();
        p.pop();
        env::set_current_dir(p).expect_np("couldn't cd");
        if fs::read_dir(".\\build").is_ok() {
            fs::remove_dir_all(".\\build").expect_np("couldn't delete build");
        }
        println!("{}", "done cleaning".color(Color::Yellow));
    }
}

fn run_cmd(mut c: Command) {
    if !c.spawn().expect_np("command failed").wait().expect_np("couldn't wait for command").success() {
        error("command failed: exit code nonzero");
    }
}