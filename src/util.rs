use std::time::Instant;

pub fn time_function(name: &str, input: String, f: fn(String) -> String) {
    let start = Instant::now();
    let result = f(input);
    let duration = start.elapsed();
    print!("{} result: \'{}\'", name, result);
    println!(", time elapsed: {:?}", duration);
}
