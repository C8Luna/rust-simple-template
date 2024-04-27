use std::{
    fmt::format,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

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

    //simple html server to serve the html
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn get_weather() -> Vec<DailyWeather> {
    vec![
        ("Monday", "Sunny").into(),
        ("Tuesday", "Rainy").into(),
        ("Wednesday", "Cloudy").into(),
        ("Thursday", "Sunny").into(),
        ("Friday", "Rainy").into(),
        ("Saturday", "Cloud)y").into(),
        ("Sunday", "Sunny").into(),
    ]
}
fn render_weather_page() -> String {
    let forecast = get_weather();
    let weather_page = WeatherPage::new(forecast);
    weather_page.render()
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    //need to do partial read in a loop to fill the buffer
    stream.read(&mut buffer).unwrap();
    let request = match std::str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 : {}", e),
    };
    println!("Received request: {}", request);

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
        render_weather_page()
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
// let exp_template = format!(include_str!("./mytempl.txt"), message = message);
