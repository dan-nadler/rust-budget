use chrono::{Datelike, NaiveDate};
use memoize::memoize;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

// a list contianing the number of days in each month
pub const DAYS_IN_MONTH: [u32; 12] = [
    31, // January
    28, // February
    31, // March
    30, // April
    31, // May
    30, // June
    31, // July
    31, // August
    30, // September
    31, // October
    30, // November
    31, // December
];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub enum Frequency {
    Once,
    // Weekly,
    // BiWeekly,
    SemiMonthly,
    MonthStart,
    MonthEnd,
    // Quarterly,
    // SemiAnnually,
    Annually,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Payment {
    pub cash_flow: CashFlow,
    pub date: NaiveDate,
    pub amount: f32,
}

impl Payment {
    pub fn new(date: NaiveDate, amount: f32, cash_flow: CashFlow) -> Payment {
        Payment {
            date,
            amount,
            cash_flow,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CashFlow {
    pub name: Option<String>,
    pub amount: f32,
    pub frequency: Frequency,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub tax_rate: f32,
}

impl CashFlow {
    // cache the cash flow payments

    // frequency is optional with default value of "once"
    pub fn new(
        name: Option<String>,
        amount: f32,
        frequency: Option<Frequency>,
        start_date: Option<chrono::NaiveDate>,
        end_date: Option<chrono::NaiveDate>,
        tax_rate: Option<f32>,
    ) -> CashFlow {
        CashFlow {
            name,
            amount,
            frequency: frequency.unwrap_or(Frequency::Once),
            start_date,
            end_date,
            tax_rate: tax_rate.unwrap_or(0.0),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn payments(
        &mut self,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
        tax_payments: bool,
    ) -> Vec<Payment> {
        // returns a vec of payments
        let mut d = start_date.pred_opt().unwrap();
        let mut payments: Vec<Payment> = vec![];

        // If tax payments have been requests, but the tax rate is 0, return an empty vec
        if tax_payments && self.tax_rate == 0.0 {
            return payments;
        }

        while d < end_date {
            d = d.succ_opt().unwrap();

            // handle start and end dates
            if let Some(start_date) = &self.start_date {
                if start_date > &d {
                    continue;
                }
            }
            if let Some(end_date) = &self.end_date {
                if end_date < &d {
                    continue;
                }
            }

            // handle 'once' frequency
            if let Frequency::Once = &self.frequency {
                if let Some(start_date) = &self.start_date {
                    if start_date != &d {
                        continue;
                    }
                }
            }

            // handle 'month-start' frequency
            if let Frequency::MonthStart = &self.frequency {
                if d.day() != 1 {
                    continue;
                }
            }

            //handle 'month-end' frequency
            if let Frequency::MonthEnd = &self.frequency {
                let last_day_of_month = DAYS_IN_MONTH[(d.month() as usize) - 1];
                if d.day() != last_day_of_month {
                    continue;
                }
            }

            // handle 'semi-monthly' frequency
            if let Frequency::SemiMonthly = &self.frequency {
                let last_day_of_month = DAYS_IN_MONTH[(d.month() as usize) - 1];
                if d.day() != last_day_of_month && d.day() != 15 {
                    continue;
                }
            }

            // handle 'annually' frequency
            if let Frequency::Annually = &self.frequency {
                let sd = &self.start_date.unwrap();
                if d.month() != sd.month() || d.day() != sd.day() {
                    continue;
                }
            }

            let mut p = Payment::new(
                d,
                if tax_payments {
                    self.amount * -self.tax_rate
                } else {
                    self.amount
                },
                self.clone(),
            );

            if tax_payments {
                p.cash_flow
                    .set_name(format!("{} Tax", self.name.clone().unwrap()));
            }

            payments.push(p.clone());
        }
        payments
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct Account {
    pub name: String,
    pub balance: f32,
    pub cash_flows: Vec<CashFlow>,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

impl Account {
    pub fn new(
        name: String,
        balance: f32,
        cash_flows: Vec<CashFlow>,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
    ) -> Account {
        Account {
            name,
            balance,
            cash_flows,
            start_date,
            end_date,
        }
    }

    pub fn add_cash_flow(&mut self, cash_flow: CashFlow) {
        self.cash_flows.push(cash_flow);
    }

    pub fn payments(
        &mut self,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
    ) -> Vec<Payment> {
        let mut payments: Vec<Payment> = vec![];
        for cash_flow in &mut self.cash_flows {
            payments.append(&mut cash_flow.payments(start_date, end_date, false));
            payments.append(&mut cash_flow.payments(start_date, end_date, true));
        }
        // sort by date
        payments.sort_by(|a, b| a.date.cmp(&b.date));
        payments
    }

    pub fn balance_at(&mut self, date: chrono::NaiveDate) -> f32 {
        let mut balance = self.balance;
        for cash_flow in &mut self.cash_flows {
            let payments = cash_flow.payments(self.start_date, date, false);
            let taxes = cash_flow.payments(self.start_date, date, true);
            for payment in payments.into_iter().chain(taxes) {
                balance += payment.amount;
            }
        }
        balance
    }

    pub fn flows_at(&mut self, date: chrono::NaiveDate) -> Vec<Payment> {
        // Returns a vec of Payment objects corresponding to all flows on this date
        let mut flows: Vec<Payment> = vec![];
        for cash_flow in &mut self.cash_flows {
            let payments = &mut cash_flow.payments(date, date, false);
            let taxes = &mut cash_flow.payments(date, date, true);
            for payment in payments.iter_mut().chain(taxes) {
                flows.push(payment.clone());
            }
        }
        flows.sort_by(|a, b| a.date.cmp(&b.date));
        flows
    }
}

impl std::cmp::PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::cmp::Eq for Account {}

impl std::hash::Hash for Account {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[test]
fn test_account_hash() {
    // test that the account hash is based on the name
    let mut account1 = Account::new(
        "Test Account".to_string(),
        0.0,
        vec![CashFlow::new(
            Some("Test Cash Flow".to_string()),
            100.0,
            Some(Frequency::MonthStart),
            None,
            None,
            None,
        )],
        NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2020, 12, 31).unwrap(),
    );

    let mut account2 = Account::new(
        "Test Account 2".to_string(),
        0.0,
        vec![CashFlow::new(
            Some("Test Cash Flow".to_string()),
            100.0,
            Some(Frequency::MonthStart),
            None,
            None,
            None,
        )],
        NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2020, 12, 31).unwrap(),
    );

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    account1.hash(&mut hasher);
    let hash1 = hasher.finish();

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    account2.hash(&mut hasher);
    let hash2 = hasher.finish();

    assert_ne!(hash1, hash2);
}

#[memoize]
pub fn get_account_balance_at(account: Account, date: chrono::NaiveDate) -> f32 {
    let mut a = account.clone();
    let f = a.flows_at(date);
    let mut b: f32 = 0.0;
    if date > a.start_date {
        b = get_account_balance_at(a, date - chrono::Duration::days(1));
    }
    b + f.iter().fold(0.0, |acc, x| acc + x.amount)
}

#[test]
fn test_get_account_balance_at() {
    let mut account = Account::new(
        "Test Account".to_string(),
        0.0,
        vec![CashFlow::new(
            Some("Test Cash Flow".to_string()),
            100.0,
            Some(Frequency::MonthStart),
            None,
            None,
            None,
        )],
        NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2020, 12, 31).unwrap(),
    );
    let balance = get_account_balance_at(
        account.clone(),
        NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
    );
    assert_eq!(balance, 100.0);
    let balance = get_account_balance_at(
        account.clone(),
        NaiveDate::from_ymd_opt(2020, 1, 2).unwrap(),
    );
    assert_eq!(balance, 100.0);
    let balance = get_account_balance_at(
        account.clone(),
        NaiveDate::from_ymd_opt(2020, 1, 31).unwrap(),
    );
    assert_eq!(balance, 100.0);
    let balance = get_account_balance_at(
        account.clone(),
        NaiveDate::from_ymd_opt(2020, 2, 1).unwrap(),
    );
    assert_eq!(balance, 200.0);
    let balance = get_account_balance_at(
        account.clone(),
        NaiveDate::from_ymd_opt(2020, 2, 2).unwrap(),
    );
    assert_eq!(balance, 200.0);
    let balance = get_account_balance_at(
        account.clone(),
        NaiveDate::from_ymd_opt(2020, 2, 29).unwrap(),
    );
    assert_eq!(balance, 200.0);
    let balance = get_account_balance_at(
        account.clone(),
        NaiveDate::from_ymd_opt(2020, 3, 1).unwrap(),
    );
    assert_eq!(balance, 300.0);
    let balance = get_account_balance_at(
        account.clone(),
        NaiveDate::from_ymd_opt(2020, 3, 2).unwrap(),
    );
    assert_eq!(balance, 300.0);
    let balance = get_account_balance_at(
        account.clone(),
        NaiveDate::from_ymd_opt(2020, 3, 31).unwrap(),
    );
    assert_eq!(balance, 300.0);
}
