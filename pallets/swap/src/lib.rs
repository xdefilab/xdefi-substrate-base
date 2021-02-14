#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_event, decl_module, decl_storage, decl_error, ensure,
    StorageMap, Parameter,
};
use sp_runtime::{
    DispatchResult as Result, RuntimeDebug, ModuleId,
    traits::{
        Member, Hash, AtLeast32BitUnsigned, AccountIdConversion, Zero
    }, 
};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_core::crypto::{UncheckedFrom, UncheckedInto};
use sp_std::{marker::PhantomData, mem, vec::Vec, convert::TryInto};
use codec::{Encode, Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

const MAX_BOUND_TOKENS: usize = 8;

/// Pending atomic swap operation.
#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode)]
pub struct PoolInfo<T: Trait> {
    pub tokens: Vec<T::AssetId>,
	pub balance: Vec<T::Balance>,
	pub denorm: Vec<u64>,
}

pub trait Trait: frame_system::Trait {
	type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

	type AssetId: Parameter + AtLeast32BitUnsigned + Default + Copy;

	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	type Pool: PoolFactory<Self::AssetId, Self::AccountId>;
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Pools get(fn get_pool): map hasher(blake2_128_concat) T::Hash => Option<PoolInfo<T>>;

	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {

		SomethingStored(u32, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		MaxTokens,
		InvalidTokenLength,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn createPool(
			origin,
			tokens: Vec<T::AssetId>,
			balance: Vec<T::Balance>,
			denorm: Vec<u64>
		) -> Result {

			// check inputs
			ensure!(tokens.len() < MAX_BOUND_TOKENS, Error::<T>::MaxTokens);
			ensure!(balance.len() == tokens.len(), Error::<T>::InvalidTokenLength);
			ensure!(denorm.len() == tokens.len(), Error::<T>::InvalidTokenLength);

			// TODO: check duplication
			// TODO: sort tokens to get a unique identifier of the pool and check if exists
	
			Ok(())
		}
		
		#[weight = 10_000]
		pub fn joinPool(
			origin,
			poolId: u64,
			poolAmountOut: T::Balance,
			maxAmountsIn: Vec<T::Balance>
		) -> Result {
		
			Ok(())
		}

		#[weight = 10_000]
		pub fn exitPool(
			origin,
			poolId: u64,
			poolAmountIn: T::Balance,
			minAmountsOut: Vec<T::Balance>
		) -> Result {
		
			Ok(())
		}

		#[weight = 10_000]
		pub fn swapExactAmountIn(
			origin,
			poolId: u64,
			tokenIn: T::AssetId,
			tokenAmountIn: T::Balance,
			tokenOut: T::AssetId,
			minAmountOut: T::Balance,
			maxPrice: T::Balance
		) -> Result {

			Ok(())
		}

		#[weight = 10_000]
		pub fn swapExactAmountOut(
			origin, 
			poolId: u64,
			tokenIn: T::AssetId,
			maxAmountIn: T::Balance,
			tokenOut: T::AssetId,
			tokenAmountOut: T::Balance,
			maxPrice: T::Balance) -> Result {
			
			Ok(())
		}

	}
}

pub trait PoolFactory<AssetId, AccountId> {
    /// The generate function
    fn get_pool_address(tokens: Vec<AssetId>) -> AccountId;
}

/// Exchange Address
pub struct PoolAddress<T: Trait>(PhantomData<T>);

/// Impl PoolFactory for PoolAddress
impl<T: Trait> PoolFactory<T::AssetId, T::AccountId> for PoolAddress<T>
where
    T::AccountId: UncheckedFrom<T::Hash>,
    u64: core::convert::From<T::AssetId>,
{
	// TODO: need to make it unique
    fn get_pool_address(tokens: Vec<T::AssetId>) -> T::AccountId {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"swap");
		for token in tokens {
			buf.extend_from_slice(&u64_to_bytes(token.into()));
		}
        T::Hashing::hash(&buf[..]).unchecked_into()
    }
}

/// helper function
fn u64_to_bytes(x: u64) -> [u8; 8] {
    unsafe { mem::transmute(x.to_le()) }
}