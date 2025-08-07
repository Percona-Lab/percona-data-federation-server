use percona_data_federation_server::factorial;
use percona_data_federation_server::fibonacci;

#[test]
fn integration_test_1() {
    assert!(factorial::compute(8) + fibonacci::compute(16) > 1);
}
