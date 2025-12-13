fn main() {
    let item_type = 2;
    let weight = 7;
    let is_owner_vip = true;

    // Match against a tuple (pair) to check two conditions at once
    match (item_type, is_owner_vip) {
        
        (1, _) => println!("Clothes: Allowed"),
        
        // Match Guard: Adds an extra condition (weight < 5) to this pattern
        (2, _) if weight < 5 => println!("Electronics: Allowed"),
        
        (2, _) => println!("Electronics: Denied (Too heavy)"), 

        (3, true) => println!("Liquids: Allowed (VIP)"),
        (3, false) => println!("Liquids: Denied"),
        
        (4, _) => println!("Food: Allowed (Check smell)"),
        (5, _) => println!("SECURITY ALERT! CALL POLICE!"),
        
        // Catch-all: Must be present to handle any undefined cases
        (_, _) => println!("Scan Error"),
    }
}