//! State types for the Fogo Stake Pool program.
//!
//! This module contains all account state structures that can be deserialized
//! from on-chain data.

#[cfg(feature = "borsh")]
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
#[cfg(feature = "codama")]
use codama_macros::CodamaType;

use {
    bytemuck::{Pod, Zeroable},
    num_derive::{FromPrimitive, ToPrimitive},
    num_traits::{FromPrimitive, ToPrimitive},
    solana_program::{
        program_error::ProgramError,
        program_memory::sol_memcmp,
        program_pack::{Pack, Sealed},
        pubkey::{Pubkey, PUBKEY_BYTES},
        stake::state::Lockup,
    },
    spl_pod::primitives::{PodU32, PodU64},
    std::fmt,
};

/// Enum representing the account type managed by the program
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "borsh", derive(BorshDeserialize, BorshSerialize, BorshSchema))]
#[cfg_attr(feature = "codama", derive(CodamaType))]
pub enum AccountType {
    /// If the account has not been initialized, the enum will be 0
    #[default]
    Uninitialized,
    /// Stake pool
    StakePool,
    /// Validator stake list
    ValidatorList,
}

/// Initialized program details.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "borsh", derive(BorshDeserialize, BorshSerialize, BorshSchema))]
#[cfg_attr(feature = "codama", derive(CodamaType))]
pub struct StakePool {
    /// Account type, must be `StakePool` currently
    pub account_type: AccountType,

    /// Manager authority, allows for updating the staker, manager, and fee
    /// account
    pub manager: Pubkey,

    /// Staker authority, allows for adding and removing validators, and
    /// managing stake distribution
    pub staker: Pubkey,

    /// Stake deposit authority
    ///
    /// If a depositor pubkey is specified on initialization, then deposits must
    /// be signed by this authority. If no deposit authority is specified,
    /// then the stake pool will default to the result of:
    /// `Pubkey::find_program_address(
    ///     &[&stake_pool_address.as_ref(), b"deposit"],
    ///     program_id,
    /// )`
    pub stake_deposit_authority: Pubkey,

    /// Stake withdrawal authority bump seed
    /// for `create_program_address(&[state::StakePool account, "withdrawal"])`
    pub stake_withdraw_bump_seed: u8,

    /// Validator stake list storage account
    pub validator_list: Pubkey,

    /// Reserve stake account, holds deactivated stake
    pub reserve_stake: Pubkey,

    /// Pool Mint
    pub pool_mint: Pubkey,

    /// Manager fee account
    pub manager_fee_account: Pubkey,

    /// Pool token program id
    pub token_program_id: Pubkey,

    /// Total stake under management.
    /// Note that if `last_update_epoch` does not match the current epoch then
    /// this field may not be accurate
    pub total_lamports: u64,

    /// Total supply of pool tokens (should always match the supply in the Pool
    /// Mint)
    pub pool_token_supply: u64,

    /// Last epoch the `total_lamports` field was updated
    pub last_update_epoch: u64,

    /// Lockup that all stakes in the pool must have
    pub lockup: Lockup,

    /// Fee taken as a proportion of rewards each epoch
    pub epoch_fee: Fee,

    /// Fee for next epoch
    pub next_epoch_fee: FutureEpochFee,

    /// Preferred deposit validator vote account pubkey
    pub preferred_deposit_validator_vote_address: Option<Pubkey>,

    /// Preferred withdraw validator vote account pubkey
    pub preferred_withdraw_validator_vote_address: Option<Pubkey>,

    /// Fee assessed on stake deposits
    pub stake_deposit_fee: Fee,

    /// Fee assessed on withdrawals
    pub stake_withdrawal_fee: Fee,

    /// Future stake withdrawal fee, to be set for the following epoch
    pub next_stake_withdrawal_fee: FutureEpochFee,

    /// Fees paid out to referrers on referred stake deposits.
    /// Expressed as a percentage (0 - 100) of deposit fees.
    pub stake_referral_fee: u8,

    /// Toggles whether the `DepositSol` instruction requires a signature from
    /// this `sol_deposit_authority`
    pub sol_deposit_authority: Option<Pubkey>,

    /// Fee assessed on SOL deposits
    pub sol_deposit_fee: Fee,

    /// Fees paid out to referrers on referred SOL deposits.
    /// Expressed as a percentage (0 - 100) of SOL deposit fees.
    pub sol_referral_fee: u8,

    /// Toggles whether the `WithdrawSol` instruction requires a signature from
    /// the `deposit_authority`
    pub sol_withdraw_authority: Option<Pubkey>,

