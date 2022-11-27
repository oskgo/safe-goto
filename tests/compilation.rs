use safe_goto::safe_goto;

#[test]
fn test_minimal() {
    safe_goto! {
        begin() {}
    }
}

#[test]
fn test_call() {
    safe_goto! {
        begin() {Vec::<()>::new()}
    };
}

#[test]
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
