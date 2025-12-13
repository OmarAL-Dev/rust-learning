fn main() {
    let mut fuel = 1100;
    let mut speed = 0;
    let mut altitude = 0;
    let mut seconds = 0;

    loop {
        // 1. Physics Engine: Update stats every second
        seconds += 1;
        fuel -= 20;
        speed += 40;
        altitude += speed / 10;

        // 2. Reporting: Print logs every 5 seconds only
        if seconds % 5 == 0 {
            println!("Elapsed time: {}s", seconds);
            println!("fuel: {}L", fuel);
            println!("speed: {} km/h", speed);
            println!("altitude: {} km\n", altitude);
        }

        // 3. Timeline Events: Handle specific moments
        match seconds {
            10 => {
                println!("Stage 1 Separation!\n");
                fuel -= 100; // Extra fuel consumption
            },
            30 => {
                println!("Entering Vacuum!\n");
                speed += 100; // Speed boost
            },
            50 => {
                println!("Orbit Reached! Success!\n");
                break;
            }
            _ => () // Do nothing for other seconds
        }

        // 4. Safety Checks (Win/Loss conditions)
        if fuel <= 0 {
            println!("Mission Failed: No Fuel!");
            break;
        }

        // Explosion Logic: High speed + High fuel mass = Danger!
        if speed > 1700 && fuel > 200 {
            println!("BOOM! Explosion!");
            break;
        }
    }
}