#[test]
fn test_validate_source() {
    let source = "Ap(7, a)";
    let prime_set_json = "[7]";
    let stratum_id = "Tower_A";
    let res = pirtm_compiler::compiler::validate_source(source, prime_set_json, stratum_id);
    assert!(res.is_ok());
    let envelope = res.unwrap();
    assert_eq!(envelope.diagnostics.len(), 0);
}
