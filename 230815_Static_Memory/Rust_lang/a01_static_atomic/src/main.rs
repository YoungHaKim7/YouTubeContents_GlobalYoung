use std::sync::atomic::AtomicUsize;

static mut LEVELS: AtomicUsize = AtomicUsize::new(0);

fn bump_levels_safe() {
    let old_levels = unsafe {
        let levels = LEVELS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        levels
    };
    println!("bump_levels to {}", old_levels);
}

fn main() {
    bump_levels_safe();
    bump_levels_safe();
    bump_levels_safe();
    bump_levels_safe();
}
