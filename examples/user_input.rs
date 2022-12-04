use safe_goto::safe_goto;

fn get_num_from_user() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.replace(&['\n', '\r'], "").parse().unwrap()
}

fn main() {
    println!(
        "{}",
        safe_goto!(
            begin() {
                goto first(0)
            },
            first(mut n: i32) {
                n += get_num_from_user();
                if n % 2 == 0 {
                    goto second(n)
                } else if n < 100 {
                    goto third(n)
                }
                n
            },
            second(mut n: i32) {
                n += 2*get_num_from_user();
                if n % 2 == 0 {
                    goto third(n)
                } else if n < 100{
                    goto first(n)
                }
                n
            },
            third(mut n: i32) {
                n += 3*get_num_from_user();
                if n % 2 == 0 {
                    goto first(n)
                } else if n < 100{
                    goto second(n)
                }
                n
            },
        )
    );
}
