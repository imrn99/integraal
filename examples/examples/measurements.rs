use csv::ReaderBuilder;
use integraal::{ComputeMethod, DomainDescriptor, FunctionDescriptor, Integraal};
use std::path::Path;

const RMAX: f64 = 41.;

#[rustfmt::skip]
fn main() {
    let (domain, speed_values) = parse_csv("examples/anemometry.csv");

    let values = speed_values
        .iter()
        .zip(domain.iter())
        .map(|(s, r)| *s * (*r))
        .collect();

    let mut integral = Integraal::default()
        .domain(DomainDescriptor::Explicit(&domain))
        .function(FunctionDescriptor::Values(values))
        .method(ComputeMethod::Trapezoid);

    let area = std::f64::consts::PI * RMAX.powi(2);
    let volume_velocity = 2.0 * std::f64::consts::PI * integral.compute().unwrap();

    // print results, with some unit shenanigans
    println!("results:");
    println!("   area of the section:      {:7.3} cm^2"  , area * 1.0e-2);
    println!("   volume velocity:          {:7.3} cm^3/s", volume_velocity * 1.0e-2);
    println!("   superficial velocity:     {:7.3} m/s"   , volume_velocity / area);
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
