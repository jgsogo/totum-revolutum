use openexchangerates::models::{CurrenciesResponse, ExchangeRatesResponse, UsageResponse};
use serde::Serialize;

pub trait Output {
    fn print_exchange_rates(&self, data: ExchangeRatesResponse);
    fn print_usage(&self, data: UsageResponse);
    fn print_currencies(&self, data: CurrenciesResponse);
}

#[derive(clap::ValueEnum, Clone)]
pub enum OutputArg {
    /// Dumps all the JSON returned by OpenExchangeRates APIs
    JSON,

    /// Gives the output in an easy to parse format. Usually just one data point or one data per line.
    Porcelain,
}

pub enum OutputImpl {
    JSONOutput(JSONOutput),
    Porcelain(Porcelain),
}

impl OutputImpl {
    pub fn new(value: OutputArg) -> Self {
        match value {
            OutputArg::JSON => OutputImpl::JSONOutput(JSONOutput {}),
            OutputArg::Porcelain => OutputImpl::Porcelain(Porcelain {}),
        }
    }
}

impl Output for OutputImpl {
    fn print_exchange_rates(&self, data: ExchangeRatesResponse) {
        match self {
            OutputImpl::JSONOutput(v) => v.print_exchange_rates(data),
            OutputImpl::Porcelain(v) => v.print_exchange_rates(data),
        }
    }

    fn print_usage(&self, data: UsageResponse) {
        match self {
            OutputImpl::JSONOutput(v) => v.print_usage(data),
            OutputImpl::Porcelain(v) => v.print_usage(data),
        }
    }

    fn print_currencies(&self, data: CurrenciesResponse) {
        match self {
            OutputImpl::JSONOutput(v) => v.print_currencies(data),
            OutputImpl::Porcelain(v) => v.print_currencies(data),
        }
    }
}

pub struct Porcelain;

impl Output for Porcelain {
    fn print_exchange_rates(&self, data: ExchangeRatesResponse) {
        for rate in data.rates {
            println!("{} {}", rate.0, rate.1);
        }
    }

    fn print_usage(&self, data: UsageResponse) {
        println!("app_id {}", data.data.app_id);
        println!("status {}", data.data.status);
        println!("plan {}", data.data.plan.name);
        // Iterate all the features dynamically
        for feature in serde_json::to_value(data.data.plan.features)
            .unwrap()
            .as_object()
            .unwrap()
        {
            println!("features.{} {}", feature.0, feature.1);
        }
        println!("requests_quota {}", data.data.usage.requests_quota);
        println!("days_elapsed {}", data.data.usage.days_elapsed);
        println!("requests_remaining {}", data.data.usage.requests_remaining);
        println!("days_remaining {}", data.data.usage.days_remaining);
    }

    fn print_currencies(&self, data: CurrenciesResponse) {
        for ccy in data.currencies {
            println!("{} {}", ccy.0, ccy.1);
        }
    }
}

pub struct JSONOutput;

impl JSONOutput {
    fn pretty_print<T: Serialize>(obj: T) {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        obj.serialize(&mut ser).unwrap();
        println!("{}", String::from_utf8(buf).unwrap());
    }
}

impl Output for JSONOutput {
    fn print_exchange_rates(&self, data: ExchangeRatesResponse) {
        Self::pretty_print(data);
    }

    fn print_usage(&self, data: UsageResponse) {
        Self::pretty_print(data);
    }

    fn print_currencies(&self, data: CurrenciesResponse) {
        Self::pretty_print(data);
    }
}
