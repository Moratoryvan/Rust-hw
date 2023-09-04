pub fn intersection(a: &Vec<String>, b: &Vec<String>) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    'outer: for i_b in b.clone() {
        for i_a in a.clone() {
            if i_b == i_a {
                ret.push(i_b);
                continue 'outer; 
            }
        }
    }
    return ret
}