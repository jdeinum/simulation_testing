use anyhow::{Context, Result};
use core::panicking::panic;
use rand::{SeedableRng, rngs::SmallRng};

/// simulation contains any external, non-deterministic code
/// righht now we are pretending serializing is free and doesn't exist
#[async_trait::async_trait]
pub trait Simulation<T: SeedableRng> {
    async fn send_message(&mut self, rng: &mut T, message: Message) -> Result<()>;
    async fn receive_message(&mut self, rng: &mut T) -> Result<Message>;
}

pub enum Message {
    Number(i32),
    Launch,
    Received(usize),
}

/// Solver represents our deterministic business logic
pub struct BigNumberLauncher {
    pub count_gt_10: usize,
}

impl BigNumberLauncher {
    // Launch
    pub fn process_message(&mut self, msg: &Message) -> Result<()> {
        match msg {
            Message::Number(x) => {
                if *x > 10 {
                    self.count_gt_10 += 1
                }
            }

            // for some reason, if we have received exactly 9 numbers gt 10 when we launch
            // it blows up
            Message::Launch => {
                if self.count_gt_10 == 9 {
                    panic!("what da heck");
                }
            }
            _ => {}
        }

        Ok(())
    }
}

/// Our system is the combination of our deterministic logic and non-deterministic simulation
pub struct System<T, S>
where
    T: SeedableRng,
    S: Simulation<T>,
{
    pub sim: S,
    pub b: BigNumberLauncher,
    pub rng: T,
}

impl<T, S> System<T, S>
where
    T: SeedableRng,
    S: Simulation<T>,
{
    pub async fn run(mut self) -> Result<()> {
        for _ in 0..10 {
            let msg = self.sim.receive_message(&mut self.rng).await?;
            self.b.process_message(&msg);
            self.sim
                .send_message(&mut self.rng, Message::Received)
                .await?;
        }
        Ok(())
    }
}

pub struct DeterministicSimulator {
    messages: Vec<Message>,
}

#[async_trait::async_trait]
impl Simulation<SmallRng> for DeterministicSimulator {
    // sends the message to the outside world
    // in this case, we can just assume it works fine
    // in a system that depends on two way comms, this is far more important
    async fn send_message(&mut self, _rng: &mut SmallRng, _message: &Message) -> Result<()> {
        Ok(())
    }

    // we want to control
    async fn receive_message(&mut self, _rng: &mut SmallRng) -> Result<Message> {}
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // pick a seed
    let seed: u64 = 1;

    // generate our RNG
    let rng = rand::rngs::SmallRng::seed_from_u64(seed);

    // create our system
    let sys = System {
        b: BigNumberLauncher { count_gt_10: 0 },
        sim: DeterministicSimulator {},
        rng,
    };

    sys.run().await.context("run system")
}
