fn main() {
    let answer = 43;
    /*
        assert_eq
        - compares two items for a match
        - if no match, throws a runtime error
    */
    assert_eq!(answer, 43); // -> no runtime error
    assert_eq!(answer, 40); // -> runtime error
}
