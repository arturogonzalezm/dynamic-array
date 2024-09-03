use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn dynamic_array(n: usize, queries: &[Vec<i32>]) -> Vec<i32> {
    let mut seq_list = vec![Vec::new(); n];
    let mut last_answer = 0;
    let mut answers = Vec::with_capacity(queries.len()); // Pre-allocate space for answers

    for query in queries {
        let (query_type, x, y) = (query[0], query[1], query[2]);
        let idx = ((x ^ last_answer) as usize) % n;

        if query_type == 1 {
            seq_list[idx].push(y);
        } else if query_type == 2 {
            let size = seq_list[idx].len();
            last_answer = seq_list[idx][(y as usize) % size];
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

    let first_input_line = stdin_iterator.next().unwrap().unwrap();
    let mut first_input_iter = first_input_line.split_whitespace();

    let n: usize = first_input_iter.next().unwrap().parse().unwrap();
    let q: usize = first_input_iter.next().unwrap().parse().unwrap();

    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        let line = stdin_iterator.next().unwrap().unwrap();
        let mut query = Vec::with_capacity(3);
        for num in line.split_whitespace() {
            query.push(num.parse().unwrap());
        }
        queries.push(query);
    }

    let result = dynamic_array(n, &queries);

    for answer in result {
        writeln!(fptr, "{}", answer)?;
    }

    Ok(())
}
