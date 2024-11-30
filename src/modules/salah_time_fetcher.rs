use salah::prelude::*;

pub fn calculate_current_salah() -> Prayer {
    let my_location = Coordinates::new(40.48482, 71.76167);

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
