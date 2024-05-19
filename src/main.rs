use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use json;
use json::JsonValue;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len()==0 {
        println!("commands\n\
                nasm_proj new <name> - makes a new project in the directory .\\<name>\n\
                nasm_proj build - builds the project in the cwd into an exe\n\
                nasm_proj run - builds and runs the project in the cwd\n\
                nasm_proj clean - cleans up the .\\build directory and the built exe");
        return;
    }
    if args[0]=="new" || args[0]=="n" {
        if args.len()<2 {
            println!("syntax: nasm_proj new <project name>");
            return;
        }
        if fs::read_dir(format!(".\\{}", args[1])).is_ok_and(|x| x.peekable().peek().is_some()) {
            println!("project already exists");
            return;
        }
        fs::create_dir_all(format!(".\\{}\\src", args[1])).expect("couldn't create directory tree");
        File::create(format!(".\\{}\\src\\main.asm", args[1])).expect("couldn't make main.asm").write_all(include_bytes!("main.asm")).expect("couldn't write to main.asm");
        File::create(format!(".\\{}\\nasm_proj.json", args[1])).expect("couldn't make project config").write_all(include_str!("nasm_proj.json").replace("$name", &args[1]).as_bytes()).expect("couldn't write to project config");
    } else if args[0]=="build" || args[0]=="b" {
        build();
    } else if args[0]=="run" || args[0]=="r" {
        build();
        let mut p = env::current_dir().expect("cwd error");
        let name = get_proj_name(json_config_open().as_ref());
        p.push(Path::new(&name));
        let mut c = Command::new(p.as_os_str());
        c.spawn().unwrap_or_else(|x| {panic!("failed command: {}", x.to_string())}).wait().expect("couldn't wait for command");
    } else if (args[0]=="clean" || args[0]=="c") && fs::read_dir(".\\build").is_ok() {
        let name = get_proj_name(json_config_open().as_ref());
        if fs::read_dir(".\\build").is_ok() {
            fs::remove_dir_all(".\\build").expect("couldn't delete build");
        }
        if File::open(format!(".\\{name}.exe")).is_ok() {
            fs::remove_file(format!(".\\{name}.exe")).expect("couldn't delete exe");
        }
    }
}

fn run_cmd(mut c: Command) {
    if !c.spawn().unwrap_or_else(|x| {panic!("failed command: {}", x.to_string())}).wait().expect("couldn't wait for command").success() {
        panic!("command failed");
    }
}
fn build() {
    fs::create_dir_all(".\\build").expect("couldn't make build directory");

    let j = json::parse(&fs::read_to_string(".\\nasm_proj.json").expect("couldn't read project config")).expect("couldn't parse json");
    let name = get_proj_name(json::Result::Ok(&j).as_deref());
    if let JsonValue::Object(s) = j {
        let build;
        let link;

        if let (Some(JsonValue::Object(b)), Some(JsonValue::Short(l))) = (s.get("build"), s.get("link")) {
            build = b;
            link = l;
        } else {
            panic!("JSON has wrong format");
        }

        println!("building");
        let mut objs = String::new();
        for file in fs::read_dir(".\\src").expect("missing src directory") {
            if let Ok(file) = file {
                let fname = file.file_name().into_string().expect("non unicode filename");
                let (n, t) = fname.rsplit_once('.').or(Some((&fname, ""))).unwrap();
                let t = format!(".{}", t);
                objs.extend(format!("build\\\\{n}{t}.o ").chars());

                if let (Ok(f), Ok(s)) = (File::open(&format!("build\\{n}{t}.o")), File::open(&format!("src\\{n}{t}"))) {
                    if let (Ok(obj), Ok(src)) = (f.metadata().expect("couldn't get file metadata").modified(), s.metadata().expect("couldn't get file metadata").modified()) {
                        if src<obj {
                            continue;
                        }
                    }
                }

                if let Some(JsonValue::Short(b)) = build.get(&t) {
                    let c = b.replace("$build", &format!("build\\\\{n}{t}"));
                    let c = c.replace("$src", &format!("src\\\\{n}{t}"));
                    println!("running {c}");
                    let c = execute::command(c);
                    run_cmd(c);
                }

            } else {
                panic!("file error?");
            }
        }

        
        println!("linking");
        let objs = objs.trim_end();
        let c = link.replace("$proj", &name);
        let c = c.replace("$obj", objs);
        println!("running {c}");
        let c = execute::command(c);
        run_cmd(c);
    } else {
        panic!("JSON has wrong format");
    }
}
fn json_config_open() -> json::Result<JsonValue> {
    json::parse(&fs::read_to_string(".\\nasm_proj.json").expect("couldn't read project config"))
}
fn get_proj_name(js: Result<&JsonValue, &json::JsonError>) -> String {
    if let JsonValue::Object(s) = js.expect("couldn't parse json") {
        if let Some(JsonValue::Short(l)) = s.get("name") {
            return l.to_string();
        } else {
            panic!("JSON has wrong format");
        }
    } else {
        panic!("JSON has wrong format");
    }
}