use minigrep::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    match run(config) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Application error: {}", err);
            process::exit(1);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy_more() {
        let x = 3;
        assert_eq!(x, 4);
    }
}
