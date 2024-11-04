use std::fs;

fn main() {

    let contents = fs::read_to_string("../cal_doc.txt").expect("should have read file"); 

    let ans: u32 = contents
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| {c.is_digit(10)});

            let first = chars
                .next()
                .expect("there should be atleast one digit");

            let num = match chars.last() {
                Some(last) => format!("{}{}",first,last),
                None => format!("{}{}",first,first),
            };
        num.parse::<u32>().unwrap()
    }) 
    .sum();

    println!("{}",ans);
}
