pub enum ValidCommands {
    Led(bool),
    PumpOnFor(u32),
}

pub struct Commands {}

impl Commands {
    fn led(parts: Vec<&str>) -> Result<ValidCommands, ()> {
        match parts.as_slice() {
            ["led", "on"] => {
                return Ok(ValidCommands::Led(true));
            }
            ["led", "off"] => {
                return Ok(ValidCommands::Led(false));
            }
            _ => {
                return Err(());
            }
        }
    }

    fn pump(parts: Vec<&str>) -> Result<ValidCommands, ()> {
        match parts.as_slice() {
            ["pump", "on", time] => {
                return Ok(ValidCommands::PumpOnFor(time.parse::<u32>().unwrap()));
            }
            _ => return Err(()),
        }
    }

    pub fn parse(command: String) -> Result<ValidCommands, ()> {
        let command = command.trim();
        let parts = command.split_whitespace().collect::<Vec<_>>();

        match parts[0] {
            "led" => Commands::led(parts),
            "pump" => Commands::pump(parts),
            _ => return Err(()),
        }
    }
}
