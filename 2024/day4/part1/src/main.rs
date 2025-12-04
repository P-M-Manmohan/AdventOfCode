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

fn find_string(matrix: Vec<Vec<char>>,input: String, l: i32) -> i32 {
    let mut reps = 0;
    let mut input_str = Vec::new();
    for c in input.chars(){
       input_str.push(c); 
    }
    // scan from start to end row wise
    for row in matrix.clone() {
        let mut j = 0;
        let mut i = 0;
        while i < l  { 
           if input_str[j] == row[i as usize]{
               j+=1;
           }else{
               if j!= 0{
                    i-=1;
               }
               j = 0;
           }
           if j == 4 {
               reps+=1;
               j = 0;
           }
           i+=1;
        }
    }
    println!("{reps:?}");

    
    // scan from start to end column wise
    let mut column = 0;
    while column < l{
        let mut i = 0;
        let mut j = 0;
        while i < l {
           if input_str[j] == matrix.clone()[i as usize][column as usize]{
               j+=1;
           }else{
               if j!= 0{
                    i-=1;
               }
               j = 0;
           }
           if j == 4 {
               //println!("{i} {column}");
               reps+=1;
               j = 0;
           }
           i+=1;
        }
        column+=1;
    }

    println!("{reps:?}");
    // scan diagonal start from top left
    let mut row: i32 = l-1;
    while row >= 0{
        let mut column: i32 = 0;
        let mut input_pointer = 0;
        while  row < l{
            
            if input_str[input_pointer] == matrix.clone()[row as usize][column as usize]{
                input_pointer+=1;
            }
            else{
                if input_pointer != 0{
                    row-=1;
                    column-=1;
                }
                input_pointer=0;
            }
            if input_pointer == 4{
                reps+=1;
                input_pointer=0;
            }
            row+=1;
            column+=1;
        }
        row = (l-1) - column;
    }
    let mut column: i32 = l-1;
    while column >= 0{
        let mut row: i32 = 0;
        let mut input_pointer = 0;
        while  column < l {
            if input_str[input_pointer] == matrix.clone()[row as usize][column as usize] && row != column{
                input_pointer+=1;
            }
            else{
                if input_pointer != 0{
                    row-=1;
                    column-=1;
                }
                input_pointer=0;
            }
            if input_pointer == 4{
                reps+=1;
                input_pointer=0;
            }
            row+=1;
            column+=1;
        }
        column = (l-1) - row;
    }


    println!("{reps:?}");
    
    // start scan from top right to bottom left
    let mut row: i32 = l-1;
    while row >= 0{
        let mut column: i32 = l-1;
        let mut input_pointer = 0;
        while  row < l {
            
            if input_str[input_pointer] == matrix.clone()[row as usize][column as usize]{
                input_pointer+=1;
            }
            else{
                if input_pointer != 0{
                    row-=1;
                    column+=1;
                }
                input_pointer=0;
            }
            if input_pointer == 4{
                
                reps+=1;
                input_pointer=0;
            }
            row+=1;
            column-=1;
        }
        row = column;
    }

    let mut column: i32 = l-1;
    while column >= 0 {
        let mut row: i32 = 0;
        let mut input_pointer = 0;
        while  column >= 0{
            if input_str[input_pointer] == matrix.clone()[row as usize][column as usize] && (row+column) != (l-1){
                input_pointer+=1;
            }
            else{
                if input_pointer != 0{
                    row-=1;
                    column+=1;
                }
                input_pointer=0;
            }
            if input_pointer == 4{
                reps+=1;
                input_pointer=0;
            }
            row+=1;
            column-=1;
        }
        column = row-2;
    }


    println!("{reps:?}");

    return reps;
}

fn main() {
    let contents = fs::read_to_string("input").expect("input file missing");
    let matrix = create_matrix(contents);
    let l = matrix[0].len();
    let mut reps = find_string(matrix.clone(), "XMAS".to_string(),l as i32);
    reps += find_string(matrix,"SAMX".to_string(), l as i32);
    println!("{reps:?}");
    
}
