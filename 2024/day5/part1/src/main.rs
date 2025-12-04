use std::fs;
use std::collections::HashMap;

fn extract_rules_updates(input: String) -> (HashMap<u32, Vec<u32>>,Vec<Vec<u32>>){
    let mut rules: HashMap<u32, Vec<u32>>  = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut position = 0;
    let mut update = Vec::new();
    let mut num = String::new();

    for i in input.chars(){
        if i == '\n' && position == 1{
            let n1 = num1.parse::<u32>().unwrap();
            let n2 = num2.parse::<u32>().unwrap();
            if rules.contains_key(&n1){
                let mut arr: Vec<u32> =  rules.get(&n1).unwrap().clone();
                arr.push(n2);
                rules.insert(n1, arr);
            }else{
                let mut arr = Vec::new();
                arr.push(n2);
                rules.insert(n1,arr);

            }
            num1 = "".to_string();
            num2 = "".to_string();
            position = 0;
            continue;
        }else if i == '|' && position == 0{
            position = 1;
            continue;
        }else{
            if position == 0{
                num1.push(i);
            }else{
                num2.push(i);
            }
        }
        if position == 0 && i == '\n'{
            position = 3;
            continue;
        }

        if position == 3{
           if i == ','{
                update.push(num.parse::<u32>().unwrap());
                num = "".to_string();
                continue;
           }else if i == '\n'{
                update.push(num.parse::<u32>().unwrap());
                num = "".to_string();
                updates.push(update.clone());
                update = Vec::new();
                continue;
            }
           num.push(i);
        }
    }

    (rules,updates)
}

fn find_valid(rules: HashMap<u32, Vec<u32>>, updates: Vec<Vec<u32>>) -> u32{
    let mut sum = 0;
    for update in updates{
        let mut valid = 1;
        let l = update.len();
        for i in 0..l{
            let default_arr = Vec::new();
            let conditions = rules.get(&update[i]).unwrap_or(&default_arr);
            for condition in conditions{
                for j in 0..i{
                    if *condition == update[j]{
                        valid = 0;
                        break;
                    }
                }
                if valid == 0 {
                    break;
                }
            }
        }
        if valid == 1{
            sum += update[l/2];
            println!("{update:?}")
        }
    }
    return sum;
}

fn main() {

    let contents = fs::read_to_string("input").expect("input file missing");
    let (rules, updates) = extract_rules_updates(contents);

    let valid = find_valid(rules, updates);

    println!("{valid:?}");
}
