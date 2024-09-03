use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

struct Query {
    typ: i32,
    x: i32,
    y: i32,
}

fn dynamic_array(n: usize, queries: &[Query]) -> Vec<i32> {
    let mut seq_list = vec![Vec::new(); n];
    let mut last_answer = 0;
    let mut answers = Vec::new();

    for query in queries {
        let idx = ((query.x ^ last_answer) as usize) % n;

        match query.typ {
            1 => {
                if let Some(seq) = seq_list.get_mut(idx) {
                    seq.push(query.y);
                }
            }
            2 => {
                if let Some(seq) = seq_list.get(idx) {
                    if let Some(&value) = seq.get((query.y as usize) % seq.len()) {
                        last_answer = value;
                        answers.push(last_answer);
                    }
                }
            }
            _ => {}
        }
    }

    answers
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let stdin_iterator = stdin.lock().lines();

    let output_path = env::var("OUTPUT_PATH").unwrap_or_else(|_| "output.txt".to_string());
    let mut fptr = File::create(&output_path).context("Failed to create output file")?;

    let mut lines = stdin_iterator.filter_map(Result::ok);
    if let Some(first_input) = lines.next() {
        let mut first_input_iter = first_input.split_whitespace();
        let n = first_input_iter
            .next()
            .context("Failed to parse n")?
            .parse()
            .context("Failed to parse n as usize")?;
        let q = first_input_iter
            .next()
            .context("Failed to parse q")?
            .parse()
            .context("Failed to parse q as usize")?;

        let mut queries = Vec::with_capacity(q);
        for line in lines.take(q) {
            let mut parts = line.split_whitespace();
            let typ = parts
                .next()
                .context("Failed to parse query type")?
                .parse()
                .context("Failed to parse query type as i32")?;
            let x = parts
                .next()
                .context("Failed to parse x")?
                .parse()
                .context("Failed to parse x as i32")?;
            let y = parts
                .next()
                .context("Failed to parse y")?
                .parse()
                .context("Failed to parse y as i32")?;
            queries.push(Query { typ, x, y });
        }

        let result = dynamic_array(n, &queries);

        for answer in result {
            writeln!(fptr, "{}", answer).context("Failed to write to output file")?;
        }
    }

    Ok(())
}
