// meters
const EARTH_RADIUS: Meters = Meters(6_371_000.0);

/*
*
* let gps = Gps {
        latitude: 51.1166662,
        longitude: 17.0333332,
        altitude: 1.0,
        speed: 100.0,
    };
*
* */
fn main() {
    let mut lat: f64 = 52.00882034091296;
    let mut lon: f64 = 17.0333332;
    let bearing: Degrees = Degrees(15.0);
    let distance = calculate_distance(27.7777, 3.0);
    println!("distance {:?}", distance);

    for _ in 1..101 {
        let new_lat = calculate_new_latitude(lat.to_radians(), distance, bearing.to_radians());
        let new_lon = calculate_new_longitude(
            lon.to_radians(),
            distance,
            bearing.to_radians(),
            lat.to_radians(),
            new_lat.to_radians(),
        );
        println!("lat: {}", new_lat);
        println!("lon: {}", new_lon);
        lat = new_lat;
        lon = new_lon;
    }
}

#[derive(Debug, Clone, Copy)]
struct Degrees(pub f64);

impl Degrees {
    fn to_radians(self) -> Radians {
        Radians(self.0.to_radians())
    }
}

#[derive(Debug, Clone, Copy)]
struct Meters(pub f64);

#[derive(Debug, Clone, Copy)]
struct Radians(pub f64);

impl Radians {
    fn cos(self) -> f64 {
        self.0.cos()
    }

    fn sin(self) -> f64 {
        self.0.sin()
    }
}

fn calculate_distance(speed: f64, time: f64) -> Meters {
    Meters(speed * time)
}

fn calculate_new_latitude(latitude: f64, distance: Meters, bearing: Radians) -> f64 {
    let distance_f64 = distance.0;
    let earth_radius_f64 = EARTH_RADIUS.0;
    let distance_over_earth_radius = distance_f64 / earth_radius_f64;
    let term1 = latitude.sin() * distance_over_earth_radius.cos();
    let term2 = latitude.cos() * distance_over_earth_radius.sin() * bearing.cos();
    let inside_arcsin = (term1 + term2).clamp(-1.0, 1.0);
    // Calculations were in radians, convert back to degrees
    inside_arcsin.asin().to_degrees()
}

fn calculate_new_longitude(
    longitude: f64,
    distance: Meters,
    bearing: Radians,
    latitude1: f64,
    latitude2: f64,
) -> f64 {
    let distance_f64 = distance.0;
    let earth_radius_f64 = EARTH_RADIUS.0;
    let distance_over_earth_radius = distance_f64 / earth_radius_f64;
    let term1 = bearing.sin() * distance_over_earth_radius.sin() * latitude1.cos();
    let term2 = distance_over_earth_radius.cos() - latitude1.sin() * latitude2.sin();
    let atan2 = term1.atan2(term2);

    // Calculations were in radians, convert back to degrees
    (longitude + atan2).to_degrees()
}
