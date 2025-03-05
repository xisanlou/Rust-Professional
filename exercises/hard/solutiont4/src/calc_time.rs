use std::collections::HashMap;
use std::collections::HashSet;

pub fn time_info(time: &str) -> String {
    let time_list: Vec<&str> = time.split("-").collect();
    if time_list.len() != 3 {return "Time input error.".to_string();}

    // 从输入中识别年，月，日
    let t_year: usize = time_list[0].parse().unwrap();
    let t_month_of_year: usize = time_list[1].parse().unwrap();
    let t_day_of_month: usize = time_list[2].parse().unwrap();

    // 判断是否为闰年
    let t_is_leap_year: bool = leap_year(t_year);

    println!("Haogy time: {}, {}, {}, {}", t_year,t_month_of_year,t_day_of_month,t_is_leap_year);

    // 这一天是一年中的第几天
    let t_day_of_year = day_of_year(t_month_of_year, t_day_of_month, t_is_leap_year);

    // 一年中剩余天数
    let remaining_days: usize; 
    if t_is_leap_year {
        remaining_days = 366 - t_day_of_year;
    } else {
        remaining_days = 365 - t_day_of_year;
    }

    format!("{},{},{},{},{},{}",
        week_of_year(t_year, t_day_of_year),
        day_of_week(t_year, t_day_of_year),
        t_day_of_year,
        remaining_days,
        days_to_spring_festival(t_year, t_month_of_year, t_day_of_month),
        days_to_a_share_openning(t_year, t_month_of_year, t_day_of_month),
    )
}

// 识别是否为闰年
fn leap_year(year: usize) -> bool {
    if year % 400 == 0 {
        return true;
    } else if year % 100 == 0 {
        return false;
    } else if year % 4 == 0 {
        return true;
    } else {
        return false;
    }
}

// 计算一个月有几天
fn days_in_a_month(month: usize, is_leap_year: bool) -> usize {
    if month == 2 {
        if is_leap_year {
            return 29;
        } else {
            return 28;
        }
    }

    let set: HashSet<usize> = HashSet::from([1, 3, 5, 7, 8, 10, 12]);
    if set.get(&month).is_none() {
        return 30;
    }

    31
}

// 计算某日是一年中第几天
fn day_of_year(month: usize, day: usize, is_leap_year: bool) -> usize {
    if month == 1 {
        return day;
    }

    let mut days = 0;
    for i in 1..month {
        days += days_in_a_month(i, is_leap_year);
    }

    days + day
    
}

// 计算元旦是周几
fn new_year_day_day_of_week(year: usize) -> usize {
    if year == 2000 {return 6;}
    let mut days: usize = 0;
    let annual_diff:usize;
    if year > 2000 {
        annual_diff = year - 2000;
        // 计算这年1月1日距离2000年1月1日有多少天
        days += annual_diff * 365 + 1;
        // 每4年闰1天
        days += (annual_diff - 1) / 4;
        // 每百年不润
        days -= (annual_diff - 1) / 100;
        // 400年闰
        days += (annual_diff - 1) / 400;
        return (days % 7 + 6) % 7;
    } else {
        annual_diff = 2000 - year;
        // 计算这年1月1日距离2000年1月1日有多少天
        days += annual_diff * 365;
        // 每4年闰1天
        days += (annual_diff - 1) / 4;
        // 每百年不润
        days -= (annual_diff - 1) / 100;
        // 400年闰
        days += (annual_diff - 1) / 400;
        return 6 - days % 7;
    }
}

// 计算某年第几天是周几
fn day_of_week(year: usize, day_of_year: usize) -> usize {
    let day_of_week_1_1 = new_year_day_day_of_week(year);
    let x = ((day_of_year - 1) % 7 + day_of_week_1_1) % 7;
    if x == 0 {
        return 7;
    }
    x
}

