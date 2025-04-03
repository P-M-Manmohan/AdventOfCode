use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Maps{
    name: String,
    values: Vec<(u32,u32,u32)>,
    len: usize
}

fn string_to_vec(line: &str) -> Vec<String>{
    let re = Regex::new(r"[ ]").unwrap();
    let mut seeds= vec![];
    for seed in re.split(line){
        seeds.push(seed.to_string());
    }
    seeds 
}

fn string_tup(mut root: Vec<String>) -> Maps{
                
    let title:String = root[0].clone();
    let mut values= Vec::new();
    root.remove(0);
    root.remove(0);
    let length = root.len();
    let mut i=0;
    while i < length-1{
        values.push((root[i].parse().unwrap(),root[i+1].parse().unwrap(),root[i+2].parse().unwrap()));
        i+=3;
    }
    let length=values.len();

    let map = Maps{
        name : title,
        values : values.clone(),
        len : length,
    } ;
    map
}


fn classification_split(contents: &mut Vec<Vec<String>>) -> Vec<Maps>{
    let mut flag=0;
    let mut complete = Vec::new();
    let mut root:Vec<String> = Vec::new();
    for line in &mut *contents{
        if line[0]==""{
            if root[0] != "seeds:"{
                complete.push(string_tup(root.clone()));
            }
            flag=0;
            continue;
        }
        if flag==0{
            root= line.to_vec();
            flag=1;
        }else {
            root.append(line);
        }
    }
        complete.push(string_tup(root));
        complete
}

fn get_result(inp: u32, map:&Maps) -> u32 {
    let mut flag = 0;
    let mut result: i64 = 0;
    println!("{:?}",map.name);
    for value in map.values.clone(){
        if inp>value.1 && inp< (value.1 + value.2){
            result = inp as i64 + ( value.0 as i64-value.1 as i64 );
            flag=1;
            break;
        }
    }
    if flag==0{
        result = inp as i64;
    }
    result as u32
}

fn get_lowest_location(contents: Vec<Maps>,seeds: Vec<String>) -> u32 {
    
    let mut loc: u32 = 0;
    for seed in seeds {
        let seed = seed.parse::<u32>().unwrap();
        let soil = get_result(seed, &contents[0]);
        let fertilizer = get_result(soil, &contents[1]);
        
        let water = get_result(fertilizer, &contents[2]);
        let light = get_result(water, &contents[3]);
        let temperature = get_result(light, &contents[4]);
        let humidity = get_result(temperature, &contents[5]);
        let location = get_result(humidity, &contents[6]);

        if loc > location || loc == 0  {
            loc = location;
        }

    }
    loc
}   

fn main() {
    let contents = fs::read_to_string("input").expect("file not found");
    let mut contents_vector:Vec<Vec<String>> = contents.lines().map(|line| string_to_vec(line)).collect();
    let mut seeds=contents_vector[0].clone();
    seeds.remove(0);
    let contents = classification_split(&mut contents_vector);
    
    let location = get_lowest_location(contents, seeds);

    println!("{:?}",location);
}
