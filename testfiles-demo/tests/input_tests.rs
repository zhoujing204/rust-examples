// tests/input_tests.rs

use testfiles_demo::get_user_input_safe;

#[test]
fn test_get_user_input() {
    let result = get_user_input_safe("Enter: ");
    assert!(result.is_ok());

    if let Ok(input) = result {
        assert!(input.is_empty() || !input.contains('\n'));
    }
}