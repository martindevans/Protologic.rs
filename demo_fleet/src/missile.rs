use std::time::Duration;

use rand::{prelude::*, distributions::Uniform};
use protologic_core::{
    highlevel::actions::*,
    utils::*,
    radio::*,
    wasi::sched_yield,
};

use crate::turn_and_stop;

pub fn run()
{
    // Configure radio to receive all messages
    radio_receive_filter(0, 0);

    // Pulse the engines a bit to clear the ship
    engine_set_throttle(1.0);
    wait_time(Duration::from_secs_f32(0.1f32));
    engine_set_throttle(0.0);

    // Turn in a random direction for a random amount of time
    let mut rng = rand::thread_rng();
    let ticks = rng.gen_range(750..1500u32);
    let xyz = Uniform::new(-1f32, 1f32);
    let x = xyz.sample(&mut rng);
    let y = xyz.sample(&mut rng);
    let z = xyz.sample(&mut rng);
    turn_and_stop(x, y, z, ticks);

    // Pulse the engines to separate from other missiles
    engine_set_throttle(1.0);
    wait_time(Duration::from_secs_f32(0.1f32));
    engine_set_throttle(0.0);

    // Wait for a radio message
    let mut messages = Vec::new();
    loop {
        radio_receive(&mut messages);
        sched_yield();

        if messages.len() > 0 {
            let pos = crate::radio::unpack_message(messages[0]);
            todo!("Turn to target: {:?}", pos);
        }
    }
}