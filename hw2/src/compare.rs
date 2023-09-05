// 我们假设全空的字符串是字母序最小的
pub fn compare_string(x: &str, y: &str) -> bool {
    let x_str: Vec<char> = x.chars().collect();
    let y_str: Vec<char> = y.chars().collect();
    let mut i = 0;
    loop {
        if i>= x_str.len() || i >= y_str.len() {
            break false;
        } 
        if x_str[i] == y_str[i] {
            i += 1;
            continue;
        } 
        if x_str[i] > y_str[i] {
            break true;
        } else {
            break false;
        }
    }
}