    /// Fee assessed on SOL withdrawals
    pub sol_withdrawal_fee: Fee,

    /// Future SOL withdrawal fee, to be set for the following epoch
    pub next_sol_withdrawal_fee: FutureEpochFee,

    /// Last epoch's total pool tokens, used only for APR estimation
    pub last_epoch_pool_token_supply: u64,

    /// Last epoch's total lamports, used only for APR estimation
    pub last_epoch_total_lamports: u64,
}

impl StakePool {
    /// Calculate the pool tokens that should be minted for a deposit of
    /// `stake_lamports`
    #[inline]
    pub fn calc_pool_tokens_for_deposit(&self, stake_lamports: u64) -> Option<u64> {
        if self.total_lamports == 0 || self.pool_token_supply == 0 {
            return Some(stake_lamports);
        }
        u64::try_from(
            (stake_lamports as u128)
                .checked_mul(self.pool_token_supply as u128)?
                .checked_div(self.total_lamports as u128)?,
        )
        .ok()
    }

    /// Calculate lamports amount on withdrawal
    #[inline]
    pub fn calc_lamports_withdraw_amount(&self, pool_tokens: u64) -> Option<u64> {
        let numerator = (pool_tokens as u128).checked_mul(self.total_lamports as u128)?;
        let denominator = self.pool_token_supply as u128;
        if numerator < denominator || denominator == 0 {
            Some(0)
        } else {
            u64::try_from(numerator.checked_div(denominator)?).ok()
        }
    }

    /// Calculate pool tokens to be deducted as stake withdrawal fees
    #[inline]
    pub fn calc_pool_tokens_stake_withdrawal_fee(&self, pool_tokens: u64) -> Option<u64> {
        u64::try_from(self.stake_withdrawal_fee.apply(pool_tokens)?).ok()
    }

    /// Calculate pool tokens to be deducted as SOL withdrawal fees
    #[inline]
    pub fn calc_pool_tokens_sol_withdrawal_fee(&self, pool_tokens: u64) -> Option<u64> {
        u64::try_from(self.sol_withdrawal_fee.apply(pool_tokens)?).ok()
    }

    /// Calculate pool tokens to be deducted as stake deposit fees
    #[inline]
    pub fn calc_pool_tokens_stake_deposit_fee(&self, pool_tokens_minted: u64) -> Option<u64> {
        u64::try_from(self.stake_deposit_fee.apply(pool_tokens_minted)?).ok()
    }

    /// Calculate pool tokens to be deducted from deposit fees as referral fees
    #[inline]
    pub fn calc_pool_tokens_stake_referral_fee(&self, stake_deposit_fee: u64) -> Option<u64> {
        u64::try_from(
            (stake_deposit_fee as u128)
                .checked_mul(self.stake_referral_fee as u128)?
                .checked_div(100u128)?,
        )
        .ok()
    }

    /// Calculate pool tokens to be deducted as SOL deposit fees
    #[inline]
    pub fn calc_pool_tokens_sol_deposit_fee(&self, pool_tokens_minted: u64) -> Option<u64> {
        u64::try_from(self.sol_deposit_fee.apply(pool_tokens_minted)?).ok()
    }

    /// Calculate pool tokens to be deducted from SOL deposit fees as referral fees
    #[inline]
    pub fn calc_pool_tokens_sol_referral_fee(&self, sol_deposit_fee: u64) -> Option<u64> {
        u64::try_from(
            (sol_deposit_fee as u128)
                .checked_mul(self.sol_referral_fee as u128)?
                .checked_div(100u128)?,
        )
        .ok()
    }

    /// Calculate the fee in pool tokens that goes to the manager
    ///
    /// This function assumes that `reward_lamports` has not already been added
    /// to the stake pool's `total_lamports`
    #[inline]
    pub fn calc_epoch_fee_amount(&self, reward_lamports: u64) -> Option<u64> {
        if reward_lamports == 0 {
            return Some(0);
        }
        let total_lamports = (self.total_lamports as u128).checked_add(reward_lamports as u128)?;
        let fee_lamports = self.epoch_fee.apply(reward_lamports)?;
        if total_lamports == fee_lamports || self.pool_token_supply == 0 {
            Some(reward_lamports)
        } else {
            u64::try_from(
                (self.pool_token_supply as u128)
                    .checked_mul(fee_lamports)?
                    .checked_div(total_lamports.checked_sub(fee_lamports)?)?,
            )
            .ok()
        }
    }

