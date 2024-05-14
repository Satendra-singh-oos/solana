use solana_program::{
    account_info::AccountInfo,     // Importing utilities to work with account information
    entrypoint,                    // Entry point to interact with the blockchain
    entrypoint::ProgramResult,    // Result type to indicate success or failure of the program
    pubkey::Pubkey,                // Data type representing the address of a Solana account
    msg                            // Macro for printing messages to the console
};

// Now we define the entry point which will be called when this program is executed on the blockchain

entrypoint!(process_instruction);

// Function to process instructions sent to the program
fn process_instruction(
    program_id: &Pubkey,               // The program ID, which uniquely identifies this program on the blockchain
    accounts: &[AccountInfo],          // Array of accounts associated with the program
    instruction_data: &[u8],           // Raw instruction data sent to the program
) -> ProgramResult {
    // Print a message to the console indicating that the instruction processing has started
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );
    
    // Return Ok(()) to indicate that the instruction processing was successful
    Ok(())
}
