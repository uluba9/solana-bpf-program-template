use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

/// Simple example instruction handler for demonstration purposes.
/// Logs a message to show how Solana BPF programs can process input data.
pub fn process_hello_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello from example Solana program!");
    Ok(())
}
