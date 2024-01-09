use budget::sim;
use budget::sim::SimulationResult;
use std::process::exit;

fn generate_json_schemas() {
    use schemars::schema_for;

    let schematize_objs = vec![
        (schema_for!(sim::cash::Account), ".account.json"),
        (schema_for!(sim::cash::Payment), ".payment.json"),
        (schema_for!(sim::portfolio::Asset), ".asset.json"),
        (schema_for!(sim::portfolio::Portfolio), ".portfolio.json"),
    ];

    for obj in schematize_objs {
        let schema = obj.0;
        let ser = serde_json::to_string_pretty(&schema).unwrap();
        std::fs::write(format!("src/schemas/{}", obj.1), ser).unwrap();
    }
}

fn main() {
    // parse command line args
    let args: Vec<String> = std::env::args().collect();

    // flag to generate json schema
    let gen_schema = args.contains(&String::from("--gen-schema"));

    // run simulation with provided config
    let run_sim = args.contains(&String::from("--run-sim"));
    let config_file = args.iter().position(|s| s == "--config");
    let mut config = String::new();
    let mut results: SimulationResult = SimulationResult::new(vec![], vec![]);

    // Optional Portfolio conifguration
    let portfolio_file = args.iter().position(|s| s == "--portfolio");
    let mut portfolio: Option<sim::portfolio::Portfolio> = None;

    // Output to excel file
    let excel = args.contains(&String::from("--excel"));
    let excel_file = args.iter().position(|s| s == "--excel");

    if config_file.is_some() {
        let config_file = config_file.unwrap();
        let config_file = &args[config_file + 1];
        config = std::fs::read_to_string(config_file).unwrap();
    }

    if portfolio_file.is_some() {
        let portfolio_file = portfolio_file.unwrap();
        let portfolio_file = &args[portfolio_file + 1];
        let portfolio_config = std::fs::read_to_string(portfolio_file).unwrap();
        portfolio = Some(serde_yaml::from_str(&portfolio_config).unwrap());
    }

    if run_sim {
        if config_file.is_none() {
            println!("--run-sim requires --config <config_file>");
            exit(1)
        }
        let account: sim::cash::Account = serde_yaml::from_str(&config).unwrap();
        results = sim::run_simulation(account, portfolio);

        if excel {
            if excel_file.is_none() {
                println!("--excel requires --excel <excel_file>");
                exit(1)
            }
            let excel_file = excel_file.unwrap();
            let excel_file = &args[excel_file + 1];
            sim::excel::write_sim(results, excel_file);
        }
    }

    if gen_schema {
        generate_json_schemas();
        exit(0)
    }

    exit(0)
}
