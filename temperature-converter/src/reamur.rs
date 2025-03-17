use crate::constant;

pub fn convert(unit: &str, temp: f32) -> f32 {
    match unit {
        "Celcius" => to_celcius(temp),
        "Fahrenheit" => to_fahrenheit(temp),
        "Kelvin" => to_kelvin(temp),
        _ => panic!("Invalid unit {}", unit),
    }
}

fn to_celcius(temp: f32) -> f32 {
    return temp * constant::REAMUR_5_4;
}

fn to_fahrenheit(temp: f32) -> f32 {
    return temp * 9.0 / 4.0 + constant::FAHRENHEIT_32;
}

fn to_kelvin(temp: f32) -> f32 {
    return temp * constant::REAMUR_5_4 + constant::KELVIN_273;
}
