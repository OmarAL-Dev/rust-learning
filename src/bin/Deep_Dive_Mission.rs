fn main() {
    // 1. Mission Configuration (Initial State)
    let mut current_depth = 0;
    let mut oxygen = 100;
    let mut battery = 100;
    let mut pressure_shield = 100;

    // 2. Main Game Loop (Dive until target)
    while current_depth < 3000 {

        println!("current depth: {}m", current_depth);
        println!("oxygen: {}%", oxygen);
        println!("battery: {}%", battery);
        println!("pressure shield: {}%\n", pressure_shield);

        // Physics Engine: Update resources
        current_depth += 100;
        oxygen -= 3;
        battery -= 2;

        // High Pressure Zone Logic
        if current_depth > 1500 {
            pressure_shield -= 5;
            battery -= 2;
        }

        // Random Events & Encounters
        if current_depth == 1000 {
            println!("Passing the Twilight Zone");
        } else if current_depth == 2000 {
            println!("Shark Attack!");
            pressure_shield -= 20;
            battery -= 10;
        } else if current_depth == 2500 {
            println!("Critical Pressure Alert!");
        }

        // Survival Checks (Game Over Conditions)
        if oxygen <= 0 {
            println!("Crew suffocated! Game Over!.");
            break;
        }
        
        if battery <= 0 {
            println!("Power Failure! Stranded forever!.");
            break;
        }
        
        if pressure_shield <= 0 {
            println!("Implosion! Submarine crushed!.");
            break;
        }      
    }

    // 3. Mission Report
    if current_depth >= 3000 {
        println!("Success! Wreckage found at 3000m. Mission Accomplished!");
    }
}