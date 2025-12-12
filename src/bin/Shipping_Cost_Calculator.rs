fn main() {
    // Input data
    let weight = 3.5;
    let zone = "rural";
    let is_premium_member = true;
    let has_free_shipping_coupon = false;

    let mut cost = 0.0;

    // 1. Calculate base cost based on weight tiers
    if weight < 1.0 {
        cost = 20.0;
    } else if weight > 1.0 && weight < 5.0 {
        cost = 50.0;
    } else if weight > 5.0 {
        cost = 100.0;
    }

    // 2. Apply location multipliers (Rural areas cost 50% more)
    if has_free_shipping_coupon {
        println!("Final cost: 0");
    } else if zone == "urban" {
        cost *= 1.0;
    } else if zone == "rural" {
        cost *= 1.5; 
    }

    // 3. Apply flat discount for premium members
    if is_premium_member {
        cost -= 20.0;
    }

    // Print final result if not free
    if !has_free_shipping_coupon {
        println!("Final cost: {}", cost);
    }
}