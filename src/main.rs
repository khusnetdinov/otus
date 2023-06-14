use std::fmt;

enum DeviseStatus {
    On,
    Off
}

impl fmt::Display for DeviseStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeviseStatus::On => write!(f, "On"),
            DeviseStatus::Off => write!(f, "Off"),
        }
    }
}

trait Devise {
    fn info(&self) -> String;
    fn power(&mut self, new_status: DeviseStatus);
    fn usage_info(& self) -> u32;
}

struct Socket {
    info: String,
    power_usage: u32,
    status: DeviseStatus
}

impl fmt::Debug for Socket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Socket info: {}, power_usage: {}, status: {}", self.info, self.power_usage, self.status)
    }
}

struct Termometer {
    info: String,
    power_usage: u32,
    status: DeviseStatus,
    temperature: i16
}

impl fmt::Debug for Termometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Termometer info: {}, power_usage: {}, status: {}, temperature: {}", self.info, self.power_usage, self.status, self.temperature)
    }
}

impl Termometer {
    fn temperature_info(&self) -> i16 {
        self.temperature
    }
}

impl Devise for Socket {
    fn info(&self) -> String {
        return format!("Devise: {}. Power usage: DS {} WT.", self.info, self.power_usage);
    }
    fn power(&mut self, new_status: DeviseStatus) {
        self.status = new_status
    }

    fn usage_info(&self) -> u32 {
        match self.status {
            DeviseStatus::On => self.power_usage,
            DeviseStatus::Off => 0
        }
    }
}


impl Devise for Termometer {
    fn info(&self) -> String {
        return format!("Devise: {}. Power usage: DS {} WT.", self.info, self.power_usage);
    }

    fn power(&mut self, new_status: DeviseStatus) {
        self.status = new_status
    }

    fn usage_info(&self) -> u32 {
        match self.status {
            DeviseStatus::On => self.power_usage,
            DeviseStatus::Off => 0
        }
    }
}

fn main() {
    let mut socket: Socket = Socket {
        info: String::from("Smart Socket 2023"),
        power_usage: 12,
        status: DeviseStatus::On,
    };

    let mut termometer: Termometer = Termometer {
        info: String::from("Smart Termometer 2023"),
        power_usage: 1,
        status: DeviseStatus::On,
        temperature: 25
    };

    println!("{}", socket.info());
    println!("{}", termometer.info());
    println!("\n");

    println!("Socket: status: {}, power usage: {}v", socket.status, socket.usage_info());
    println!("Termometer: status: {}, temperature: {} С, power usage: {} WT", termometer.status, termometer.temperature_info(), termometer.usage_info());
    println!("\n");

    socket.power(DeviseStatus::Off);
    termometer.power(DeviseStatus::Off);

    println!("Socket: status: {}, power usage: {}v", socket.status, socket.usage_info());
    println!("Termometer: status: {}, temperature: {} С, power usage: {} WT", termometer.status, termometer.temperature_info(), termometer.usage_info());
    println!("\n");

    println!("{:?}", socket);
    println!("{}", socket.info());
    println!("{:?}", termometer);
    println!("{}", termometer.info());
}
