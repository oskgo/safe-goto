use safe_goto::safe_goto;

fn foo(n: i32) -> i32 {
    safe_goto!(
        begin() {
            goto first(n)
        },
        first(mut n: i32) {
            n -= 73;
            if n % 33 == 0 {
                goto second(n)
            } else if n > 150 {
                goto third(n)
            }
            n
        },
        second(mut n: i32) {
            n -= 83;
            if n % 43 == 0 {
                goto third(n)
            } else if n > 120{
                goto first(n)
            }
            n
        },
        third(mut n: i32) {
            n -= 93;
            if n % 53 == 0 {
                goto first(n)
            } else if n > 100{
                goto second(n)
            }
            n
        },
    )
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.replace(&['\n', '\r'], "").parse().unwrap();
    foo(n);
}