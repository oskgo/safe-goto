#![feature(trace_macros)]

//trace_macros!(true);

use safe_goto::safe_goto;

#[test]
fn test_begin() {
    safe_goto!{
        'begin: {println!("foo")}
    };
}

#[test]
fn test_goto_other() {
    safe_goto!{
        'begin: {
            goto 'other
        },
        'other: {
            "";
        }
    }
}