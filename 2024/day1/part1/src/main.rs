use std::fs;


fn get_int_list( contents: String) -> (Vec<u32>,Vec<u32>) {

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    let mut i: u32 = 0;
    let mut s: String = String::new();
    for char in contents.chars(){
        if (char == ' ' || char == '\n')  && s.len() != 0 {
            if i % 2 == 0{
                left.push(s.parse().unwrap());
                i+=1;
                s.clear();
            }else{
                right.push(s.parse().unwrap());
                i+=1;
                s.clear();
            }
        }
        if char != ' ' && char != '\n'{
            s.push(char);
        }
    }
    return (left,right)
}

fn merge_sort(arr: &mut Vec<u32>){
    let length = arr.len();
    if length > 1{
        let mid = length/2;
        let mut left = arr[0..mid].to_vec();
        let mut right = arr[mid..].to_vec();

        merge_sort(&mut left);
        merge_sort(&mut right);

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        while i < left.len() && j < right.len(){
            if left[i] < right[j]{
                arr[k] = left[i];
                i+=1;
            }else{
                arr[k] = right[j];
                j+=1;
            }
            k+=1;
        }
        while i < left.len() {
            arr[k] = left[i];
            i+=1;
            k+=1;
        }

        while j < right.len() {
            arr[k] = right[j];
            j+=1;
            k+=1;
        }
    }
}


fn main() {
    let contents = fs::read_to_string("../input").expect("error reading file");
    let (mut left, mut right) = get_int_list(contents);
    merge_sort(&mut left);
    merge_sort(&mut right);
    let mut i = 0;
    let mut sum = 0;
    while i < left.len(){
        sum += left[i].abs_diff(right[i]);
        i+=1 ;
    }
    println!("{sum:?}");
    
}
