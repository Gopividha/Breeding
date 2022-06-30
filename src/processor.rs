// use crate::validation::Validator;
use crate::{
    instruction::{BreedInstruction},
    state::{PlatformData, NFTData, ChildNFTData},
};

use metaplex_token_metadata::state::Metadata;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::{clock::Clock, Sysvar},
    program_error::ProgramError,

};

use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token;
use std::str::FromStr;


pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = BreedInstruction::unpack(instruction_data)?;

        match instruction {
            BreedInstruction::UpdatePlatformData { amount } => {
                msg!("Instruction: platform data");
                Self::process_update_platform_acc(accounts, amount, program_id)
            }
            BreedInstruction::InitBreed { amount } => {
                msg!("Instruction: Init breed state");
                Self::process_init_breed(accounts, amount, program_id)
            }
        }
    }


    pub fn process_update_platform_acc(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {

        let admin_update_auth =
            Pubkey::from_str("J7A8AeFaPNxe3w7jCxnE2xHVWZz2GgjAF9LWky5AG2Jq").unwrap();

        let account_info_iter = &mut accounts.iter();

        let user = next_account_info(account_info_iter)?;

        let platfrom_account = next_account_info(account_info_iter)?;

        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        let platform_treasury_acc = next_account_info(account_info_iter)?;


        // validation check if the user calling this instruction
        // actually holds the authority for updating the platfrom account
        if admin_update_auth != *user.key {
            msg!("wrong update auth.....");
            return Err(ProgramError::InvalidAccountData);
        }

        // check if program owns platfrom_account account
        if platfrom_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !rent.is_exempt(platfrom_account.lamports(), platfrom_account.data_len()) {
            return Err(ProgramError::InvalidInstructionData);
        }

        // unpack the platfrom_account state, to store data into
        let mut account_update_info =
            PlatformData::unpack_unchecked(&platfrom_account.try_borrow_data()?)?;

        account_update_info.is_initialized = true;
        account_update_info.treasury_acc = *platform_treasury_acc.key;
        account_update_info.platform_fee = amount;

        msg!("fee percentage : {:?}", account_update_info.platform_fee);

        // pack data into the platfrom account
        PlatformData::pack(
            account_update_info,
            &mut platfrom_account.try_borrow_mut_data()?,
        )?;

        Ok(())
    }


    pub fn process_init_breed(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {

        let user = next_account_info(account_info_iter)?;

        let nft_state_account_1 = next_account_info(account_info_iter)?;

        let nft_state_account_2 = next_account_info(account_info_iter)?;

        let nft_mint_1 = next_account_info(account_info_iter)?;

        let nft_mint_2 = next_account_info(account_info_iter)?;
        
        let user = next_account_info(account_info_iter)?;
        let user = next_account_info(account_info_iter)?;
        let user = next_account_info(account_info_iter)?;
        let user = next_account_info(account_info_iter)?;
        let user = next_account_info(account_info_iter)?;
        let user = next_account_info(account_info_iter)?;
        let user = next_account_info(account_info_iter)?;
        let user = next_account_info(account_info_iter)?;
        let user = next_account_info(account_info_iter)?;
        let user = next_account_info(account_info_iter)?;
        let user = next_account_info(account_info_iter)?;

        
        Ok(())
    }

}



