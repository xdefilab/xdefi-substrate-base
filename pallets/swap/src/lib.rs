#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
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
		NoneValue,
		StorageOverflow,
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
			tokens: Vec<AssetId>,
			balance: Vec<Balance>,
			denorm: Vec<u64>
		) -> dispatch::DispatchResult {
		
		}
		
		#[weight = 10_000]
		pub fn joinPool(
			origin,
			poolId: u64,
			poolAmountOut: Balance,
			maxAmountsIn: Vec<Balance>
		) -> dispatch::DispatchResult {
		
		}

		#[weight = 10_000]
		pub fn exitPool(
			origin,
			poolId: u64,
			poolAmountIn: Balance,
			minAmountsOut: Vec<Balance>
		) -> dispatch::DispatchResult {
		
		}

		#[weight = 10_000]
		pub fn swapExactAmountIn(
			origin,
			poolId: u64,
			tokenIn: AssetId,
			tokenAmountIn: Balance,
			tokenOut: AssetId,
			minAmountOut: Balance,
			maxPrice: Balance
		) -> dispatch::DispatchResult {

		}

		#[weight = 10_000]
		pub fn swapExactAmountOut(
			origin, 
			poolId: u64,
			tokenIn: AssetId,
			maxAmountIn: Balance,
			tokenOut: AssetId,
			tokenAmountOut: Balance,
			maxPrice: Balance) -> dispatch::DispatchResult {

		}

	}
}
