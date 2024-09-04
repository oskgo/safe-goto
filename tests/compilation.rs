use safe_goto::safe_goto;

#[test]
#[allow(unreachable_code)]
fn test_minimal() {
    safe_goto! {
        begin() {}
    }
}

#[test]
#[allow(unreachable_code)]
fn test_call() {
    safe_goto! {
        begin() {Vec::<()>::new()}
    };
}

#[test]
#[allow(unreachable_code)]
fn test_print() {
    safe_goto! {
        begin() {println!("foo")}
    };
}

#[test]
fn test_goto() {
    safe_goto! {
        begin() {
            goto other()
        },
        other() {}
    }
}

#[test]
#[allow(unused_braces)]
fn test_goto_braces() {
    safe_goto! {
        begin() {
            {goto other()}
        },
        other() {}
    };
}

#[test]
#[allow(path_statements)]
fn test_goto_args() {
    safe_goto! {
        begin() {
            goto other(2)
        },
        other(n: i32) {
            n;
        }
    }
}

#[test]
fn test_references() {
    safe_goto! {
        begin() {
            let a = &mut 30;
            goto other()
        },
        other() {
            *a = 10;
        }
    }
}