    /// Get the current value of pool tokens, rounded up
    #[inline]
    pub fn get_lamports_per_pool_token(&self) -> Option<u64> {
        self.total_lamports
            .checked_add(self.pool_token_supply)?
            .checked_sub(1)?
            .checked_div(self.pool_token_supply)
    }

    /// Check if `StakePool` is actually initialized as a stake pool
    pub fn is_valid(&self) -> bool {
        self.account_type == AccountType::StakePool
    }

    /// Check if `StakePool` is currently uninitialized
    pub fn is_uninitialized(&self) -> bool {
        self.account_type == AccountType::Uninitialized
    }
}

/// Storage list for all validator stake accounts in the pool.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "borsh", derive(BorshDeserialize, BorshSerialize, BorshSchema))]
#[cfg_attr(feature = "codama", derive(CodamaType))]
pub struct ValidatorList {
    /// Data outside of the validator list, separated out for cheaper
    /// deserialization
    pub header: ValidatorListHeader,

    /// List of stake info for each validator in the pool
    pub validators: Vec<ValidatorStakeInfo>,
}

/// Helper type to deserialize just the start of a `ValidatorList`
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "borsh", derive(BorshDeserialize, BorshSerialize, BorshSchema))]
#[cfg_attr(feature = "codama", derive(CodamaType))]
pub struct ValidatorListHeader {
    /// Account type, must be `ValidatorList` currently
    pub account_type: AccountType,

    /// Maximum allowable number of validators
    pub max_validators: u32,
}

impl ValidatorListHeader {
    /// Length of the header in bytes
    pub const LEN: usize = 1 + 4;

    /// Check if validator stake list is actually initialized as a validator
    /// stake list
    pub fn is_valid(&self) -> bool {
        self.account_type == AccountType::ValidatorList
    }

    /// Check if the validator stake list is uninitialized
    pub fn is_uninitialized(&self) -> bool {
        self.account_type == AccountType::Uninitialized
    }
}

impl ValidatorList {
    /// Create an empty instance containing space for `max_validators`
    pub fn new(max_validators: u32) -> Self {
        Self {
            header: ValidatorListHeader {
                account_type: AccountType::ValidatorList,
                max_validators,
            },
            validators: vec![ValidatorStakeInfo::default(); max_validators as usize],
        }
    }

    /// Calculate the number of validator entries that fit in the provided length
    pub fn calculate_max_validators(buffer_length: usize) -> usize {
        let header_size = ValidatorListHeader::LEN.saturating_add(4);
        buffer_length
            .saturating_sub(header_size)
            .saturating_div(ValidatorStakeInfo::LEN)
    }

    /// Check if contains validator with particular pubkey
    pub fn contains(&self, vote_account_address: &Pubkey) -> bool {
        self.validators
            .iter()
            .any(|x| x.vote_account_address == *vote_account_address)
    }

    /// Find a validator by vote account address (mutable)
    pub fn find_mut(&mut self, vote_account_address: &Pubkey) -> Option<&mut ValidatorStakeInfo> {
        self.validators
            .iter_mut()
            .find(|x| x.vote_account_address == *vote_account_address)
    }

    /// Find a validator by vote account address
    pub fn find(&self, vote_account_address: &Pubkey) -> Option<&ValidatorStakeInfo> {
        self.validators
            .iter()
            .find(|x| x.vote_account_address == *vote_account_address)
    }

    /// Check if the list has any active stake
    pub fn has_active_stake(&self) -> bool {
        self.validators
            .iter()
            .any(|x| u64::from(x.active_stake_lamports) > 0)
    }
}

/// Status of the stake account in the validator list, for accounting
#[derive(
    ToPrimitive,
    FromPrimitive,
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
)]
#[cfg_attr(feature = "borsh", derive(BorshDeserialize, BorshSerialize, BorshSchema))]
#[cfg_attr(feature = "codama", derive(CodamaType))]
pub enum StakeStatus {
    /// Stake account is active, there may be a transient stake as well
    #[default]
    Active,
    /// Only transient stake account exists, when a transient stake is
    /// deactivating during validator removal
    DeactivatingTransient,
    /// No more validator stake accounts exist, entry ready for removal during
    /// `UpdateStakePoolBalance`
    ReadyForRemoval,
    /// Only the validator stake account is deactivating, no transient stake
    /// account exists
    DeactivatingValidator,
    /// Both the transient and validator stake account are deactivating, when
    /// a validator is removed with a transient stake active
    DeactivatingAll,
}

