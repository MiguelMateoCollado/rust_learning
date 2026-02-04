// funciona pero no con los parametros que se me dijo
// fn main() {
//     let mut list = String::from("######");
//     let mut num = 0;
//     let mut list_clone = list.clone();
//     while num < list.len() {
//         println!("{list_clone}");
//         list_clone.pop();
//         num += 1;
//     }
// }

fn main() {
    let mut list = String::from("#############");
    let mut newList = String::from("");
    let mut num = list.len();
    while num >= 0 {
        println!("{:?}", (num));
        let mut size = 0;
        while size < num {
            newList.push_str("#");
            size += 1;
        }
        newList.push_str("\n");
        println!("{newList}");
        num -= 1;
    }
}
