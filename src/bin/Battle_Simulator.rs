fn main() {
    // Initialize game state variables
    let mut hero_health = 100;
    let mut monster_health = 140;
    let mut mana = 0;

    // Battle Loop: Runs for a maximum of 20 rounds
    for round in 1..=20 {
        println!("Round : {}\n", round);
        
        mana += 2;

        // Hero's Turn: Check if we can perform a Critical Hit
        if mana >= 5 {
            mana -= 5;
            monster_health -= 30;
            println!("Critical Magic Hit!\n");
        } else {
            monster_health -= 12;
            println!("Hit!");
        }
        
        // Monster's Turn: Attacks only if still alive
        if monster_health > 0 {
            hero_health -= 15;
            println!("Monster attacks!\n");
        }

        // Display current stats
        println!("hero health: {}", hero_health);
        println!("monsterhealth: {}", monster_health);
        println!("mana: {}", mana);
        println!("===========================");

        // Check Lose Condition
        if hero_health <= 0 {
            println!("You Died! Game Over.");
            break;
        }

        // Check Win Condition
        if monster_health <= 0 {
            println!("Monster Defeated! You Win!");
            break;
        }

        // Time Limit Condition
        if round == 20 {
            println!("The monster escaped!\n")
        }
        
    }
}