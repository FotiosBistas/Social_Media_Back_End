static mut UID_COUNTER: u32 = 0;

/**
 * This function fetches the next available user ID.
 */
pub fn get_next_uid() -> u32 {
    let curr: u32;
    unsafe {
        curr = UID_COUNTER;
        UID_COUNTER += 1;
    }
    return curr;
}