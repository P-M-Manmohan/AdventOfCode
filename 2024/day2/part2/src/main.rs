use std::fs;

fn line_arr(arr: String) -> Vec<Vec<u32>> {
    let mut list:Vec<Vec<u32>> = Vec::new();
    let mut inner:Vec<u32> = Vec::new();
    let mut s:String = String::new();
    for char in arr.chars(){
        if char != ' ' && char != '\n'{
            s.push(char);
        }else if char == ' '{
           inner.push(s.parse().unwrap());
           s.clear();
        }else if char == '\n' {
            inner.push(s.parse().unwrap());
            s.clear();
            list.push(inner.clone());
            inner.clear();
        }
    }
    return list;
}

fn remove_element(arr: Vec<u32>,index: usize)-> Vec<u32>{
    let mut list1 = Vec::new();
    let mut i = 0;
    while i < arr.len(){
        if index == i{
            break;
        }
        list1.push(arr[i]);
        i+=1;
    }
    i+=1;
    while i < arr.len(){
        list1.push(arr[i]);
        i+=1;
    }
    return list1;
}

fn find_valid(arr: Vec<Vec<u32>> ) -> u32{
    let mut rep = 0;
    for mut list in arr{
        let mut j = 0;
        let original_list = list.clone();
        while j<= original_list.len(){
            let mut safe = 1;
            let mut order = -1;
            let mut i = 1;
           while i<list.len(){
               let diff = list[i] as i32 - list[i-1] as i32;
               if order ==-1{
                  if diff < 0 {
                      order = 2;
                  }else if diff > 0{
                      order = 1;
                  }else{
                      safe = 0;
                      break;
                      }
                  }
                  if order == 1 && ( diff <= 0 || diff >3 ){
                      safe = 0;
                      break;
                  }
                  if order == 2 && ( diff >= 0 || diff < -3 ){
                      safe = 0;
                      break;
                  }
                  i+=1;
               }
               if safe == 1 {
               println!("{original_list:?} - safe");
               rep+=1;
               break;
           }
           else{
               println!("{list:?} - unsafe");
               }
           list = original_list.clone();
           list = remove_element(list, j);
           j+=1;
           println!("{j:?}");
        }
        
    }
    return rep;
}

fn main() {
    let contents = fs::read_to_string("input").expect("error reading file");
    let arr = line_arr(contents);
    let rep = find_valid(arr);
    println!("{rep:?}");
}
