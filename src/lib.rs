use chrono::Utc;

fn get_seed() -> u64 {
    let now = Utc::now();
    now.timestamp_nanos() as u64 // fallback to `.timestamp()` if needed
}

fn pick(seed: &mut u64, pool: &[char]) -> char {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1); // LCG
    let index = (*seed % pool.len() as u64) as usize;
    pool[index]
}

pub fn create_urbit_name() -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let consonants = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'y', 'z',
    ];

    let mut seed = get_seed();

    format!(
        "~{}{}{}{}{}{}-{}{}{}{}{}{}",
        pick(&mut seed, &consonants),
        pick(&mut seed, &vowels),
        pick(&mut seed, &consonants),
        pick(&mut seed, &consonants),
        pick(&mut seed, &vowels),
        pick(&mut seed, &consonants),
        pick(&mut seed, &consonants),
        pick(&mut seed, &vowels),
        pick(&mut seed, &consonants),
        pick(&mut seed, &consonants),
        pick(&mut seed, &vowels),
        pick(&mut seed, &consonants),
    )
}
