use crate::templates::{DailyWeather, Render, WeatherPage};

mod templates;

fn main() {
    let forecast: Vec<DailyWeather> = vec![
        ("Monday", "Sunny").into(),
        ("Tuesday", "Rainy").into(),
        ("Wednesday", "Cloudy").into(),
        ("Thursday", "Sunny").into(),
        ("Friday", "Rainy").into(),
        ("Saturday", "Cloud)y").into(),
        ("Sunday", "Sunny").into(),
    ];
    let weather_page = WeatherPage::new(forecast);
    let weekly_weather = weather_page.render();

    println!("{}", weekly_weather);
}
// let exp_template = format!(include_str!("./mytempl.txt"), message = message);
