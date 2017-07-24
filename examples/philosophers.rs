extern crate rand;

#[macro_use]
extern crate lazy_static;

use std::thread;
use std::time;
use std::sync::{Mutex};
use rand::Rng;

// Each philosopher runs as a different thread
// Each philosopher tries to acquire their lower fork.
//    if they get it:
//		try to acquire their higher fork
//      if they get it:
//          Eat for a while, and then release both forks
//      else:
//          release the lower fork as well
//    else:
//      wait for a while and try again

struct Philosopher {
	id: &'static str,
	first_fork: &'static Mutex<i32>,
	second_fork: &'static Mutex<i32>,
}

fn main() {
	lazy_static! {
		static ref FORK1: Mutex<i32> = Mutex::new(1);
		static ref FORK2: Mutex<i32> = Mutex::new(2);
		static ref FORK3: Mutex<i32> = Mutex::new(3);

		static ref PHILOSOPHER_INFO_1: Philosopher = Philosopher {id: "1", first_fork: &FORK1, second_fork: &FORK2};
		static ref PHILOSOPHER_INFO_2: Philosopher = Philosopher {id: "2", first_fork: &FORK2, second_fork: &FORK3};
		static ref PHILOSOPHER_INFO_3: Philosopher = Philosopher {id: "3", first_fork: &FORK1, second_fork: &FORK3};
	}

	let philosopher1: std::thread::JoinHandle<i32> = start_philosopher(&PHILOSOPHER_INFO_1);
	let philosopher2: std::thread::JoinHandle<i32> = start_philosopher(&PHILOSOPHER_INFO_2);
	let philosopher3: std::thread::JoinHandle<i32> = start_philosopher(&PHILOSOPHER_INFO_3);

	philosopher1.join().expect("Philosopher 1 failed");
	philosopher2.join().expect("Philosopher 2 failed");
	philosopher3.join().expect("Philosopher 3 failed");
}

fn start_philosopher(context: &'static Philosopher) -> std::thread::JoinHandle<i32> {
	return thread::spawn(move || {
		let mut rng: rand::ThreadRng = rand::thread_rng();

		loop {
			waitForAWhile(&mut rng);
		    let lock1 = context.first_fork.try_lock();
		    if let Ok(_) = lock1 {
	    		get_second_fork(context, &mut rng);
	    	} 
		}
	});
}

fn get_second_fork(context: &'static Philosopher, rng: &mut rand::ThreadRng) {
	let lock2 = context.second_fork.try_lock();

	if let Ok(_) = lock2 {
		println!("{}", context.id);
		waitForAWhile(rng);
	}
}

fn waitForAWhile(rng: &mut rand::ThreadRng) {
	thread::sleep(time::Duration::from_millis(rng.gen_range(5, 10)))
}
