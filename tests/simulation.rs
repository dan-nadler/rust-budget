use budget::sim;

#[test]
fn integration_test() {
    let config = std::fs::read_to_string("./scenarios/examples/default_account.yaml").unwrap();
    let account: sim::cash::Account = serde_yaml::from_str(&config).unwrap();
    sim::run_simulation(account, None, false);
}
