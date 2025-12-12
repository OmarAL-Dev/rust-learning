fn main() {
    // 1. Define Player Stats
    let is_silenced: bool = false;
    let mana: i32 = 60;
    let health: i32 = 40;
    let has_ancient_amulet: bool = true;
    let lucky_number: i32 = 77;

    // 2. Logic Check
    let magic_shield: bool = !is_silenced && ((mana > 50 && health < 30) || (has_ancient_amulet && lucky_number % 2 != 0));

    // 3. Print Result
    println!("Activate the magic shield : {}", magic_shield);
}