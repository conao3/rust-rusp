use anyhow::Context;
use rusp::core;
use rusp::types;

fn repl() -> anyhow::Result<()> {
    let mut rl = rustyline::Editor::<()>::new()?;
    let xdg_dirs =
        xdg::BaseDirectories::with_prefix("rusp").context("Failed to get XDG directories")?;
    let history_file = xdg_dirs
        .place_config_file("history.txt")
        .context("Failed to get history file path")?;
    let history_file_path = history_file.as_path();
    _ = rl.load_history(history_file_path);

    let mut env = core::default_env();

    loop {
        let line = rl.readline("rusp> ");
        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let res = core::rep(&line, &mut env);

                match res {
                    Ok(res) => println!("{}", res),
                    Err(e) => {
                        if let Some(types::RuspErr::ReplEmptyError) = e.downcast_ref() {
                            break;
                        };
                        eprintln!("{:#?}", e);
                    }
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
    rl.save_history(history_file_path).with_context(|| {
        format!(
            "Failed save history file: {}",
            history_file_path.to_str().unwrap()
        )
    })?;
    Ok(())
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> anyhow::Result<()> {
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
        return Ok(());
    }

    repl()
}
