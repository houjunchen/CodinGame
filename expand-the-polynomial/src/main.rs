// https://www.codingame.com/ide/puzzle/expand-the-polynomial
use std::io;

fn extract_coef(input: &str) -> Vec<i32> {
    let splited: Vec<&str> = input.split("x").collect();
    let mut ces = Vec::new();

    for i in 0..splited.len() {
        if splited[i] == "" {
            ces.push(1);
        } else if splited[i] == "-" {
            ces.push(-1);
        } else if splited[i].starts_with("+") || splited[i].starts_with("-") {
            ces.push(1);
            ces.push(splited[i].parse::<i32>().unwrap_or(0));
        } else if splited[i].starts_with("^") {
            if splited[i].contains("+") {
                let index = splited[i].find("+").unwrap();
                ces.push(splited[i][1..index].parse::<i32>().unwrap());
                ces.push(splited[i][index..splited[i].len()].parse::<i32>().unwrap_or(1));
            } else if splited[i].contains("-") {
                let index = splited[i].find("-").unwrap();
                ces.push(splited[i][1..index].parse::<i32>().unwrap());
                ces.push(splited[i][index..splited[i].len()].parse::<i32>().unwrap_or(-1));
            }
        } else {
            ces.push(splited[i].parse::<i32>().unwrap_or(0));
        }
    }

    if ces.len() % 2 == 1 {
        ces.push(0);
    }

    let mut coef = vec![0; ces[1] as usize + 1];
    for i in 0..ces.len()/2 {
        coef[ces[i*2+1] as usize] = ces[i*2];
    }
    coef
}

fn multiply(coef1: &Vec<i32>, coef2: &Vec<i32>, output_e: usize) -> Vec<i32> {
    let mut output_c = vec![0; output_e + 1];
    for (i, &m) in coef1.iter().enumerate() {
        if m == 0 {
            continue;
        }
        for (j, &n) in coef2.iter().enumerate() {
            if n == 0 {
                continue;
            }
            output_c[i + j] += m * n;
        }
    }
    output_c
}

fn get_output_coef(coefs: Vec<Vec<i32>>, output_e: usize) -> Vec<i32> {
    let mut output_c = multiply(&coefs[0], &coefs[1], output_e);
    for k in 2..coefs.len() {
        output_c = multiply(&output_c, &coefs[k], output_e);
    }
    output_c
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let poly = input_line.trim().to_string();

    let mut coefs = Vec::new();
    let mut output_e = 0;

    let splited: Vec<&str> = poly.split(|c| c == '(' || c == ')').collect();
    let mut last_coef: Vec<i32> = Vec::new();
    for i in 0..splited.len() {
        if splited[i] == "" || splited[i] == "*" {
            continue;
        }
        if splited[i].starts_with("^") {
            for _i in 0..splited[i].trim_left_matches("^").parse::<i32>().unwrap() - 1 {
                output_e += last_coef.len() - 1;
                coefs.push(last_coef.clone());
            }
        } else {
            let coef = extract_coef(splited[i]);
            output_e += coef.len() - 1;
            last_coef = coef.clone();
            coefs.push(coef);
        }
    }

    let output_c = get_output_coef(coefs, output_e);
    for i in (0..output_c.len()).rev() {
        if output_c[i] == 0 {
            continue;
        }
        if i == 0 {
            match output_c[i] {
                x if x > 0 => print!("+{}", output_c[i]),
                _ => print!("{}", output_c[i]),
            }
        } else if i == 1 {
            match output_c[i] {
                1 => print!("+x"),
                -1 => print!("-x"),
                x if x > 0 => print!("+{}x", output_c[i]),
                _ => print!("{}x", output_c[i]),
            }
        } else if i == output_c.len() - 1 {
            match output_c[i] {
                1 => print!("x^{}", i),
                -1 => print!("-x^{}", i),
                _ => print!("{}x^{}", output_c[i], i),
            }
        } else {
            match output_c[i] {
                1 => print!("+x^{}", i),
                -1 => print!("-x^{}", i),
                x if x > 0 => print!("+{}x^{}", output_c[i], i),
                _ => print!("{}x^{}", output_c[i], i),
            }
        }
    }
}
