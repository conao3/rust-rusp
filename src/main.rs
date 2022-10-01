extern crate getopts;

mod core;
mod types;

fn repl() -> Result<(), rustyline::error::ReadlineError> {
    let mut rl = rustyline::Editor::<()>::new()?;
    let history_file_path = "~/.rusp_history";
    _ = rl.load_history(history_file_path);

    loop {
        let line = rl.readline("rusp> ");
        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let res = core::rep(&line);

                match res {
                    Ok(res) => println!("{}", res),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };
    }
    rl.save_history(history_file_path)
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "verbose", "print extra information");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    _ = repl();
}
