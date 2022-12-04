fn main() {
    let x = String::new();
    safe_goto::safe_goto! {
      begin() { if true { goto s1(x) } else { goto s2(x) } },
      s1(x: String) { drop(x); goto s3() },
      s2(x: String) { println!("{x}"); return },
      s3() { return }
    };
}
