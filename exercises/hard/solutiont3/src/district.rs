use serde_json::{Value, Map};
use std::fs;
use std::collections::HashSet;

pub fn count_provinces() -> String {
    let data = fs::read_to_string("district.json").unwrap();
    let value: Value = serde_json::from_str(&data).unwrap();

    //println!("Value: {:?}", value);

    let mut result: Vec<usize> = Vec::new();

    // 将输入按照序号1..进行划分计算
    for (_, v) in value.as_object().unwrap().iter() {
        result.push(provinces_num(v));
    }

    // 汇总并打印
    result.iter().map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
    
}

fn provinces_num(v: &Value) -> usize {
    let mut provinces: Vec<HashSet<String>> = Vec::new();
    // 迭代内层的列表
    for (i, (kc, vc)) in v.as_object().unwrap().iter().enumerate() {
        provinces.push(HashSet::new());
        provinces[i].insert(kc.to_string());
        // 进一步遍历array
        for c in vc.as_array().unwrap().iter() {
            if let Value::String(name) = c {
                provinces[i].insert(name.to_string());
            } 
        }

    }
    //println!("###raw citys: {:?}", provinces);
    // 合并有重叠的Set
    let mut j = 0;
    let mut m: usize;
    let mut jointed: bool;
    while j < provinces.len() - 1 {
        jointed = false;
        m = provinces.len() - 1;
        while j < m {
            //println!("j={}:{:?} | m={}:{:?} | ", j, &provinces[j], m, &provinces[m]);
            //println!("provinces: {:?}", &provinces);
            if !(provinces[j].is_disjoint(&provinces[m])) {
                jointed = true;
                //println!("j={} is_disjoint m={} |", j, m); 
                provinces[j] = provinces[j].union(&provinces[m]).map(|x| x.to_string()).collect::<HashSet<String>>();
                provinces.remove(m);
            }
            m -= 1;
        }
        if jointed == false {
            j += 1;
        }
    }
    //println!("citys: {:?}", provinces);
    provinces.len()
}
