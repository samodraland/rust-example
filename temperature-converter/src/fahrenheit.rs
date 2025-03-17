use crate::constant;

pub fn convert(unit: &str, temp: f32) -> f32 {
    match unit {
        "Celcius" => to_celcius(temp),
        "Kelvin" => to_kelvin(temp),
        "Reamur" => to_reamur(temp),
        _ => panic!("Invalid unit {}", unit),
    }
}

fn to_celcius(temp: f32) -> f32 {
    return (temp - constant::FAHRENHEIT_32) * constant::FAHRENHEIT_5_9;
}

fn to_kelvin(temp: f32) -> f32 {
    return (temp - constant::FAHRENHEIT_32) * constant::FAHRENHEIT_5_9 + constant::KELVIN_273;
}

fn to_reamur(temp: f32) -> f32 {
    return (temp - constant::FAHRENHEIT_32) * 4.0 / 9.0;
}
