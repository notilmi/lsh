use std::io::Write;
use std::process::Command;
use std::io;

pub fn lsh_loop() {
    loop {
        print!("# ");

        io::stdout().flush().unwrap();
        let Some(line) = read_line() else {
            break;
        };

        let args = split_line(&line);
        let status = execute(&args);

        if !status {
            break;
        }
    }
}

fn execute(args: &Vec<&str>) -> bool {
    let Some((cmd, rest)) = args.split_first() else {
        return true;
    };

    match *cmd {
        "exit" => false,
        "cd" => lsh_cd(rest),
        _ => lsh_call(cmd, rest)
    }
}

fn read_line() -> Option<String> {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(0) => None,
        Ok(_) => Some(line),
        Err(e) => {
            eprintln!("lsh {}", e);
            None
        }
    }
}

fn split_line(lines: &str) -> Vec<&str> {
    lines.split_whitespace().collect()
}

fn lsh_call(cmd: &&str, rest: &[&str]) -> bool {
    match Command::new(cmd).args(rest).status() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("lsh: {}", e)
        }
    }

    true
}

fn lsh_cd(rest: &[&str]) -> bool {
    let dir = rest.first()
        .copied()
        .unwrap_or("/"); // Take / if not found

    if let (Err(e)) = std::env::set_current_dir(dir) {
        eprintln!("lsh: cd: {}", e)
    }

    true
}
