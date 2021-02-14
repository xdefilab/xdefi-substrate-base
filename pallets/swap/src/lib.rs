#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_event, decl_module, decl_storage, decl_error, ensure,
    StorageMap, Parameter,
};
use sp_runtime::{
    DispatchResult as Result, RuntimeDebug, ModuleId,
    traits::{
        Member, AtLeast32BitUnsigned, AccountIdConversion, Zero
    }, 
};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_std::{vec::Vec, convert::TryInto};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

const MAX_BOUND_TOKENS: usize = 8;

pub trait Trait: frame_system::Trait {
	type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

	type AssetId: Parameter + AtLeast32BitUnsigned + Default + Copy;

	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}


decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Something get(fn something): Option<u32>;
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
