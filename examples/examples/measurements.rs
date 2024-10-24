use std::path::Path;

use csv::ReaderBuilder;
use integraal::{ComputeMethod, DomainDescriptor, FunctionDescriptor, Integraal};

fn main() {
    let (domain, speed_values) = parse_csv("examples/anemometry.csv");

    // we cannot integrate directly the speed values

    let values = todo!();

    let mut integral = Integraal::default()
        .domain(DomainDescriptor::Explicit(&domain))
        .function(FunctionDescriptor::Values(values))
        .method(ComputeMethod::Trapezoid);

    let res = integral.compute().unwrap();

    println!("average fluid speed at pipe section: {res:.10} m/s");
}

fn parse_csv(path: impl AsRef<Path>) -> (Vec<f64>, Vec<f64>) {
    let mut builder = ReaderBuilder::new();
    builder.has_headers(true).delimiter(b';');
    let mut reader = builder.from_path(path).unwrap();
    let radius_col = reader
        .headers()
        .unwrap()
        .iter()
        .enumerate()
        .find_map(|(id, s)| if *s == *"radius (mm)" { Some(id) } else { None })
        .unwrap();
    let speed_col = reader
        .headers()
        .unwrap()
        .iter()
        .enumerate()
        .find_map(|(id, s)| if *s == *"U" { Some(id) } else { None })
        .unwrap();
    println!("r: {radius_col:?}");
    println!("s: {speed_col:?}");
    reader
        .records()
        .filter_map(|r| {
            match (
                r.as_ref()
                    .unwrap()
                    .get(radius_col)
                    .unwrap()
                    .replace(",", ".")
                    .parse::<f64>(),
                r.as_ref()
                    .unwrap()
                    .get(speed_col)
                    .unwrap()
                    .replace(",", ".")
                    .parse::<f64>(),
            ) {
                (Ok(v1), Ok(v2)) => Some((v1, v2)),
                _ => None,
            }
        })
        .unzip()
}
