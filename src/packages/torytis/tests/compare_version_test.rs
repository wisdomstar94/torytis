use version_compare::compare;

#[test]
fn compare_version_test() {
    let k = compare("1.0.0", "1.0.0").unwrap();
    println!("k : {:?}", k);
}