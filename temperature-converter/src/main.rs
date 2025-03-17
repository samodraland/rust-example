mod celcius;
mod constant;
mod fahrenheit;
mod kelvin;
mod reamur;

type Converter = fn(&str, f32) -> f32;

fn scales() -> [(&'static str, Converter, [&'static str; 3]); 4] {
    [
        //Tuples in array
        (
            "Celcius",
            celcius::convert,
            ["Fahrenheit", "Kelvin", "Reamur"],
        ),
        (
            "Fahrenheit",
            fahrenheit::convert,
            ["Celcius", "Kelvin", "Reamur"],
        ),
        (
            "Kelvin",
            kelvin::convert,
            ["Celcius", "Fahrenheit", "Reamur"],
        ),
        (
            "Reamur",
            reamur::convert,
            ["Fahrenheit", "Celcius", "Kelvin"],
        ),
    ]
}

fn main() {
    let temperature: f32 = 100.0;
    let scales_list = scales();

    for (unit_name, fn_name, targets) in scales_list.iter() {
        for target in targets.iter() {
            let result = fn_name(target, temperature);
            println!(
                "{}° in {} is {:.2}° in {}",
                temperature, unit_name, result, target
            );
        }
        println!("------------------------------------------");
    }
}
