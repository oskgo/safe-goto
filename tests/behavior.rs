use safe_goto::safe_goto;

#[test]
fn test_output() {
    assert_eq!{1, safe_goto!{
        begin() {1}
    }}
}

#[test]
fn test_goto_args() {
    let x = safe_goto!{
        begin() {
            goto other(2)
        },
        other(n: i32) {
            n
        }
    };
    assert_eq!(x, 2)
}