/// Wrapper struct that can be `Pod`, containing a byte that *should* be a valid
/// `StakeStatus` underneath.
#[repr(transparent)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Pod,
    Zeroable,
)]
#[cfg_attr(feature = "borsh", derive(BorshDeserialize, BorshSerialize, BorshSchema))]
#[cfg_attr(feature = "codama", derive(CodamaType))]
pub struct PodStakeStatus(u8);

impl TryFrom<PodStakeStatus> for StakeStatus {
    type Error = ProgramError;
    fn try_from(pod: PodStakeStatus) -> Result<Self, Self::Error> {
        FromPrimitive::from_u8(pod.0).ok_or(ProgramError::InvalidAccountData)
    }
}

impl From<StakeStatus> for PodStakeStatus {
    fn from(status: StakeStatus) -> Self {
        PodStakeStatus(status.to_u8().unwrap())
    }
}

/// Information about a validator in the pool
///
/// NOTE: ORDER IS VERY IMPORTANT HERE, PLEASE DO NOT RE-ORDER THE FIELDS UNLESS
/// THERE'S AN EXTREMELY GOOD REASON.
///
/// To save on BPF instructions, the serialized bytes are reinterpreted with a
/// `bytemuck` transmute, which means that this structure cannot have any
/// undeclared alignment-padding in its representation.
#[repr(C)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Pod,
    Zeroable,
)]
#[cfg_attr(feature = "borsh", derive(BorshDeserialize, BorshSerialize, BorshSchema))]
#[cfg_attr(feature = "codama", derive(CodamaType))]
pub struct ValidatorStakeInfo {
    /// Amount of lamports on the validator stake account, including rent
    ///
    /// Note that if `last_update_epoch` does not match the current epoch then
    /// this field may not be accurate
    pub active_stake_lamports: PodU64,

    /// Amount of transient stake delegated to this validator
    ///
    /// Note that if `last_update_epoch` does not match the current epoch then
    /// this field may not be accurate
    pub transient_stake_lamports: PodU64,

    /// Last epoch the active and transient stake lamports fields were updated
    pub last_update_epoch: PodU64,

    /// Transient account seed suffix, used to derive the transient stake
    /// account address
    pub transient_seed_suffix: PodU64,

    /// Unused space, initially meant to specify the end of seed suffixes
    pub unused: PodU32,

    /// Validator account seed suffix
    pub validator_seed_suffix: PodU32,

    /// Status of the validator stake account
    pub status: PodStakeStatus,

    /// Validator vote account address
    pub vote_account_address: Pubkey,
}

impl ValidatorStakeInfo {
    /// Get the total lamports on this validator (active and transient)
    pub fn stake_lamports(&self) -> Option<u64> {
        u64::from(self.active_stake_lamports)
            .checked_add(self.transient_stake_lamports.into())
    }

    /// Performs a very cheap comparison, for checking if this validator stake
    /// info matches the vote account address
    pub fn memcmp_pubkey(data: &[u8], vote_address: &Pubkey) -> bool {
        sol_memcmp(
            &data[41..41_usize.saturating_add(PUBKEY_BYTES)],
            vote_address.as_ref(),
            PUBKEY_BYTES,
        ) == 0
    }

    /// Performs a comparison, used to check if this validator stake
    /// info has more active lamports than some limit
    #[cfg(feature = "borsh")]
    pub fn active_lamports_greater_than(data: &[u8], lamports: &u64) -> bool {
        u64::try_from_slice(&data[0..8]).unwrap() > *lamports
    }

    /// Performs a comparison, used to check if this validator stake
    /// info has more transient lamports than some limit
    #[cfg(feature = "borsh")]
    pub fn transient_lamports_greater_than(data: &[u8], lamports: &u64) -> bool {
        u64::try_from_slice(&data[8..16]).unwrap() > *lamports
    }

    /// Check that the validator stake info is totally removed
    pub fn is_removed(data: &[u8]) -> bool {
        FromPrimitive::from_u8(data[40]) == Some(StakeStatus::ReadyForRemoval)
            && data[0..16] == [0; 16]
    }

    /// Check that the validator stake info is active
    pub fn is_active(data: &[u8]) -> bool {
        FromPrimitive::from_u8(data[40]) == Some(StakeStatus::Active)
    }
}

impl Sealed for ValidatorStakeInfo {}

impl Pack for ValidatorStakeInfo {
    const LEN: usize = 73;

