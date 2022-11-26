use safe_goto::safe_goto;

#[test]
fn test_output() {
    assert_eq!{1, safe_goto!{
        'begin: {1}
    }}
}