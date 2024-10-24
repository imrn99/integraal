use csv::ReaderBuilder;
use integraal::{ComputeMethod, DomainDescriptor, FunctionDescriptor, Integraal};
use std::path::Path;

const RMAX: f64 = 41.;

fn main() {
    let (domain, speed_values) = parse_csv("examples/anemometry.csv");

    // we cannot integrate directly using the speed values
    // we need to compose the integrated term since this is a weighted sum

    let values = speed_values
        .iter()
        .zip(domain.iter())
        .map(|(s, r)| *s * (*r))
        .collect();

    let mut integral = Integraal::default()
        .domain(DomainDescriptor::Explicit(&domain))
        .function(FunctionDescriptor::Values(values))
        .method(ComputeMethod::Trapezoid);

    let mut res = integral.compute().unwrap();

    res *= 2.0 / RMAX.powi(2);

    println!("average fluid speed at pipe section: {res:.3} m/s");
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
        .find_map(|(id, s)| if *s == *"Average" { Some(id) } else { None })
        .unwrap();
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
