use pbc_zk::*;

#[allow(unused)]
const MESSAGE_VARIABLE_KIND: u8 = 0u8;

/// Perform a zk computation on secret-shared data concat the secret (message) variables.
///
/// ### Returns:
///
/// The concat of the secret (message) variables.
#[zk_compute(shortname = 0x61)]
pub fn concat_everything() -> Sbi64 {
    // Initialize state
    let mut concated_message: Sbi64 = Sbi64::from(0);

    // Sum each variable
    for variable_id in secret_variable_ids() {
        if load_metadata::<u8>(variable_id) == MESSAGE_VARIABLE_KIND {
            let message = load_sbi::<Sbi64>(variable_id);
            //
            concated_message = concated_message + message;
        }
    }

    concated_message
}
