#[test]
fn should_not_compile() {
    let test_case = trybuild::TestCases::new();
    test_case.compile_fail("tests/fails/*.rs");
}
