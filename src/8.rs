fn main() {
    let mut v = vec![1, 2, 3];
    let x = 4;
    let y = 5;
    for i in 0..x {
        for j in 0..y {
            if i == j {
                v.push(i);
            }
        }
    }
}
