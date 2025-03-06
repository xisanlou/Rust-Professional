pub fn retire_time(time: &str, tp: &str) -> String {
    // 出生年月分解
    let time_list: Vec<&str> = time.split("-").collect();
    if time_list.len() != 2 {return "Birth time error.".to_string();}
    let birth_year: usize = time_list[0].parse().unwrap();
    let birth_month: usize = time_list[1].parse().unwrap();

    // 根据类型调用不同处理函数
    match tp {
        "男职工" => return comute_retire_time(birth_year, birth_month, 60, 3, 4),
        "原法定退休年龄55周岁女职工" => return comute_retire_time(birth_year, birth_month, 55, 3, 4),
        "原法定退休年龄50周岁女职工" => return comute_retire_time(birth_year, birth_month, 50, 5, 3),
        _ => return "Worker type error.".to_string(),
    };
}

// 
fn comute_retire_time(
        birth_year: usize, 
        birth_month: usize, 
        old_retire_age: usize, 
        max_delay_year: usize,
        every_year_delay_months: usize) -> String 
{
    let implementation_year: usize = 2025;
    let begin_delay_birth_year = implementation_year - old_retire_age;

    if birth_year < begin_delay_birth_year {
        return format!("{}-{:0>2},{},0", birth_year + old_retire_age, birth_month, old_retire_age);
    }

    // 计算推迟的月数
    let mut delay_months: usize = ((birth_year - begin_delay_birth_year) * 12 + birth_month) / every_year_delay_months;
    if ((birth_year - begin_delay_birth_year) * 12 + birth_month) % every_year_delay_months != 0 {
        delay_months += 1;
    }
    // 最多推迟3年
    if delay_months > max_delay_year * 12 { delay_months = max_delay_year * 12; }

    // 计算退休年月
    let mut retire_year = birth_year + old_retire_age + (birth_month + delay_months) / 12;
    if (birth_month + delay_months) % 12 == 0 {retire_year -= 1; }
    let mut retire_month = (birth_month + delay_months) % 12;
    if retire_month == 0 { retire_month = 12;}

    // 计算退休年龄，并且只保留两位小数
    let retire_age: f32 = ((old_retire_age as f32 + (delay_months as f32) / 12.0) * 100.0).round() / 100.0;
    if retire_age.round() == retire_age {
        return format!("{}-{:0>2},{},{}", retire_year, retire_month, retire_age as usize, delay_months);
    }

    format!("{}-{:0>2},{:.2},{}", retire_year, retire_month, retire_age, delay_months)
}
