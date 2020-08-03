use plotters::prelude::*;
use std::env;
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[2])?;
    let reader = BufReader::with_capacity(1000000, file);

    let mut data = vec![];
    for line in reader.lines() {
        let float = line.unwrap().parse::<f64>().unwrap() * 100.0;
        data.push(float as i32);
    }
    let len = data.len() as u32;

    let root = BitMapBackend::new(&args[1], (1280, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0).into_font())
        .build_ranged(0i32..1i32, 0u32..(len / 5))?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .line_style_1(&WHITE.mix(0.3))
        .x_label_offset(30)
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15).into_font())
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|x: &i32| (*x, 1))),
    )?;

    Ok(())
}
