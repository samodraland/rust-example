use crate::constant;

pub fn convert(unit: &str, temp: f32) -> f32 {
    match unit {
        "Fahrenheit" => to_fahrenheit(temp),
        "Kelvin" => to_kelvin(temp),
        "Reamur" => to_reamur(temp),
        _ => panic!("Invalid unit {}", unit),
    }
}

fn to_fahrenheit(temp: f32) -> f32 {
    return temp * constant::FAHRENHEIT_9_5 + constant::FAHRENHEIT_32;
}
fn to_kelvin(temp: f32) -> f32 {
    return temp + constant::KELVIN_273;
}
fn to_reamur(temp: f32) -> f32 {
    return temp * constant::KELVIN_4_5;
}
