use pbc_zk::*;

#[allow(unused)]
const MESSAGE_VARIABLE_KIND: u8 = 0u8;

/// Perform a zk computation on secret-shared data concat the secret (message) variables.
///
/// ### Returns:
///
/// The concat of the secret (message) variables.
#[zk_compute(shortname = 0x61)]
pub fn concat_everything() -> Sbi32 {
    // Initialize state
    let mut concated_message: Sbi32 = Sbi32::from(0);

    // MAX_NUMBER_COMMENTS is a u32 so count can also be a u32
    let mut shift : i32 = 1;
    // concat each variable
    for variable_id in secret_variable_ids() {
        if load_metadata::<u8>(variable_id) == MESSAGE_VARIABLE_KIND {
            let message = load_sbi::<Sbi32>(variable_id);
            // each message is of length 8
            concated_message = concated_message + message * Sbi32::from(shift);
            shift = shift * 8;
        }
    }

    concated_message
}
