/*
    Appellation: default <module>
    Contrib: @FL03
*/

#[test]
fn crate_compiles() {
    fn adder<A, B, C>(a: A, b: B) -> C
    where
        A: core::ops::Add<B, Output = C>,
    {
        a + b
    }
    // verify the function works
    assert_eq!(adder(10, 10), 20);
    // test different types against a wrong answer
    assert_ne!(adder(1.0, 0.75), 1.74);
}
