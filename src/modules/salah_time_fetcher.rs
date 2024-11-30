use std::env;

use salah::prelude::*;

pub fn calculate_current_salah() -> Prayer {
    let latitude = env::var("LAT").expect("LAT env is missing").parse::<f64>().expect("LAT is not number");
    let longitude = env::var("LONG").expect("LONG env is missing").parse::<f64>().expect("LONG is not number");

    let my_location = Coordinates::new(latitude, longitude);

    let date = Local::now().date_naive();
    let params = Configuration::with(Method::MuslimWorldLeague, Madhab::Hanafi);
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(my_location)
        .with_configuration(params)
        .calculate()
        .expect("Failed to calculate prayer schedule");

    return prayers.current();
}
