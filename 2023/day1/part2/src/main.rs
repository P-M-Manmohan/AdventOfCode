use std::fs;


#[derive(Debug,PartialEq)]
struct NumberPosition{
    number : u32,
    position: usize,
}

fn get_digits(input : &str) -> Vec<NumberPosition>
{
    input
        .chars()
        .enumerate()
        .filter(|(_,c)| c.is_digit(10))
        .map(|(i,c)|NumberPosition{
            number : c.to_digit(10).unwrap(),//to_digit used for a single char
            //.parse used for strings
            position: i,
        })
        .collect()
}

fn spelled_digits(input : &str) -> Vec<NumberPosition>
{
    let spelled_numbers = vec!["one","two","three","four","five","six","seven","eight","nine"];
    let mut positions : Vec<NumberPosition> = Vec::new();
    for spelled_number in spelled_numbers.clone(){
        let mut start=0;
            while let Some(position) = input[start..].find(spelled_number){
                 positions.push(NumberPosition{
                    number : spelled_numbers.iter().position(|&n| n==spelled_number).unwrap() as u32 + 1,
                    position : start +position,
                 });
                 start+=position +spelled_number.len();
            }
    }
    positions.sort_by_key(|np| np.position);
    positions
}

fn numbers_from_line(input: &str) -> u32 {
   

    let mut numbers = get_digits(&input);
    let spelled_digits = spelled_digits(&input);
    numbers.extend(spelled_digits);
    numbers.sort_by_key(|np| np.position);

    let first = numbers.first().expect("atleast 1 element must be there");
    let last = numbers.last().expect("atleast 1 element must be there");
    let answer = first.number * 10 + last.number;
    answer
}

fn main() {
    let contents = fs::read_to_string("../cal_doc.txt")
        .expect("error reading file");
        
    let ans = contents
        .lines()
        .map(|line| numbers_from_line(&line)).sum::<u32>();

    println!("{:?}",ans)
    
        
}
