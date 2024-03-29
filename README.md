# About

I had to do some budgetting and used it as an excuse to learn Rust.

This works by defining an "account" as a YAML file. The account includes a starting 
balance and a list of cash flows. The cash flows can be configured with one of a handful 
of basic recurrences, an effective date range, and a tax rate.

Use this by running `cargo run -- --run-sim --config path/to/your/account.yaml`

Optionally include `--excel path/to/excel_output.xlsx` to write the time series of cash flows and account 
balance to an excel file.

# Plans for the future
- [x] ~Define portfolios to use with accounts~ ✅
- [x] ~Excel exports~ ✅
- [ ] Basic portfolio management and investment forecasts
- [ ] Monte carlo forecasting
- [ ] Web API
- [ ] Better Excel exports
  - [x] ~Pivots~ ✅
  - [ ] Charts

# Other thoughts
- Integrating the API with ChatGPT might be neat if it works well
- Polars is probably required for a monte carlo simulation
- Could Excel exports include pre-built charts and formulas?
