use quickcheck::TestResult;
use quickcheck_macros::quickcheck;
use simulation_testing::shuffle_simulation::ShuffleSimulation;
use simulation_testing::simulation::Simulation;
use simulation_testing::broadcast::BroadcastLayer;
use std::collections::HashSet;

#[quickcheck]
fn test_shuffle_behavior(seed: u64) -> TestResult {
    // Create a shuffle simulation with the given seed
    let mut sim = ShuffleSimulation::new(seed);
    
    // Inject some test messages
    let messages = vec![
        ("node1", "message1"),
        ("node2", "message2"),
        ("node3", "message3"),
        ("node4", "message4"),
        ("node5", "message5"),
    ];
    
    for (from, msg) in &messages {
        sim.inject_message(from.to_string(), msg.as_bytes());
    }
    
    // Collect the messages back
    let mut received_messages = Vec::new();
    // Use tokio runtime to run async code
    let rt = tokio::runtime::Runtime::new().unwrap();
    while let Ok(Some(msg)) = rt.block_on(sim.receive_message()) {
        if let Ok(s) = String::from_utf8(msg.to_vec()) {
            received_messages.push(s);
        }
    }
    
    // Check that we got all messages back
    if received_messages.len() != messages.len() {
        return TestResult::failed();
    }
    
    // Convert original messages to strings for comparison
    let original_messages: Vec<String> = messages.iter()
        .map(|(_, msg)| msg.to_string())
        .collect();
    
    // If seed is divisible by 10, order might be different
    if seed % 10 == 0 {
        // Check that all messages are present (order doesn't matter)
        let received_set: HashSet<_> = received_messages.iter().collect();
        let original_set: HashSet<_> = original_messages.iter().collect();
        
        if received_set != original_set {
            return TestResult::failed();
        }
        
        // For seeds divisible by 10, there's a high probability the order is different
        // (unless the shuffle happened to produce the same order)
        println!("Seed {} is divisible by 10 - messages were shuffled", seed);
    } else {
        // For non-divisible seeds, order should be preserved
        if received_messages != original_messages {
            return TestResult::failed();
        }
        println!("Seed {} is not divisible by 10 - order preserved", seed);
    }
    
    TestResult::passed()
}

#[quickcheck]
fn test_broadcast_with_shuffle(seed: u64) -> TestResult {
    // Create a shuffle simulation
    let sim = ShuffleSimulation::new(seed);
    
    // Create broadcast layer with some peers
    let peers = HashSet::from([
        "peer1".to_string(),
        "peer2".to_string(),
        "peer3".to_string(),
    ]);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut broadcast = rt.block_on(BroadcastLayer::build(peers, sim)).unwrap();
    
    // Send a test message
    if let Err(_) = rt.block_on(broadcast.broadcast("test message")) {
        return TestResult::failed();
    }
    
    // The broadcast should have sent to all peers
    let sent_count = broadcast.s.sent_messages.len();
    if sent_count != 3 {
        return TestResult::failed();
    }
    
    TestResult::passed()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quickcheck_runs() {
        // QuickCheck will automatically test with many random u64 values
        // This test just ensures our property tests compile and run
        println!("QuickCheck tests will run automatically");
    }
}