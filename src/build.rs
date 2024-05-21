use std::{fs::{self, File}, env};

use colored::{Color, Colorize};

use crate::{config::{self, Config}, errors::Expect, run_cmd};

pub fn build(c: &Config) {
    fs::create_dir_all(".\\build").expect_np("couldn't make build directory");

    let mut lib_loc = env::current_exe().expect_np("couldn't find current exe");
    lib_loc.pop();
    lib_loc.pop();
    lib_loc.push("lib");

    let name = &c.name;
    let build = &c.build;
    let link = &c.link;

    let mut objs = build_lib(c);
    println!("{}", "building".color(Color::Yellow));
    for file in fs::read_dir(".\\src").expect_np("missing src directory").map(|x| x.expect_np("file error")) {
        let fname = file.file_name().into_string().map_err(|x| x.to_string_lossy().into_owned()).expect_np("non unicode filename");
        let (n, t) = fname.rsplit_once('.').or(Some((&fname, ""))).unwrap();
        let t = format!(".{}", t);
        objs.extend(format!("build\\\\{n}{t}.o ").chars());

        if let (Ok(f), Ok(s)) = (File::open(&format!("build\\{n}{t}.o")), File::open(&format!("src\\{n}{t}"))) {
            if let (Ok(obj), Ok(src)) = (f.metadata().expect_np("couldn't get file metadata").modified(), s.metadata().expect_np("couldn't get file metadata").modified()) {
                if src<obj {
                    continue;
                }
            }
        }

        if let Some(b) = build.get(&t) {
            let c = b.replace("$build", &format!("build\\{n}{t}"));
            let c = c.replace("$src", &format!("src\\{n}{t}"));
            let c = c.replace("$lib", &lib_loc.as_mut_os_string().clone().into_string().map_err(|_| "".to_string()).expect_np("non unicode path"));
            let c = c.replace('\\', "\\\\");
            println!("{} {}", "running".color(Color::BrightCyan), c.color(Color::BrightBlue));
            let c = execute::command(c);
            run_cmd(c);
        }
    }

    
    println!("{}", "linking".color(Color::Yellow));
    let objs = objs.trim_end();
    let c = link.replace("$proj", &name);
    let c = c.replace("$obj", objs);
    println!("{} {}", "running".color(Color::BrightCyan), c.color(Color::BrightGreen));
    let c = execute::command(c);
    run_cmd(c);
    println!("{}", "done building".color(Color::Yellow));
}

fn build_lib(c: &Config) -> String {
    if c.lib.is_empty() {
        return String::new();
    }
    println!("{}", "building libs".color(Color::Yellow));
    let mut objs = String::new();
    let mut nasm_dir = env::current_exe().expect_np("couldn't find current exe");
    nasm_dir.pop();
    nasm_dir.pop();
    let cwd = env::current_dir().expect_np("couldn't find cwd");
    env::set_current_dir(&nasm_dir).expect_np("couldn't cd");
    fs::create_dir_all(".\\build").expect_np("couldn't make build");
    let nc = config::parse_config();
    nasm_dir.push("build");
    for x in &c.lib {
        nasm_dir.push(format!("{x}.o"));
        if let Ok(_) = File::open(&nasm_dir) {
            objs.push_str(&nasm_dir.as_mut_os_str().to_owned().into_string().map_err(|_| "".to_string()).expect_np("non unicode path").replace("\\", "\\\\"));
            objs.push(' ');
            nasm_dir.pop();
            continue;
        }
        nasm_dir.pop();
        nasm_dir.pop();

        nasm_dir.push("lib");
        nasm_dir.push(x);
        File::open(&nasm_dir).expect_np("missing file in lib");
        
        let (n, t) = x.rsplit_once('.').or(Some((&x, ""))).unwrap();
        
        if let Some(b) = nc.build.get(&format!(".{}", t)) {
            let c = b.replace("$build", &format!("build\\\\{n}.{t}"));
            let c = c.replace("$src", &format!("lib\\\\{n}.{t}"));
            let c = execute::command(c);
            run_cmd(c)
        }
        nasm_dir.pop();
        nasm_dir.pop();
        nasm_dir.push("build");
        nasm_dir.push(format!("{x}.o"));
        objs.push_str(&nasm_dir.as_mut_os_str().to_owned().into_string().map_err(|_| "".to_string()).expect_np("non unicode path").replace('\\', "\\\\"));
        objs.push(' ');
        nasm_dir.pop();
    }
    env::set_current_dir(cwd).expect_np("couldn't cd");
    println!("{}", "done building libs".color(Color::Yellow));
    objs
}