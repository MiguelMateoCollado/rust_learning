fn main() {
    let mut list = String::from("");
    let mut num = 0;
    while num < 100 {
        let mut new_num_first = 0;
        while new_num_first <= num {
            list.push_str("#");
            new_num_first += 1
        }
        list.push_str("\n");
        num += 1;
    }

    // list.push_str("\n");
    while num - 1 > 0 {
        // println!("{:?} {:?}", list.len(), num - 1);
        let mut new_num = num.clone() - 1;
        while new_num > 0 {
            list.push_str("#");
            new_num -= 1
        }
        list.push_str("\n");
        num -= 1
    }
    println!("{list}");
}
