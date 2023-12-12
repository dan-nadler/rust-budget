use std::io::empty;
use std::process::exit;

use budget::api;
use budget::sim;
use budget::sim::SimulationResult;

fn generate_json_schemas() {
    use schemars::schema_for;

    let schema = schema_for!(sim::cash::Account);
    let ser = serde_json::to_string_pretty(&schema).unwrap();
    std::fs::write("src/schemas/.account.json", ser).unwrap();

    let schema = schema_for!(sim::cash::Payment);
    let ser = serde_json::to_string_pretty(&schema).unwrap();
    std::fs::write("src/schemas/.payment.json", ser).unwrap();
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

    // Output to excel file
    let excel = args.contains(&String::from("--excel"));
    let excel_file = args.iter().position(|s| s == "--excel");


    if config_file.is_some() {
        let config_file = config_file.unwrap();
        let config_file = &args[config_file + 1];
        config = std::fs::read_to_string(config_file).unwrap();
    }

    if run_sim {
        if config_file.is_none() {
            println!("--run-sim requires --config <config_file>");
            exit(1)
        }
        let account: sim::cash::Account = serde_yaml::from_str(&config).unwrap();
        results = sim::run_simulation(account);

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
