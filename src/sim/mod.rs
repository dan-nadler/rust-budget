use serde::Serialize;
pub mod cash;
pub mod excel;

#[allow(dead_code)]
#[derive(Serialize, Clone)]
pub struct AccountBalance {
    pub date: chrono::NaiveDate,
    pub account_name: String,
    pub balance: f32,
}

impl AccountBalance {
    fn new(date: chrono::NaiveDate, account_name: String, balance: f32) -> AccountBalance {
        AccountBalance {
            date,
            account_name,
            balance,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct SimulationResult {
    pub balances: Vec<AccountBalance>,
    pub payments: Vec<cash::Payment>,
}

impl SimulationResult {
    pub fn new(balances: Vec<AccountBalance>, payments: Vec<cash::Payment>) -> SimulationResult {
        SimulationResult { balances, payments }
    }
}

pub fn run_simulation(mut account: cash::Account) -> SimulationResult {
    // read config from file account.yaml
    println!("--- Beginning Simulation ---");
    println!("Loaded Account: {}\n", account.name);

    let mut results = SimulationResult::new(vec![], vec![]);

    let mut d = account.start_date;

    while d < account.end_date {
        let b = account.balance_at(d);
        println!("{}, {} balance, {}", d, account.name, b);
        results
            .balances
            .push(AccountBalance::new(d, account.name.clone(), b));
        d = d.succ_opt().unwrap();
    }

    let mut d = account.start_date;
    while d < account.end_date {
        let flows = account.flows_at(d);
        for f in &flows {
            println!("{}, {}, {}", d, f.cash_flow.name.clone().unwrap(), f.amount);
            results.payments.push(f.clone());
        }
        d = d.succ_opt().unwrap();
    }

    println!("--- End of Simulation ---");
    results
}


#[test]
fn test() {
    let config = std::fs::read_to_string("./scenarios/examples/default.yaml").unwrap();
    let account: cash::Account = serde_yaml::from_str(&config).unwrap();
    run_simulation(account);
}