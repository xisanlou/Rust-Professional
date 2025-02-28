use rand::Rng;
use num::Integer;

/// Help by deepseek
pub fn find_max_prime_factor(number: u128) -> u128 {
    if number <= 1 { return 1; }
    factor(number).into_iter().max().unwrap_or(1)
}


// 核心优化：所有运算适配u128且防溢出
fn pollards_rho(n: u128) -> u128 {
    if n % 2 == 0 { return 2; }
    
    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(2..100) as u128;
    let c = rng.gen_range(1..100) as u128;
    let mut y = x;
    let mut d = 1u128;

    // 使用溢出安全计算
    let next = |x: u128, c: u128, n: u128| {
        x.checked_mul(x)
            .and_then(|v| v.checked_add(c))
            .and_then(|v| v.checked_rem(n))
            .unwrap_or_else(|| (x.wrapping_mul(x) + c) % n)
    };

    while d == 1 {
        x = next(x, c, n);
        y = next(next(y, c, n), c, n);
        d = (x.max(y) - x.min(y)).gcd(&n);
    }
    d
}

// 递归分解质因数（优化尾递归）
fn factor(n: u128) -> Vec<u128> {
    if n <= 1 { return vec![]; }
    if miller_rabin(n) { return vec![n]; }
    
    let d = pollards_rho(n);
    let mut factors = factor(d);
    factors.append(&mut factor(n / d));
    factors
}

// Miller-Rabin测试（适配u128）
fn miller_rabin(n: u128) -> bool {
    if n <= 3 { return n >= 2; }
    if n % 2 == 0 { return false; }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    for a in [2, 3, 5, 7] {
        if a >= n { continue; }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 { continue; }
        
        for _ in 0..s-1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 { break; }
        }
        if x != n - 1 { return false; }
    }
    true
}

/// 俄罗斯农民算法实现安全的乘法取模
fn mul_mod(a: u128, b: u128, m: u128) -> u128 {
    let mut res = 0;
    let (mut a, mut b) = (a % m, b % m);
    
    while b > 0 {
        if b & 1 == 1 {
            res = (res + a) % m;
        }
        a = (a << 1) % m;
        b >>= 1;
    }
    res
}

/// 快速幂算法实现安全模幂
fn mod_pow(base: u128, exp: u128, modulus: u128) -> u128 {
    if modulus == 1 { return 0; }
    let mut result = 1;
    let (mut base, mut exp) = (base % modulus, exp);
    
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exp >>= 1;
    }
    result
}


