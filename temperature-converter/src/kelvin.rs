use crate::constant;

pub fn convert(unit: &str, temp: f32) -> f32 {
    match unit {
        "Celcius" => to_celcius(temp),
        "Fahrenheit" => to_fahrenheit(temp),
        "Reamur" => to_reamur(temp),
        _ => panic!("Invalid unit {}", unit),
    }
}

fn to_celcius(temp: f32) -> f32 {
    return temp - constant::KELVIN_273;
}

fn to_fahrenheit(temp: f32) -> f32 {
    return (temp - constant::KELVIN_273) * constant::FAHRENHEIT_9_5 + constant::FAHRENHEIT_32;
}

fn to_reamur(temp: f32) -> f32 {
    return (temp - constant::KELVIN_273) * constant::KELVIN_4_5;
}
