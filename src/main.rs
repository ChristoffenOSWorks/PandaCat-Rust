extern crate toCairo;
extern crate rustyline;
    fn main() {
        if std::process::Command::new("clear").status().unwrap().success() {
            println!("\nTesting el cairo function");
            toCairo::cairo::to_cairo(21, 69, 96);
            let mut rl = rustyline::Editor::<()>::new();
            let readline = rl.readline(">> ");
        }
    }