    #[cfg(feature = "borsh")]
    fn pack_into_slice(&self, data: &mut [u8]) {
        borsh::to_writer(data, self).unwrap();
    }

    #[cfg(not(feature = "borsh"))]
    fn pack_into_slice(&self, data: &mut [u8]) {
        data[..Self::LEN].copy_from_slice(bytemuck::bytes_of(self));
    }

    #[cfg(feature = "borsh")]
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let unpacked = Self::try_from_slice(src)?;
        Ok(unpacked)
    }

    #[cfg(not(feature = "borsh"))]
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        if src.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(*bytemuck::from_bytes(&src[..Self::LEN]))
    }
}

/// Wrapper type that "counts down" epochs, which is Borsh-compatible with the
/// native `Option`
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "borsh", derive(BorshSerialize, BorshDeserialize, BorshSchema))]
#[cfg_attr(feature = "codama", derive(CodamaType))]
pub enum FutureEpoch<T> {
    /// Nothing is set
    #[default]
    None,
    /// Value is ready after the next epoch boundary
    One(T),
    /// Value is ready after two epoch boundaries
    Two(T),
}

impl<T> FutureEpoch<T> {
    /// Create a new value to be unlocked in two epochs
    pub fn new(value: T) -> Self {
        Self::Two(value)
    }
}

impl<T: Clone> FutureEpoch<T> {
    /// Update the epoch, to be done after `get`ting the underlying value
    pub fn update_epoch(&mut self) {
        match self {
            Self::None => {}
            Self::One(_) => {
                *self = Self::None;
            }
            Self::Two(v) => {
                *self = Self::One(v.clone());
            }
        }
    }

    /// Get the value if it's ready, which is only at `One` epoch remaining
    pub fn get(&self) -> Option<&T> {
        match self {
            Self::None | Self::Two(_) => None,
            Self::One(v) => Some(v),
        }
    }
}

impl<T> From<FutureEpoch<T>> for Option<T> {
    fn from(v: FutureEpoch<T>) -> Option<T> {
        match v {
            FutureEpoch::None => None,
            FutureEpoch::One(inner) | FutureEpoch::Two(inner) => Some(inner),
        }
    }
}

/// Concrete type for FutureEpoch<Fee>, used in StakePool to support IDL generation.
/// This is a non-generic version specifically for Fee values.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "borsh", derive(BorshSerialize, BorshDeserialize, BorshSchema))]
#[cfg_attr(feature = "codama", derive(CodamaType))]
pub enum FutureEpochFee {
    /// Nothing is set
    #[default]
    None,
    /// Value is ready after the next epoch boundary
    One(Fee),
    /// Value is ready after two epoch boundaries
    Two(Fee),
}

impl From<FutureEpoch<Fee>> for FutureEpochFee {
    fn from(value: FutureEpoch<Fee>) -> Self {
        match value {
            FutureEpoch::None => FutureEpochFee::None,
            FutureEpoch::One(fee) => FutureEpochFee::One(fee),
            FutureEpoch::Two(fee) => FutureEpochFee::Two(fee),
        }
    }
}

impl From<FutureEpochFee> for FutureEpoch<Fee> {
    fn from(value: FutureEpochFee) -> Self {
        match value {
            FutureEpochFee::None => FutureEpoch::None,
            FutureEpochFee::One(fee) => FutureEpoch::One(fee),
            FutureEpochFee::Two(fee) => FutureEpoch::Two(fee),
        }
    }
}

impl FutureEpochFee {
    /// Create a new value to be unlocked in two epochs
    pub fn new(value: Fee) -> Self {
        Self::Two(value)
    }

    /// Update the epoch, to be done after `get`ting the underlying value
    pub fn update_epoch(&mut self) {
        match self {
            Self::None => {}
            Self::One(_) => {
                *self = Self::None;
            }
            Self::Two(v) => {
                *self = Self::One(*v);
            }
        }
    }

    /// Get the value if it's ready, which is only at `One` epoch remaining
    pub fn get(&self) -> Option<&Fee> {
        match self {
            Self::None | Self::Two(_) => None,
            Self::One(v) => Some(v),
        }
    }
}

impl From<FutureEpochFee> for Option<Fee> {
    fn from(v: FutureEpochFee) -> Option<Fee> {
        match v {
            FutureEpochFee::None => None,
            FutureEpochFee::One(inner) | FutureEpochFee::Two(inner) => Some(inner),
        }
    }
}

