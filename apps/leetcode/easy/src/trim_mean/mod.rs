pub fn trim_mean(arr: Vec<i32>) -> f64 {
    println!("arr: {:?}", arr);

    let mut array: Vec<i32> = arr;
    array.sort();
    let len = array.len(); // 20

    if len >= 20 && len <= 1000 {
        let len_del_5cent = len * 5 / 100; // 1

        println!("len: {}", len);
        let end_len = len - len_del_5cent;

        println!("array: {:?}", array);

        for i in (end_len..len).rev() {
            array.remove(i);
            // array.pop();
        }
        for i in 0..len_del_5cent {
            array.remove(i);
        }
        println!("array: {:?}", array);

        let mut sum: i32 = array.iter().sum();
        let sum_float: f64 = sum.into();
        let mut new_len: i32 = array.len().try_into().unwrap(); // error here?
        let new_len_float: f64 = new_len.into();
        let mean: f64 = sum_float / 100000.00000 / new_len_float * 100000.00000;

        return mean;
    } else {
        return 1.00000;
    }
}
