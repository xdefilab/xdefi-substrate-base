#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_event, decl_module, decl_storage, ensure, Parameter,
    StorageMap, StorageValue,
};
use frame_system::{self as system, ensure_signed, ensure_root};
use sp_std::prelude::*;
use sp_runtime::{
    DispatchResult, FixedPointOperand,
    traits::{Member, One, AtLeast32BitUnsigned, Zero},
};

/// The module configuration trait.
pub trait Trait: frame_system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    /// The units in which we record balances.
    type Balance: Member + Parameter + FixedPointOperand + AtLeast32BitUnsigned + Default + Copy;
    /// The arithmetic type of asset identifier.
    type AssetId: Parameter + AtLeast32BitUnsigned + Default + Copy;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where
        origin: T::Origin
    {
		fn deposit_event() = default;

        #[weight = 1]
        fn issue(origin, total: T::Balance) -> DispatchResult {
            let origin = ensure_signed(origin)?;

            let id = Self::next_asset_id();
            <NextAssetId<T>>::mutate(|id| *id += One::one());

            <Balances<T>>::insert(id, origin.clone(), total);
            <TotalSupply<T>>::insert(id, total);

            Self::deposit_event(RawEvent::Issued(id, origin, total));

            Ok(())
        }

        #[weight = 1]
        fn destroy(origin, id: T::AssetId) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            let balance = <Balances<T>>::take(id, origin.clone());
            ensure!(!balance.is_zero(), "origin balance should be non-zero");

            <TotalSupply<T>>::mutate(id, |total_supply| *total_supply -= balance);
            Self::deposit_event(RawEvent::Destroyed(id, origin, balance));

            Ok(())
        }

        #[weight = 1]
        pub fn set_inherent_asset(origin, asset: T::AssetId) -> DispatchResult {
            ensure_root(origin)?;
            <InherentAsset<T>>::mutate(|ia| *ia = asset.clone());

            Ok(())
        }

        #[weight = 1]
        pub fn transfer_asset(
            origin,
            id: T::AssetId,
            to: T::AccountId,
            amount: T::Balance
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            Self::transfer(from, id, to, amount);

            Ok(())
        }
	}
}

decl_event! {
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
		<T as Trait>::Balance,
		<T as Trait>::AssetId,
	{
		/// Some assets were issued. \[asset_id, owner, total_supply\]
		Issued(AssetId, AccountId, Balance),
		/// Some assets were transferred. \[asset_id, from, to, amount\]
		Transferred(AssetId, AccountId, AccountId, Balance),
		/// Some assets were destroyed. \[asset_id, owner, balance\]
		Destroyed(AssetId, AccountId, Balance),
	}
}

decl_storage! {
    trait Store for Module<T: Trait> as Assets
    {
        /// The next asset identifier up for grabs.
        NextAssetId get(fn next_asset_id): T::AssetId;
        /// The total unit supply of an asset.
        TotalSupply get(fn get_asset_total_supply): map hasher(twox_64_concat) T::AssetId => T::Balance;
        /// The number of units of assets held by any given account.
        Balances get(fn get_asset_balance): double_map 
            hasher(twox_64_concat) T::AssetId, hasher(blake2_128_concat) T::AccountId
            => T::Balance;
        /// The default inherent asset in this platform
        InherentAsset get(fn inherent_asset_id): T::AssetId;
        /// for test only
        Owner get(fn owner) config(): T::AccountId;
    }
    
    add_extra_genesis {
        config(assets): Vec<(T::AccountId, T::Balance)>;

        build(|config: &GenesisConfig<T>| {
            for asset in config.assets.iter() {
                let (account, amount) = asset;
                <Module<T>>::_issue(account.clone(), amount.clone());
                let to_account = <Owner<T>>::get();
                let asset_id = <NextAssetId<T>>::get() - 1.into();
                <Module<T>>::transfer(account.clone(), asset_id, to_account, 50000.into());
            }
        })
    }
}

impl<T: Trait> Module<T> {
    /// Issue a new class of fungible assets. There are, and will only ever be, `total`
    /// such assets and they'll all belong to the `origin` initially. It will have an
    /// identifier `AssetId` instance: this will be specified in the `Issued` event.
    /// This will make a increased id asset.
    /// @origin
    /// @total    How much balance of new asset
    fn _issue(account: T::AccountId, total: T::Balance) {
        let id = Self::next_asset_id();
        <NextAssetId<T>>::mutate(|id| *id += One::one());

        <Balances<T>>::insert(id, account.clone(), total);
        <TotalSupply<T>>::insert(id, total);

        Self::deposit_event(RawEvent::Issued(id, account, total));
    }

    /// Move some assets from one holder to another.
    /// @from    The account lost amount of a certain asset balance
    /// @id              The asset id to transfer
    /// @to      The account receive the sent asset balance
    /// @amount          The amount value to transfer
    pub fn transfer(
        from: T::AccountId,
        id: T::AssetId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> DispatchResult {
        let origin_balance = <Balances<T>>::get(id, from.clone());
        ensure!(!amount.is_zero(), "transfer amount should be non-zero");
        ensure!(
            origin_balance >= amount,
            "origin account balance must be greater than or equal to the transfer amount"
        );

        Self::deposit_event(RawEvent::Transferred(
            id,
            from,
            to,
            amount,
        ));

        Ok(())
    }

}
