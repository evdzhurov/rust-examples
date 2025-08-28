use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};
use regex::Regex;

fn collect_lines(input: &str) -> Vec<String> {
    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        reader.lines().map(|line| line.unwrap()).collect()
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        reader.lines().map(|line| line.unwrap()).collect()
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let ctx_lines = 2;

    let input = args.value_of("input").unwrap_or("-");
    let lines = collect_lines(input);

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in lines.iter().enumerate() {
        let contains_substring = re.find(&line);
        if contains_substring.is_some() {
            tags.push(i);
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in lines.iter().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let local_ctx = (i, line.clone());
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
