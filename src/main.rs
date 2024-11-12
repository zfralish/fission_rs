use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct ReactorStatus {
    power_output: f64,  // MW
    core_temp: f64,     // Celsius
    pressure: f64,      // Bar
    coolant_flow: f64,  // Percent
    control_rods: f64,  // Percent inserted
    emergency_shutdown: bool,
}

struct ReactorSimulator {
    status: ReactorStatus,
}

impl ReactorSimulator {
    fn new() -> Self {
        ReactorSimulator {
            status: ReactorStatus {
                power_output: 0.0,
                core_temp: 280.0,
                pressure: 150.0,
                coolant_flow: 100.0,
                control_rods: 100.0,
                emergency_shutdown: false,
            }
        }
    }

    fn adjust_control_rods(&mut self, position: f64) {
        self.status.control_rods = position.clamp(0.0, 100.0);
    }

    fn adjust_coolant_flow(&mut self, flow: f64) {
        self.status.coolant_flow = flow.clamp(0.0, 100.0);
    }

    fn calculate_power(&mut self) {
        let base_power = (100.0 - self.status.control_rods) * 20.0;  // Max 2000 MW
        let cooling_factor = self.status.coolant_flow / 100.0;
        self.status.power_output = base_power * cooling_factor;
    }

    fn update_temperature(&mut self) {
        let power_heat = self.status.power_output * 0.1;
        let cooling_effect = (self.status.coolant_flow / 100.0) * 30.0;
        let temp_change = power_heat - cooling_effect;
        self.status.core_temp += temp_change;
    }

    fn update_pressure(&mut self) {
        self.status.pressure = 150.0 + (self.status.core_temp - 280.0) * 0.5;
    }

    fn check_safety_systems(&mut self) -> bool {
        if self.status.core_temp > 1000.0 
            || self.status.pressure > 200.0 
            || self.status.coolant_flow < 10.0 {
            self.status.emergency_shutdown = true;
            self.status.control_rods = 100.0;
            return false;
        }
        true
    }

    fn simulate_timestep(&mut self) -> bool {
        if !self.status.emergency_shutdown {
            self.calculate_power();
            self.update_temperature();
            self.update_pressure();
            return self.check_safety_systems();
        }
        false
    }

    fn get_status(&self) -> &ReactorStatus {
        &self.status
    }
}

fn print_status(timestep: usize, status: &ReactorStatus) {
    println!("\nTimestep: {}", timestep);
    println!("Power Output: {:.2} MW", status.power_output);
    println!("Core Temperature: {:.2} °C", status.core_temp);
    println!("Pressure: {:.2} bar", status.pressure);
    println!("Coolant Flow: {:.2}%", status.coolant_flow);
    println!("Control Rods: {:.2}%", status.control_rods);
    
    if status.emergency_shutdown {
        println!("EMERGENCY SHUTDOWN ACTIVATED");
    }
}

fn main() {
    let mut reactor = ReactorSimulator::new();
    println!("Reactor Simulation Started");

    for timestep in 1..=10 {
        // Simulate operator actions
        match timestep {
            3 => reactor.adjust_control_rods(50.0), // Withdraw control rods to 50%
            6 => reactor.adjust_coolant_flow(80.0), // Reduce coolant flow to 80%
            _ => {}
        }

        // Run simulation step
        let safe = reactor.simulate_timestep();
        print_status(timestep, reactor.get_status());

        if !safe {
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }
}
