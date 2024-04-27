pub trait Render {
    fn render(&self) -> String;
}
macro_rules! html {
    ($name:expr, $body:expr) => {
        format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta http-equiv="X-UA-Compatible" content="IE=edge">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>{}</title>
            </head>
            <body>
                {}
            </body>
            </html>
        "#,
            $name, $body
        )
    };
    () => {};
}

macro_rules! render_iter {
    ($iter:expr) => {
        $iter
            .iter()
            .map(|d| d.render())
            .collect::<Vec<String>>()
            .join("\n")
    };
}

pub struct DailyWeather {
    day: String,
    weather: String,
}
impl From<(&str, &str)> for DailyWeather {
    fn from((day, weather): (&str, &str)) -> Self {
        DailyWeather {
            day: day.to_string(),
            weather: weather.to_string(),
        }
    }
}
impl Render for DailyWeather {
    // fn render(&self) -> String {
    //     format!(
    //         include_str!("./mytempl.txt"),
    //         dayofweek = self.day,
    //         weather = self.weather
    //     )
    // }

    fn render(&self) -> String {
        format!(
            r#"The weather on {dayofweek} {weather}"#,
            dayofweek = self.day,
            weather = self.weather
        )
    }
}

pub struct WeatherPage {
    forecast: Vec<DailyWeather>,
}
impl WeatherPage {
    pub fn new(forecast: Vec<DailyWeather>) -> Self {
        WeatherPage { forecast }
    }
}
impl Render for WeatherPage {
    fn render(&self) -> String {
        html!("Weather Page", render_iter!(self.forecast))
    }
}
