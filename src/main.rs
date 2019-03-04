use rand_os::OsRng;
use rand_os::rand_core::RngCore;

fn main() {
    let mut os_rng = OsRng::new().unwrap();
    let mut pool_size = 69;
    let mut pool: Vec<u32> = (1..pool_size + 1).collect();
    let mut results: Vec<u32> = Vec::new();

    for _ in 0..5 {
        let index = (os_rng.next_u32() % pool_size) as usize;
        let num = pool[index];

        results.push(num);

        pool.remove(index);
        pool_size = pool_size - 1;
    }

    results.sort();

    for num in results {
        print!("{} ", num);
    }

    println!("{}", os_rng.next_u32() % 26 + 1);
}
