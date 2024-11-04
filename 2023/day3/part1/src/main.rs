use std::fs;

fn string_to_vec(line: &str) -> Vec<char>{
    let char_vec: Vec<char> = line.chars().collect();
    return char_vec
}

fn partnum(matrix: &mut Vec<Vec<char>> , imean:usize , jmean:usize, length:usize ) -> u32 {
    let mut sum=0;
    for i in imean-1..imean+2{
        for j in jmean-1..jmean+2{
            if matrix[i][j].is_numeric(){
                let mut num= matrix[i][j].to_digit(10).unwrap();
                matrix[i][j]='.';
                let mut k=1;
                let mut tens=10;
                while matrix[i][j-k].is_numeric(){
                    num=matrix[i][j-k].to_digit(10).unwrap()*tens+num;
                    matrix[i][j-k]='.';
                    if j-k>0 {k+=1;}
                    tens*=10;
                }
                    k=1;
                while j+k<length && matrix[i][j+k].is_numeric(){
                    //println!("{:?} {:?}",num,matrix[i][j+k].to_digit(10).unwrap());
                    num=matrix[i][j+k].to_digit(10).unwrap()+num*10;
                    matrix[i][j+k]='.';
                    k+=1;
                }
                //println!("{:?}",matrix);
                //println!("{:?}",num);
                sum+=num;
            }
        }
    }
    return sum
}

fn find_symbol(matrix : &Vec<Vec<char>>,length:usize) -> u32{
    let mut part_sum: u32 =0;
    let mut clone=matrix.clone();
    for i in 0..matrix.len(){
        for j in 0..length{
            if clone[i][j].is_ascii_punctuation() == true && clone[i][j] != '.'{
                //println!("{:?} {:?} {:?}",i,j,clone.get(i));
                part_sum += partnum(&mut clone,i,j,length);
               // println!("{:?}",part_sum);
            }
        }
    }
    return part_sum
}

fn main() {
    let contents = fs::read_to_string("../input.txt").expect("shoudl have read file");
    let matrix: Vec<Vec<char>> = contents.lines().map(|line| string_to_vec(line)).collect();
    let length: usize= matrix.get(0).unwrap().len() as usize; 
    println!("{:?}",length);
    let x = find_symbol(&matrix,length);
    println!("{:?}",x);
}
