use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::env;
use std::iter::repeat;

fn pick_monster(mut rng: &mut ThreadRng, monsters: &[i32], queue: &[i32], ignore_queue: bool) -> i32 {
    let pick = monsters.choose(&mut rng).unwrap();
    if ignore_queue && *pick == 1 {
        1
    } else if !queue.contains(pick) {
        *pick
    } else {
        let r: f32 = rng.gen();
        if r < 0.25 {
            *pick
        } else {
            pick_monster(rng, monsters, queue, ignore_queue)
        }
    }
}

fn add_queue(queue: &mut Vec<i32>, pick: i32) {
    if queue.len() == 5 {
        queue.remove(0);
    }
    queue.push(pick);
}

fn calc(n: i32) -> f32 {
    let mut queue: Vec<i32> = Vec::with_capacity(5);
    let encounters_in_zone = 3;
    let additional_copies = 3;
    let ignore_queue = true;
    let monsters: Vec<i32> = repeat(1).take(additional_copies).chain(1..={encounters_in_zone}).collect();

    let mut rng = thread_rng();
    let mut count = 0;
    for _ in 0..n {
        let pick = pick_monster(&mut rng, &monsters, &queue, ignore_queue);
        if pick == 1 {
            count += 1;
        }
        add_queue(&mut queue, pick);
    }
    count as f32 / n as f32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_iter = 1_000_000;
    let iterations: i32 = if args.len() < 2 {
        default_iter
    } else {
        args[1].parse().unwrap_or(default_iter)
    };
    let avg = calc(iterations);
    println!("{}", avg);
}
