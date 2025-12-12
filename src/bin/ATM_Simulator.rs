fn main() {
    // Defining Boolean variables for user state
    let access_card = false;
    let is_on_duty = true;
    let fingerprint_scan = true;
    let retina_scan = false;

    // Gate 1: Requires both conditions (card + shift)
    let gate_1: bool = access_card && is_on_duty;

    let gate_2: bool = fingerprint_scan || retina_scan;

    // Checking the different cases based on the result of the two gates
    if gate_1 && gate_2 {
        println!("Full Access Granted"); 
    } else if !gate_1 && !gate_2 {
        println!("Failed at Gate 1"); 
    } else if gate_1 && !gate_2 {
        println!("Failed at Gate 2"); 
    } else if !gate_1 && gate_2 {
        println!("How did you get through the first gate and get here?,You will be blocked and denied entry!")
    }
}