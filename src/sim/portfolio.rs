use super::cash::Account;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// `Asset` represents a financial asset that can be invested in.
///
/// Mean return and standard deviation are used to calculate the return of the asset. These
/// are annual values.
///
/// # Example
///
/// ```
/// use budget::sim::portfolio::Asset;
///
/// let asset = Asset::new("Asset1".to_string(), 0.1, 0.05);
/// ```
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct Asset {
    pub name: String,
    pub mean_return: f64,
    pub std_dev: f64,
}

impl Asset {
    pub fn new(name: String, mean_return: f64, std_dev: f64) -> Asset {
        Asset {
            name,
            mean_return,
            std_dev,
        }
    }
}

/// `Portfolio` represents a collection of `Asset`s and their respective weights in the portfolio.
///
/// Each `Asset` in the `assets` vector corresponds to a weight in the `weights` vector.
/// The weight represents the proportion of the portfolio's value that is invested in the asset.
///
/// # Example
///
/// ```
/// use budget::sim::portfolio::{Asset, Portfolio};
///
/// let assets = vec![
///     Asset::new("Asset1".to_string(), 0.1, 0.05),
///     Asset::new("Asset2".to_string(), 0.2, 0.1),
/// ];
/// let weights = vec![0.5, 0.5];
/// let portfolio = Portfolio::new(assets, weights);
/// ```
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Portfolio {
    pub assets: Vec<Asset>,
    pub weights: Vec<f64>,
}

impl Portfolio {
    pub fn new(assets: Vec<Asset>, weights: Vec<f64>) -> Portfolio {
        Portfolio { assets, weights }
    }
}

pub trait Invest {
    fn invest(&mut self, portfolio: &Portfolio) -> f64;

    fn invest_asset(&mut self, asset: &Asset, weight: &f64) -> f64;
}

impl Invest for Account {
    fn invest(&mut self, portfolio: &Portfolio) -> f64 {
        let ai = portfolio.assets.iter();
        let wi = portfolio.weights.iter();
        let it = ai.zip(wi);

        let mut income: f64 = 0.0;
        for (a, w) in it {
            income += self.invest_asset(a, w);
        }

        self.balance += income;

        income
    }

    fn invest_asset(&mut self, asset: &Asset, weight: &f64) -> f64 {
        (weight * self.balance) * (asset.mean_return + (asset.std_dev * rand::random::<f64>()))
    }
}

#[cfg(test)]
mod invest_tests {
    use super::*;

    #[test]
    fn test_invest() {
        let mut account = Account::new(
            "test".to_string(),
            1000.0,
            vec![],
            chrono::NaiveDate::from_ymd_opt(2018, 1, 1).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2018, 12, 31).unwrap(),
        );
        let portfolio =
            Portfolio::new(vec![Asset::new("Asset 1".to_string(), 0.1, 0.0)], vec![1.0]);
        account.invest(&portfolio);
        assert_eq!(account.balance, 1100.0);
    }

    #[test]
    fn test_invest_two_assets() {
        let mut account = Account::new(
            "test".to_string(),
            1000.0,
            vec![],
            chrono::NaiveDate::from_ymd_opt(2018, 1, 1).unwrap(),
            chrono::NaiveDate::from_ymd_opt(2018, 12, 31).unwrap(),
        );
        let portfolio = Portfolio::new(
            vec![
                Asset::new("Asset 1".to_string(), 0.1, 0.0),
                Asset::new("Asset 2".to_string(), 0.2, 0.0),
            ],
            vec![0.5, 0.5],
        );
        account.invest(&portfolio);
        assert_eq!(account.balance, 1150.0);
    }
}
