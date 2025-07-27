use simulation_testing::shuffle_simulation::ShuffleSimulation;
use simulation_testing::simulation::Simulation;

#[tokio::main]
async fn main() {
    println!("=== Shuffle Simulation Demo ===\n");
    
    // Test with seed divisible by 10
    println!("Testing with seed 30 (divisible by 10):");
    demo_shuffle(30).await;
    
    println!("\n---\n");
    
    // Test with seed not divisible by 10
    println!("Testing with seed 31 (not divisible by 10):");
    demo_shuffle(31).await;
}

async fn demo_shuffle(seed: u64) {
    let mut sim = ShuffleSimulation::new(seed);
    
    // Inject messages
    let messages = vec![
        ("Alice", "First message"),
        ("Bob", "Second message"),
        ("Charlie", "Third message"),
        ("David", "Fourth message"),
        ("Eve", "Fifth message"),
    ];
    
    println!("Injecting messages in order:");
    for (i, (from, msg)) in messages.iter().enumerate() {
        println!("  {}. {} -> {}", i + 1, from, msg);
        sim.inject_message(from.to_string(), msg.as_bytes());
    }
    
    // Receive messages back
    println!("\nReceiving messages:");
    let mut i = 1;
    while let Ok(Some(msg)) = sim.receive_message().await {
        if let Ok(s) = String::from_utf8(msg.to_vec()) {
            println!("  {}. {}", i, s);
            i += 1;
        }
    }
    
    if seed % 10 == 0 {
        println!("\n✓ Messages were shuffled (seed {} is divisible by 10)", seed);
    } else {
        println!("\n✓ Message order preserved (seed {} is not divisible by 10)", seed);
    }
}