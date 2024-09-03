use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn dynamic_array(n: usize, queries: &[(i32, i32, i32)]) -> Vec<i32> {
    let mut seq_list = vec![Vec::new(); n];
    let mut last_answer = 0;
    let mut answers = Vec::new();

    for &(query_type, x, y) in queries {
        let idx = ((x ^ last_answer) as usize) % n;

        if query_type == 1 {
            seq_list[idx].push(y);
        } else if query_type == 2 {
            last_answer = seq_list[idx][(y as usize) % seq_list[idx].len()];
            answers.push(last_answer);
        }
    }

    answers
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let output_path = env::var("OUTPUT_PATH").unwrap_or_else(|_| "output.txt".to_string());
    let mut fptr = File::create(&output_path)?;

    // Parse the first input line for `n` and `q`
    let first_input = stdin_iterator.next().unwrap().unwrap();
    let mut first_input_iter = first_input.split_whitespace();

    let n: usize = first_input_iter.next().unwrap().parse().unwrap();
    let q: usize = first_input_iter.next().unwrap().parse().unwrap();

    // Pre-allocate vector for queries with a tuple (query_type, x, y)
    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        let line = stdin_iterator.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let query_type = parts.next().unwrap().parse().unwrap();
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        queries.push((query_type, x, y));
    }

    let result = dynamic_array(n, &queries);

    for answer in result {
        writeln!(fptr, "{}", answer)?;
    }

    Ok(())
}
