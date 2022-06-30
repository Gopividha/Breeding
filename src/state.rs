use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct NFTData {
    pub is_initialized: bool,
    pub mint_key: Pubkey,
    pub last_breed: u64,
    pub breed_count: u64,
}

impl Sealed for NFTData {}
impl IsInitialized for NFTData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
impl Pack for NFTData {
    const LEN: usize = 49;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, NFTData::LEN];
        let (is_initialized, 
            mint_key, 
            last_breed,
            breed_count
        ) = array_refs![src, 1, 32, 8, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        Ok(NFTData {
            is_initialized,
            mint_key: Pubkey::new_from_array(*mint_key),
            last_breed: u64::from_le_bytes(*last_breed),
            breed_count: u64::from_le_bytes(*breed_count),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, NFTData::LEN];
        let (is_initialized_dst,
             mint_key_dst, 
             last_breed_dst,
             breed_count_dst
            ) =
            mut_array_refs![dst, 1, 32, 8, 8];
        let NFTData {
            is_initialized,
            mint_key,
            last_breed,
            breed_count
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        mint_key_dst.copy_from_slice(mint_key.as_ref());
        *last_breed_dst = last_breed.to_le_bytes();
        *breed_count_dst = breed_count.to_le_bytes();
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ChildNFTData {
    pub is_initialized: bool,
    pub child_nft_mint: Pubkey,
    pub parent_one_mint: Pubkey,
    pub parent_two_mint: Pubkey,
    pub mint_time: u64,
}

impl Sealed for ChildNFTData {}
impl IsInitialized for ChildNFTData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
impl Pack for ChildNFTData {
    const LEN: usize = 105;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, ChildNFTData::LEN];
        let (is_initialized, 
            child_nft_mint, 
            parent_one_mint,
            parent_two_mint,
            mint_time,
            ) = array_refs![src, 1, 32, 32, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        Ok(ChildNFTData {
            is_initialized,
            child_nft_mint: Pubkey::new_from_array(*child_nft_mint),
            parent_one_mint: Pubkey::new_from_array(*parent_one_mint),
            parent_two_mint: Pubkey::new_from_array(*parent_two_mint),
            mint_time: u64::from_le_bytes(*mint_time),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, ChildNFTData::LEN];
        let (is_initialized_dst,
             child_nft_mint_dst, 
             parent_one_mint_dst,
             parent_two_mint_dst,
             mint_time_dst,
             
            ) =
            mut_array_refs![dst, 1, 32, 32, 32, 8];
        let ChildNFTData {
            is_initialized,
            child_nft_mint,
            parent_one_mint,
            parent_two_mint,
            mint_time,
            } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        child_nft_mint_dst.copy_from_slice(child_nft_mint.as_ref());
        parent_one_mint_dst.copy_from_slice(parent_one_mint.as_ref());
        parent_two_mint_dst.copy_from_slice(parent_two_mint.as_ref());
        *mint_time_dst = mint_time.to_le_bytes();
    }
}



#[derive(Debug, PartialEq, Copy,Clone)]
pub struct PlatformData {
    pub is_initialized: bool,
    pub treasury_acc: Pubkey,
    pub platform_fee: u64,
}

impl Sealed for PlatformData {}
impl IsInitialized for PlatformData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
impl Pack for PlatformData {
    const LEN: usize = 41;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, PlatformData::LEN];
        let (is_initialized, treasury_acc, platform_fee) = array_refs![src, 1, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        Ok(PlatformData {
            is_initialized,
            treasury_acc: Pubkey::new_from_array(*treasury_acc),
            platform_fee: u64::from_le_bytes(*platform_fee),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, PlatformData::LEN];
        let (is_initialized_dst, treasury_acc_dst, platform_fee_dst) =
            mut_array_refs![dst, 1, 32, 8];
        let PlatformData {
            is_initialized,
            treasury_acc,
            platform_fee,
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        treasury_acc_dst.copy_from_slice(treasury_acc.as_ref());
        *platform_fee_dst = platform_fee.to_le_bytes();
    }
}




#[derive(Debug, PartialEq, Copy,Clone)]
pub struct BreedingState {
    pub is_initialized: bool,
    pub child_mint_key: Pubkey,
    pub random_no_genrated: u64,

}

impl Sealed for BreedingState {}
impl IsInitialized for BreedingState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
impl Pack for BreedingState {
    const LEN: usize = 41;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, BreedingState::LEN];
        let (is_initialized, child_mint_key, random_no_genrated) = array_refs![src, 1, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        Ok(BreedingState {
            is_initialized,
            child_mint_key: Pubkey::new_from_array(*child_mint_key),
            random_no_genrated: u64::from_le_bytes(*random_no_genrated),

        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, BreedingState::LEN];
        let (is_initialized_dst, child_mint_key_dst, random_no_genrated_dst) =
            mut_array_refs![dst, 1, 32, 8];
        let BreedingState {
            is_initialized,
            child_mint_key,
            random_no_genrated,
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        child_mint_key_dst.copy_from_slice(child_mint_key.as_ref());
        *random_no_genrated_dst = random_no_genrated.to_le_bytes();

    }
}

