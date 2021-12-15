use std::error::Error;

use cli_candlestick_chart::{Candle, Chart};

fn main() -> Result<(), Box<dyn Error>> {
    // Your CSV data must have "open,high,low,close" header fields.
    let mut rdr = csv::Reader::from_path("./examples/BTC-USD.csv")?;

    let mut candles: Vec<Candle> = Vec::new();

    for result in rdr.deserialize() {
        let candle: Candle = result?;
        candles.push(candle);
    }

    let mut chart = Chart::new(&candles);

    // Set the chart title
    chart.set_name(String::from("BTC/USDT"));

    chart.draw();

    Ok(())
}
