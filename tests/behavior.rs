use safe_goto::safe_goto;

#[test]
fn test_output() {
    assert_eq! {1, safe_goto!{
        begin() {1}
    }}
}

#[test]
fn test_goto_args() {
    let x = safe_goto! {
        begin() {
            goto other(2)
        },
        other(n: i32) {
            n
        }
    };
    assert_eq!(x, 2)
}

#[test]
fn test_mut() {
    let x = safe_goto! {
        begin() {
            goto other(2)
        },
        other(mut n: i32) {
            n += 1;
            n
        }
    };
    assert_eq!(x, 3)
}

#[test]
fn test_internals() {
    fn foo(x: i32, y: i32) -> Option<i32> {
        if x + 40 <= y {
            return None;
        }
        return Some(x);
    }
    let ret = safe_goto!(
        begin() {goto s1(42)},
        s1(n: i32) {
            if n % 2 == 0 {
                goto s3(n / 2, n + 1)
            } else {
                goto s2()
            }
        },
        s2() {
            goto s1(84)
        },
        s3(x: i32, y: i32) {
            match foo(x, y) {
                None => x + y,
                Some(n) => goto s1(n),
            }
        }
    );
    assert_eq!(127, ret);
}