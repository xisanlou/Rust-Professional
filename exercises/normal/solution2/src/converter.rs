pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    

    let mut num_string = num_str.to_string();
    num_string.pop();

    let num_vec: Vec<&str> = num_string.split('(').collect();
    //println!("v0={} | v1={}", num_vec[0], num_vec[1]);
    
    let from_base = num_vec[1].parse::<u32>().unwrap();
    //println!("from_base = {}", from_base);
    let mut num_base10 = 0;
    let mut pow_base = 1;
    for c in num_vec[0].to_lowercase().chars().rev() {
        if let Some(x) = c.to_digit(from_base) {
            println!("char_to_num: {}", x);
            num_base10 = num_base10 + x * pow_base;
            pow_base *= from_base;
        }
        
    }
    //println!("base10_number is: {}", num_base10);

    let mut res_vec: Vec<char> = Vec::new();
    while num_base10 != 0 {
        if let Some(x) = char::from_digit(num_base10 % to_base, to_base){
            res_vec.insert(0, x);
            num_base10 /= to_base;
        }
        
    }

    res_vec.iter().collect::<String>()

}

