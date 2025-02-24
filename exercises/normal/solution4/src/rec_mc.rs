pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut amount = amount;
    let mut number: u32 = 0;
    let face_values: Vec<u32> = vec![100, 50, 30, 20, 10, 5, 2, 1];
    for i in face_values.iter() {
        number += amount / *i;
        amount = amount % *i;
        if amount == 0 {
            break;
        }
    }

    number
}
