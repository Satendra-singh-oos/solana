use solana_program::{
    account_info::AccountInfo,
    entrypoint, // to intract with block chain
    entrypoint::ProgramResult, // when we deploy program we get to know about the result like program is working or not
    pubkey::Pubkey, // The address of a Solana account, as the key are in the encypted form these will work like the parser (keys will be in form of binary then it will convert to hte base58 )
    msg
    
};

// Now we make entry point which will work like send and get data fromt he blockchain

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );
    Ok(())
}
