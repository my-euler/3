fn main() {
    let mut result = 1u64;
    let mut current = 600851475143;
    let mut i = 2;

    loop {
        if current % i == 0 {
            result = result.max(i);
            while current % i == 0 {
                current /= i
            }
        }

        i += 1;
        if i > current {
            break;
        }
    }

    println!("{}", result)
}
