use std::fs;
use regex::Regex;

fn multiply(arr: Vec<(&str, &str)>) -> i32 {
    let mut sum: i32 = 0;
    for (num1, num2) in arr {
        let product = num1.parse::<i32>().unwrap()*num2.parse::<i32>().unwrap();
        sum+=product;
    }
    return sum;
}

fn main() {
    let contents = fs::read_to_string("input").expect("error reading file");
    let re = Regex::new(r"mul\((?<num1>\d+),(?<num2>\d+)\)|(?<dont>don't\(\))|(?<do>do\(\))").unwrap();
    let mut active = 1;
    let muls : Vec<(&str,&str)> = re.captures_iter(&contents).map(|m| {
        if m.name("do").map(|mat| mat.as_str()).unwrap_or("") == "do()"{
            active = 1;
        }else if m.name("dont").map(|mat| mat.as_str()).unwrap_or("") == "don't()"{
            active =0;
        }
        if active == 1 {
            let num1  = m.name("num1").map(|mat| mat.as_str()).unwrap_or("0");
            let num2  = m.name("num2").map(|mat| mat.as_str()).unwrap_or("0");
            (num1,num2)
        }else{
            ("0","0")
        }
    }).collect();
    println!("{muls:?}");
    let sum = multiply(muls);
    println!("{sum:?}");
}
