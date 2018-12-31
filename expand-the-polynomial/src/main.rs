// https://www.codingame.com/ide/puzzle/expand-the-polynomial
extern crate regex;

use regex::Regex;

fn extract_coef(input: &str) -> Vec<i32> {
    let re = Regex::new(r"([+-]?\d*)([x](?:\^(\d*))?)?").unwrap();
    let mut ces = Vec::new();
    let mut max_e = 0;
    for caps in re.captures_iter(input) {
        let c_str = caps.get(1).map(|m| m.as_str()).unwrap();
        let c = match c_str {
            "+" => 1,
            "-" => -1,
            _ => c_str.parse::<i32>().unwrap_or(1),
        };
        let e = match caps.get(2).map(|m| m.as_str()).unwrap_or("") {
            "" => 0,
            "x" => 1,
            _ => caps.get(3).map_or("0", |m| m.as_str()).parse::<i32>().unwrap_or(0),
        };
        if e > max_e {
            max_e = e;
        }
        ces.push((c, e));
    }
    let mut coef = vec![0; max_e as usize + 1];
    for (c, e) in ces.iter() {
        coef[*e as usize] = *c
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
            output_c[i+j] += m*n;
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
    let re = Regex::new(r"\(([x\d^+-]+)\)(\^(\d*))?").unwrap();
    let poly = "(x-1)(x+1)(x^2+1)";
    let mut coefs = Vec::new();
    let mut output_e = 0;
    for caps in re.captures_iter(poly) {
        let e = caps.get(3).map_or("1", |m| m.as_str()).parse::<i32>().unwrap();
        for _i in 0..e {
            let coef = extract_coef(caps.get(1).unwrap().as_str());
            output_e += coef.len()-1;
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
        } else if i == output_c.len() -1 {
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
