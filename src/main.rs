use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    time::Instant,
};

use rand::prelude::*;

const N: usize = 1024;

const word_file: &str = "words_alpha.txt";

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn main() {
    let lines = read_lines(word_file.to_string());
    // Iterate over the lines of the file, and in this case print them.
    let mut line_num = 0;
    for line in lines {
        // here we get the word and
        println!("{}:{}", line_num, line.unwrap());
        line_num += 1;
    }

    // println!("hello world");
}

fn test_perf() {
    let mut rng = rand::thread_rng();

    let mut a: Vec<Vec<f64>> = Vec::with_capacity(N);
    let mut b: Vec<Vec<f64>> = Vec::with_capacity(N);
    let mut c: Vec<Vec<f64>> = Vec::with_capacity(N);

    for i in 0..N {
        a.push(Vec::with_capacity(N));
        b.push(Vec::with_capacity(N));
        c.push(Vec::with_capacity(N));
        for _ in 0..N {
            a[i].push(rng.gen());
            b[i].push(rng.gen());
            c[i].push(0.0);
        }
    }

    let start = Instant::now();

    /*
           for (int i = 0; i < n; i++)
           for (int j = 0; j < n; j++)
               for (int k = 0; k < n; k++)
                   c[i][j] += a[i][k] * b[k][j];
    */

    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                c[i][j] += a[i][k] * b[k][j]
            }
        }
    }

    println!("time elapsed {}", start.elapsed().as_millis());
}
