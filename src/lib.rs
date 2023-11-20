#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    }
}

#[test]
fn should_make_an_empty_vec() {
    let x: Vec<u32> = my_vec![];
    assert!(x.is_empty());
}
