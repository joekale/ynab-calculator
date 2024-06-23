use std::{collections::HashMap, io};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

enum Stage {
    TaxRate,
    Entry
}

struct Item {
    price: Decimal,
    category: String,
    tax_rate: Decimal,
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stage: Option<Stage> = Some(Stage::TaxRate);
    let mut tax_rates: Vec<rust_decimal::Decimal> = Vec::new();
    let mut items: Vec<Item> = Vec::new();
    let mut splits: HashMap<String, rust_decimal::Decimal> = HashMap::new();
    while stage.is_some() {
        match stage {
            Some(Stage::TaxRate) => {
                println!("Enter Tax Rate");
                buffer.clear();
                io::stdin().read_line(&mut buffer)?;
                let rate = Decimal::from_str_exact(buffer.trim()).unwrap() / dec!(100.0);
                tax_rates.push(rate);
            },
            Some(Stage::Entry) => {
                println!("Enter Item Price");
                buffer.clear();
                io::stdin().read_line(&mut buffer)?;
                let price = Decimal::from_str_exact(buffer.trim()).unwrap();
                println!("Enter Category");
                buffer.clear();
                io::stdin().read_line(&mut buffer)?;
                let category = buffer.trim().to_lowercase();
                println!("Pick Tax Rate");
                for rate in tax_rates.iter().enumerate() {
                    println!("({}) {}", rate.0, rate.1);
                }
                buffer.clear();
                io::stdin().read_line(&mut buffer)?;
                let idx: usize = buffer.trim().parse().unwrap();
                items.push(Item {
                    price: price,
                    category: category,
                    tax_rate: tax_rates.get(idx).unwrap().clone()
                });
            },
            _ => {}
        }
        println!("Add another? (Y/n)");
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        if buffer.trim().to_lowercase() == "n" {
            stage = match stage {
                Some(Stage::TaxRate) => Some(Stage::Entry),
                Some(Stage::Entry) => None,
                _ => None
            }
        }
    }

    let mut tax_buckets: HashMap<Decimal, Decimal> = HashMap::new();

    for item in items {
        tax_buckets.insert(item.tax_rate, tax_buckets.get(&item.tax_rate).unwrap_or(&dec!(0.0)) + &item.price);
        splits.insert(item.category.clone(), splits.get(&item.category).unwrap_or(&dec!(0.0)) + item.price + item.price * item.tax_rate);
    }

    let mut total: Decimal = dec!(0.0);
    for tax_rate in tax_buckets.keys() {
        let price = tax_buckets.get(tax_rate).unwrap();
        total +=  price + price * tax_rate;
    }
    println!("Total: ${}", total.round_dp_with_strategy(2, RoundingStrategy::MidpointNearestEven));
    for key in splits.keys() {
        println!("{}: {}", key, splits.get(key).unwrap().round_dp_with_strategy(2, RoundingStrategy::MidpointNearestEven));
    }
    Ok(())
}