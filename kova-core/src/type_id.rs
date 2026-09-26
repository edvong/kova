use std::sync::atomic::{AtomicUsize, Ordering};

// Global counter to hand out unique IDs. Starts at 1 since 0 means "uninitialized".
static TAG_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[inline(always)]
pub fn type_id<T: 'static>() -> usize {
    // Dedicate a static atomic to each monomorphized type T.
    static TAG_ID: AtomicUsize = AtomicUsize::new(0);

    // Fast path: 99.9% of calls will just do this.
    // Relaxed load is essentially a zero-cost memory read on x86/ARM.
    let id = TAG_ID.load(Ordering::Relaxed);

    match id != 0 {
        true => id,
        // Slow path: Only hit on the very first call for this type.
        // Trick: Defer to a non-generic function to avoid monomorphization bloat.
        false => init_type_id(&TAG_ID),
    }
}

// #[cold] tells the compiler this is rare, keeping it out of the hot instruction cache
// and letting branch prediction heavily favor the fast path.
#[cold]
fn init_type_id(tag_id: &AtomicUsize) -> usize {
    // Grab the next available ID.
    let new_id = TAG_COUNTER.fetch_add(1, Ordering::Relaxed);

    // Try to lock it in.
    match tag_id.compare_exchange(0, new_id, Ordering::Relaxed, Ordering::Relaxed) {
        Ok(_) => new_id,
        // CAS failed? Another thread beat us to the initialization by a nanosecond.
        // Just use their ID.
        // Note to self: Our `new_id` gets thrown away here, which means we leave a harmless gap/skip in the global ID sequence.
        Err(existing_id) => existing_id,
    }
}