use std::fs;


fn create_matrix(input: String) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();
    for i in input.chars() {
        if i == '\n'{
            matrix.push(row.clone());
            row.clear();
            continue;
        }
            row.push(i);
    }
    return matrix;
}

fn find_string(matrix: Vec<Vec<char>>,l: i32) -> i32 {
    let mut reps = 0;
    
    let mut j = 1;
    let l = l as usize;
    while j < l-1 {
        let mut i = 1;
        while i < l-1  { 
           if matrix[i][j] == 'A'{
               let mut sample1 = String::new();
               let mut sample2 = String::new();
               let start: i32 = -1;
               let end :i32 = 2;
               for x in start..end{
                   sample1.push(matrix[(i as i32 +x) as usize][(j as i32+x) as usize]);
                   sample2.push(matrix[(i as i32 +x) as usize][(j as i32-x) as usize]);
               }
               if (sample1 == "MAS" || sample1 =="SAM") && (sample2 == "MAS" || sample2=="SAM"){
                    reps += 1;
               }
           }
           i+=1;
        }
        j+=1;
    }


    println!("{reps:?}");

    return reps;
}

fn main() {
    let contents = fs::read_to_string("input").expect("input file missing");
    let matrix = create_matrix(contents);
    let l = matrix[0].len();
    let reps = find_string(matrix.clone(),l as i32);
    println!("{reps:?}");
    
}
