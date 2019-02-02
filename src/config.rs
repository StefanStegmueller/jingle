pub struct Config {
    pub filename: String,
    pub duty_cycle: u32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        if args.len() > 3 {
            return Err("too much arguments");
        }

        let filename = args[1].clone();
        let duty_cycle = args[2].clone().parse::<u32>().unwrap();

        Ok(Config {
            filename,
            duty_cycle,
        })
    }
}
