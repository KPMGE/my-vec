// NOTE: we use the $() for repetition, basically, anytime we 
// match the expression, we're gonna execute it on the expression block

#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($($element: expr),+) => {{
        let mut v = Vec::new();
        $(v.push($element);)+
        v
    }};
}

#[test]
fn should_make_an_empty_vec() {
    let x: Vec<u32> = my_vec![];
    assert!(x.is_empty());
}

#[test]
fn should_make_a_vec_with_comma_separated_elements() {
    let x: Vec<u32> = my_vec![1, 2, 3];
    assert!(!x.is_empty());
    assert_eq!(x[0], 1);
    assert_eq!(x[1], 2);
    assert_eq!(x[2], 3);
}