// 计算某年第几天是这年的第几周,周日是周末
fn week_of_year(year: usize, day_of_year: usize) -> usize {
    let the_year_days_num: usize;
    let pre_year = year - 1;
    let pre_year_days_num: usize;
    if leap_year(pre_year) {
        pre_year_days_num = 366;
    } else {
        pre_year_days_num = 365;
    }
    if leap_year(year) {
        the_year_days_num = 366;
    } else {
        the_year_days_num = 365;
    }

    // 计算1月1日是周几
    let day_of_week_1_1 = new_year_day_day_of_week(year);
    // 计算1月1日这周在本年有几天
    let week_1_days = (7 - day_of_week_1_1 + 1) % 7;
    
    let mut result: usize;
    
    let day_of_week_12_31: usize;

    if week_1_days >= 4 {
        // 1月1日这周是本年第一周
        if day_of_year <= week_1_days {return 1;}
    
        if (day_of_year - week_1_days) % 7 == 0 {
            // 周日是整周，只需加上第一周
            result = (day_of_year - week_1_days) / 7 + 1;
        } else {
            // 非周日还需多加一周
            result = (day_of_year - week_1_days) / 7 + 2;
        }

        // 年尾处理
        if result == 53 {
            // 12月31日是周几
            day_of_week_12_31 = day_of_week(year, the_year_days_num);
            // 在周1~周3之间，则这周是下一年的第一周
            if day_of_week_12_31 >= 1 && day_of_week_12_31 <= 3 {
                result = 1;
            }
        }

    } else {
        // 1月1日这周是上年最后一周
        if day_of_year <= week_1_days {
            
            // 递归计算上一年最后一周的周数
            return week_of_year(pre_year, pre_year_days_num);
        }
    
        if (day_of_year - week_1_days) % 7 == 0 {
            // 周日是整周，不需加上第一周
            result = (day_of_year - week_1_days) / 7;
        } else {
            // 非周日还需多加一周
            result = (day_of_year - week_1_days) / 7 + 1;
        }

        // 年尾处理
        if result == 53 {
            // 12月31日是周几
            day_of_week_12_31 = day_of_week(year, the_year_days_num);
            // 在周1~周3之间，则这周是下一年的第一周
            if day_of_week_12_31 >= 1 && day_of_week_12_31 <= 3 {
                result = 1;
            }
        }
    }
    
    result
}

// 距离下一个春节还有多少天？
fn days_to_spring_festival(year:usize, month:usize, day:usize) -> isize {
    let t_day_of_year = day_of_year(month, day, leap_year(year));

    // 春节日期列表
    let spring_festival_set: HashMap<usize,(usize, usize)> = HashMap::from([
        (2024, (2, 10)),
        (2025, (1, 29)),
        (2026, (2, 17)),
        (2027, (2, 6)),
    ]);

    let mut spring_festival_day_of_year: usize;
    if let Some(&(month_s, day_s)) = spring_festival_set.get(&year) {
        spring_festival_day_of_year = day_of_year(month_s, day_s, leap_year(year));
    } else {
        return -1;
    }

    if t_day_of_year <= spring_festival_day_of_year {
        return (spring_festival_day_of_year - t_day_of_year) as isize;
    }

    let remaining_days: usize; 
    if leap_year(year) {
        remaining_days = 366 - t_day_of_year;
    } else {
        remaining_days = 365 - t_day_of_year;
    }

    let next_year = year + 1;
    if let Some(&(month_s, day_s)) = spring_festival_set.get(&next_year) {
        spring_festival_day_of_year = day_of_year(month_s, day_s, leap_year(next_year));
    } else {
        return -1;
    }

    (remaining_days + spring_festival_day_of_year) as isize
}

// 计算到A股下次开盘的天数
fn days_to_a_share_openning(year:usize, month:usize, day:usize) -> isize {
    if year != 2025 {return -1;}
    let the_day_of_year = day_of_year(month, day, leap_year(year));
    if the_day_of_year == 365 {
        return 1;
    }

    let holiday_days_2025: Vec<(usize, usize)> = Vec::from([
        (1, 1), 
        (1, 28), (1, 29), (1, 30), (1, 31), (2, 1), (2, 2), (2, 3), (2, 4),
        (4, 4), (4, 5), (4, 6),
        (5, 1), (5, 2), (5, 3), (5, 4), (5, 5),
        (5, 31), (6, 1), (6, 2),
        (10, 1), (10, 2), (10, 3), (10, 4), (10, 5), (10, 6), (10, 7), (10, 8), 
    ]);
    let mut holiday_set: HashSet<usize> = HashSet::new();
    for (m,d) in holiday_days_2025.iter() {
        holiday_set.insert(day_of_year(*m, *d, leap_year(2025)));
    }

    let mut open_day = the_day_of_year + 1;
    let mut day_of_weak_open_day: usize;
    while open_day < 365 {
        day_of_weak_open_day = day_of_week(2025, open_day) % 7;
        if day_of_weak_open_day != 0 
            && day_of_weak_open_day != 6 
            && holiday_set.get(&open_day).is_none()
        {
            return (open_day - the_day_of_year - 1) as isize;
        }

        open_day += 1;
    }

    -1
}
