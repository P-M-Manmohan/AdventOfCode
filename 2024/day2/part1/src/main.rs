use std::fs;

fn main() {
    let contents = fs::read_to_string("../test").expect("error reading file");
    let s1: String = String::new();
    let s2: String = String::new();
    let order:i32 = -1;
    for char in contents.char() {
        if char == ' ' &&  s1.len()!=0{
            s1 = s2;
            s2.clear();
            continue;
        }else if char == '\n'{
            s1.clear();
            s2.clear();
            order = -1;
        }else if char != ' ' {
            s2.push(char);
        }else{
            if order != -1{
                if s1<s2 {
                    order = 0;
                } else if s1>s2 {
                    order = 1;
                }
            }
        }
    }
}
