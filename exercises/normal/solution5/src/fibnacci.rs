pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut f0: u32 = 0;
    let mut f1: u32 = 1;
    let mut f2: u32 = 0;

    let mut odd_sum = f1;
    while f1 < threshold {
        f2 = f0 + f1;
        if f2 % 2 != 0 && f2 < threshold {
            odd_sum += f2;
        }
        f0 = f1;
        f1 = f2;
    }

    odd_sum
}
