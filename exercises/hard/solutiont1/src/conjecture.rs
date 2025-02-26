const MAX_N: usize = 100000;


pub fn goldbach_conjecture() -> String {
    let mut prime: Vec<usize> = Vec::new();
    let mut is_prime: Vec<usize> = vec![0; MAX_N + 5];
    
    // arrays init
    let mut num1: usize = 0;
    for i in 2..=MAX_N {
        if is_prime[i] == 0 { 
            prime.push(i);
        }

        for j in 0..prime.len() {
            num1 = i * prime[j];
            if num1 > MAX_N {break;} 
            is_prime[num1] = 1;
            if i % prime[j] == 0 {break;}
        }
    }

    let mut answer: Vec<usize> = Vec::new();

    for i in (9..=MAX_N).step_by(2) {
        if is_prime[i] == 0 {continue;}
        if check(i, &prime) {continue;}
        answer.push(i);
        if answer.len() >= 2 {break;}
    }

    format!("{},{}", answer[0], answer[1])
}

fn check(n: usize, pr: &Vec<usize>) -> bool {
    for i in 0..pr.len() {
        if pr[i] >= n {break;}
        
        if binary_search(1, n-pr[i], n-pr[i]) {return true;}
    }

    false
}

fn binary_search(l: usize, r: usize, x: usize) -> bool {
    let mut l = l as usize;
    let mut r = r as usize;

    if l > r {return false;}
    
    let mid = (l + r) / 2;
    let sqrt_2 = 2 * mid * mid;

    if sqrt_2 == x {
        return true;
    } else if sqrt_2 < x {
        l = mid + 1;
    } else {
        r = mid - 1;
    }

    binary_search(l, r, x)
    
}