/// Fee rate as a ratio, minted on `UpdateStakePoolBalance` as a proportion of
/// the rewards.
///
/// If either the numerator or the denominator is 0, the fee is considered to be 0.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "borsh", derive(BorshSerialize, BorshDeserialize, BorshSchema))]
#[cfg_attr(feature = "codama", derive(CodamaType))]
pub struct Fee {
    /// Denominator of the fee ratio
    pub denominator: u64,
    /// Numerator of the fee ratio
    pub numerator: u64,
}

impl Fee {
    /// Applies the Fee's rates to a given amount, `amt`
    /// returning the amount to be subtracted from it as fees
    /// (0 if denominator is 0 or amt is 0),
    /// or None if overflow occurs
    #[inline]
    pub fn apply(&self, amt: u64) -> Option<u128> {
        if self.denominator == 0 {
            return Some(0);
        }
        let numerator = (amt as u128).checked_mul(self.numerator as u128)?;
        let denominator = self.denominator as u128;
        numerator
            .checked_add(denominator)?
            .checked_sub(1)?
            .checked_div(denominator)
    }
}

impl fmt::Display for Fee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.numerator > 0 && self.denominator > 0 {
            write!(f, "{}/{}", self.numerator, self.denominator)
        } else {
            write!(f, "none")
        }
    }
}

/// The type of fees that can be set on the stake pool
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "borsh", derive(BorshDeserialize, BorshSerialize, BorshSchema))]
#[cfg_attr(feature = "codama", derive(CodamaType))]
pub enum FeeType {
    /// Referral fees for SOL deposits
    SolReferral(u8),
    /// Referral fees for stake deposits
    StakeReferral(u8),
    /// Management fee paid per epoch
    Epoch(Fee),
    /// Stake withdrawal fee
    StakeWithdrawal(Fee),
    /// Deposit fee for SOL deposits
    SolDeposit(Fee),
    /// Deposit fee for stake deposits
    StakeDeposit(Fee),
    /// SOL withdrawal fee
    SolWithdrawal(Fee),
}

impl FeeType {
    /// Checks if the provided fee is too high, returning true if so
    pub fn is_too_high(&self) -> bool {
        match self {
            Self::SolReferral(pct) => *pct > 100u8,
            Self::StakeReferral(pct) => *pct > 100u8,
            Self::Epoch(fee) => fee.numerator > fee.denominator,
            Self::StakeWithdrawal(fee) => fee.numerator > fee.denominator,
            Self::SolWithdrawal(fee) => fee.numerator > fee.denominator,
            Self::SolDeposit(fee) => fee.numerator > fee.denominator,
            Self::StakeDeposit(fee) => fee.numerator > fee.denominator,
        }
    }

    /// Returns if the contained fee can only be updated earliest on the next epoch
    #[inline]
    pub fn can_only_change_next_epoch(&self) -> bool {
        matches!(
            self,
            Self::StakeWithdrawal(_) | Self::SolWithdrawal(_) | Self::Epoch(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stake_pool_is_valid() {
        let mut pool = StakePool::default();
        assert!(!pool.is_valid());
        assert!(pool.is_uninitialized());

        pool.account_type = AccountType::StakePool;
        assert!(pool.is_valid());
        assert!(!pool.is_uninitialized());
    }

    #[test]
    fn test_fee_apply() {
        let fee = Fee {
            numerator: 1,
            denominator: 10,
        };
        // 10% of 100 = 10
        assert_eq!(fee.apply(100), Some(10));

        // Zero denominator means zero fee
        let zero_fee = Fee {
            numerator: 1,
            denominator: 0,
        };
        assert_eq!(zero_fee.apply(100), Some(0));
    }

    #[test]
    fn test_calc_pool_tokens_for_deposit() {
        let pool = StakePool {
            total_lamports: 1_000_000,
            pool_token_supply: 1_000_000,
            ..Default::default()
        };
        // 1:1 ratio
        assert_eq!(pool.calc_pool_tokens_for_deposit(100), Some(100));

        // Empty pool returns deposit amount
        let empty_pool = StakePool::default();
        assert_eq!(empty_pool.calc_pool_tokens_for_deposit(100), Some(100));
    }

    #[test]
    fn test_future_epoch() {
        let mut future: FutureEpoch<u64> = FutureEpoch::new(42);
        assert_eq!(future.get(), None); // Two epochs away

        future.update_epoch();
        assert_eq!(future.get(), Some(&42)); // One epoch away, ready

        future.update_epoch();
        assert_eq!(future.get(), None); // Gone
    }
}
