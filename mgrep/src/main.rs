use std::env;
use::kmp;
use colored::Colorize;

fn main() {
    let rargs: Vec<String>  = env::args().collect::<Vec<_>>();
    let args = &rargs[1..];
    let mut input = String::new();
    if args.len() == 1 {
        let mut stdin = std::io::stdin().lines();
        while let Some(line) = stdin.next() {
            input.push_str(&line.expect("Failed to read line"));
            input.push('\n');
        }
    } else if args.len() == 2 {
        let mut file = std::fs::File::open(&args[1]).expect("Failed to open file");
        std::io::Read::read_to_string(&mut file, &mut input).expect("Failed to read file");
    } else {
        println!("Usage: {} <query> [file]", rargs[0]);
        return;
    }

    if search(&args[0], &input) == 0 {
        std::process::exit(1)
    }
}

fn search(query: &str, contents: &str) -> usize {
    contents.lines()
        .map(|line| deep_search(query, line))
        .filter(|a| a.1)
        .map(|a| {
            a.0.into_iter().for_each(|elem| {
                match elem {
                    LineElem::Text(s) => {
                        print!("{}", s);
                    },
                    LineElem::Match(s) => {
                        print!("{}", s.bold().red());
                    },
                }
            });
            println!()
        }).count()
}

#[derive(Debug)]
enum LineElem {
    Text(String),
    Match(String),
}


fn deep_search(query: &str, line: &str) -> (Vec<LineElem>, bool) {
    let queryu = query.as_bytes();
    let lineu = line.as_bytes();
    let res = kmp::kmp_match(queryu, lineu);
    let mut elems = Vec::new();
    let contains = res.is_empty();
    if contains {
        elems.push(LineElem::Text(line.to_string()));
    } else {
        let mut last = 0;
        let mut flag = 0;
        if res[0] as i32 != 0 {
            flag = 1;
        }
        for pos in res {
            if flag == 1 {
                elems.push(LineElem::Text(line[last..pos].to_string()))
            }
            flag = 1;
            elems.push(LineElem::Match(line[pos..pos+query.len()].to_string()));
            last = pos + query.len();
        }
        if last < line.len() {
            elems.push(LineElem::Text(line[last..].to_string()));
        }
    }
    (elems, !contains)
}
