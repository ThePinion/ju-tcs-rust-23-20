use std::path::Path;

use crate::{head, tail};

#[test]
fn test_head() {
    let result = head(Path::new("src/tests/test.in"), 1).unwrap();
    assert_eq!(result, vec!["1".to_string()])
}

#[test]
fn test_tail() {
    let result = tail(Path::new("src/tests/test.in"), 2).unwrap();
    assert_eq!(result, vec!["2".to_string(), "3".to_string()])
}
