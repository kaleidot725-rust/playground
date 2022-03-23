
pub fn execute() {
    println!("-- PANIC TEST --");

    // crewSize を 0 にすると Panic が発生する
    pirate_share(32, 2);

}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}