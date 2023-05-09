
fn generate_combinations_rec(chars:&str, len:i32) -> Vec<String> {
    let mut ret = Vec::new();

    fn com_help (ret:&mut Vec<String>, chars:&str, current: String, remaining_len:i32) {
        if remaining_len == 0 {
            ret.push(current);
        }
        else {
            for value in chars.chars() {
                let cur = current.clone() + &value.to_string();
                com_help(ret, chars, cur, remaining_len - 1);
            }
        }
    }

    com_help(&mut ret, chars, String::new(), len);
    return ret;
}

fn generate_combinations_itr(values: &str, length: usize) -> Vec<String> {

    let num_values = values.len();
    let mut combinations = Vec::new();
    let mut current_indices: Vec<usize>  = vec![0;length];

    loop {

        let combination: String = current_indices.iter().map(|&i| values.chars().nth(i).unwrap().to_string()).collect();
        combinations.push(combination);

        let mut i = length;
        while i > 0 && current_indices[i - 1] == num_values - 1 {
            i -= 1;
        }

        if i == 0 {
            break;
        }

        current_indices[i - 1] += 1;
        for j in i..length {
            current_indices[j] = 0;
        }
    }

    combinations
}


fn main() {
    let s: &str = "abc";
    println!("------------");
    let values = generate_combinations_rec(s, 3);
    for val in values {
        println!("{val}");
    }
    println!("------------");
    let values = generate_combinations_itr(s, 3);
    for val in values {
        println!("{val}");
    }
    println!("------------");
}

