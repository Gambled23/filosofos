use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut philosophers = vec![];
    let forks = Arc::new(Mutex::new(vec![false; 5]));
    let eat_counter = Arc::new(Mutex::new(vec![0; 5]));

    for i in 0..5 {
        let forks = Arc::clone(&forks);
        let eat_counter = Arc::clone(&eat_counter);
        philosophers.push(thread::spawn(move || {
            let lower = std::cmp::min(i, (i + 1) % 5);
            let higher = std::cmp::max(i, (i + 1) % 5);
            while eat_counter.lock().unwrap()[i] < 6 {
                {
                    let mut forks = forks.lock().unwrap();
                    if !forks[lower] && !forks[higher] {
                        forks[lower] = true;
                        forks[higher] = true;

                        let mut eat_counter = eat_counter.lock().unwrap();
                        eat_counter[i] += 1;
                        println!("Philosopher {} is eating. Eat count: {}", i, eat_counter[i]);
                    } else {
                        continue;
                    }
                }
                thread::sleep(std::time::Duration::from_secs(1));

                {
                    let mut forks = forks.lock().unwrap();
                    forks[lower] = false;
                    forks[higher] = false;
                }

                println!("Philosopher {} finished eating and is now thinking", i);
                thread::sleep(std::time::Duration::from_secs(1));
            }
        }));
    }

    for philosopher in philosophers {
        philosopher.join().unwrap();
    }
}