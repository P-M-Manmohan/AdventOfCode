use std::fs;
fn main() {
    let contents = fs::read_to_string("../cal_doc.txt")
        .expect("Should have been able to read the file");

    let mut sum = 0;
    let mut num = 0;
    let mut flag=0;
    let mut digit=0;
    
    for element in contents.chars(){
        
        if (element as i32) > 47 && (element as i32) < 58{
           
            digit=(element as i64) - 48;
            
            if flag==0 {
                num=digit*10;
                flag=1;
            }
        }
        else if element == '\n'{
            flag=0;
            num+=digit;
            println!("{}",num);
            sum+=num;
        }        
    }
    println!("{}",sum);
}
