use std::env;

use getopts::Options;

use rust_life::{play, setup_screen};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "c",
        "columns",
        "number of columns of board (default: full screen)",
        "columns",
    );
    opts.optopt(
        "r",
        "rows",
        "number of rows of board (default: full screen)",
        "rows",
    );

    opts.optopt("i", "iterations", "# of iterations", "iterations");
    opts.optflag("H", "hacker", "seed with the hacker emblem");
    opts.optopt(
        "w",
        "wait",
        "milliseconds to sleep between iterations (default: 500)",
        "iterations",
    );
    opts.optflag("h", "help", "display this help message");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let columns: Option<i32> = match matches.opt_get("c") {
        Ok(c) => c,
        Err(_) => {
            eprintln!("invalid # of columns; using default");
            None
        }
    };

    let rows: Option<i32> = match matches.opt_get("r") {
        Ok(r) => r,
        Err(_) => {
            eprintln!("invalid # of rows; using default");
            None
        }
    };

    let iterations: Option<u32> = match matches.opt_get("i") {
        Ok(i) => i,
        Err(_) => {
            eprintln!("invalid # of iterations; using default");
            None
        }
    };

    let hacker = matches.opt_present("H");

    let wait: Option<u64> = match matches.opt_get("w") {
        Ok(w) => w,
        Err(_) => {
            eprintln!("invalid wait time between iterations; using default");
            None
        }
    };

    let mut screen = setup_screen();

    play(&mut screen, columns, rows, iterations, hacker, wait);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);

    println!("{}", opts.usage(&brief));
}
