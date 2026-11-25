use chrono::{Datelike, Local, Weekday};

use crate::i18n::Language;

pub fn today_label(language: Language) -> String {
    let today = Local::now();
    let weekday = match (language, today.weekday()) {
        (Language::English, Weekday::Mon) => "Monday",
        (Language::English, Weekday::Tue) => "Tuesday",
        (Language::English, Weekday::Wed) => "Wednesday",
        (Language::English, Weekday::Thu) => "Thursday",
        (Language::English, Weekday::Fri) => "Friday",
        (Language::English, Weekday::Sat) => "Saturday",
        (Language::English, Weekday::Sun) => "Sunday",
        (Language::Spanish, Weekday::Mon) => "lunes",
        (Language::Spanish, Weekday::Tue) => "martes",
        (Language::Spanish, Weekday::Wed) => "miércoles",
        (Language::Spanish, Weekday::Thu) => "jueves",
        (Language::Spanish, Weekday::Fri) => "viernes",
        (Language::Spanish, Weekday::Sat) => "sábado",
        (Language::Spanish, Weekday::Sun) => "domingo",
    };
    let month = match (language, today.month()) {
        (Language::English, 1) => "January",
        (Language::English, 2) => "February",
        (Language::English, 3) => "March",
        (Language::English, 4) => "April",
        (Language::English, 5) => "May",
        (Language::English, 6) => "June",
        (Language::English, 7) => "July",
        (Language::English, 8) => "August",
        (Language::English, 9) => "September",
        (Language::English, 10) => "October",
        (Language::English, 11) => "November",
        (Language::English, _) => "December",
        (Language::Spanish, 1) => "enero",
        (Language::Spanish, 2) => "febrero",
        (Language::Spanish, 3) => "marzo",
        (Language::Spanish, 4) => "abril",
        (Language::Spanish, 5) => "mayo",
        (Language::Spanish, 6) => "junio",
        (Language::Spanish, 7) => "julio",
        (Language::Spanish, 8) => "agosto",
        (Language::Spanish, 9) => "septiembre",
        (Language::Spanish, 10) => "octubre",
        (Language::Spanish, 11) => "noviembre",
        (Language::Spanish, _) => "diciembre",
    };
    match language {
        Language::English => format!("{weekday}, {month} {}, {}", today.day(), today.year()),
        Language::Spanish => format!("{weekday}, {} de {month} de {}", today.day(), today.year()),
    }
}
