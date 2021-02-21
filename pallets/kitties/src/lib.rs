#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get, traits::Randomness, debug};
use frame_system::ensure_signed;

use codec::{Encode, Decode};
use sp_core::H256;
use sp_std::vec::Vec;


pub trait Trait: frame_system::Trait + pallet_balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type RandomnessSource: Randomness<H256>;
}


#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    dna: Hash,
    price: Balance,
    gen: u64,
}

decl_storage! {
	trait Store for Module<T: Trait> as KittyStorage {
		pub Kitties get(fn kitty): map hasher(blake2_128_concat) T::Hash => Kitty<T::Hash, T::Balance>;
		pub KittyOwner get(fn owner_of): map hasher(blake2_128_concat) T::Hash => Option<T::AccountId>;

        pub AllKittiesArray get(fn kitty_by_index): map hasher(blake2_128_concat) u64 => T::Hash;
        pub AllKittiesCount get(fn all_kitties_count): u64;
        pub AllKittiesIndex: map hasher(blake2_128_concat) T::Hash => u64;

        pub OwnedKittiesArray get(fn kitty_of_owner_by_index): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat)  u64 => T::Hash;
        pub OwnedKittiesCount get(fn owned_kitty_count): map hasher(blake2_128_concat) T::AccountId => u64;
        pub OwnedKittiesIndex: map hasher(blake2_128_concat) T::Hash => u64;

        pub Nonce: u64;
	}
}

decl_event!(
	pub enum Event<T> where
	        AccountId = <T as frame_system::Trait>::AccountId,
	        Hash = <T as frame_system::Trait>::Hash,
            Balance = <T as pallet_balances::Trait>::Balance
	{
		SomethingStored(u32, AccountId),
        Created(AccountId, Hash),
        PriceSet(AccountId, Hash, Balance),
        Transferred(AccountId, AccountId, Hash),
        Bought(AccountId, AccountId, Hash, Balance),
	}
);


decl_error! {
	pub enum Error for Module<T: Trait> {

	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        fn create_kitty(origin) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let nonce = Self::get_nonce();
            let random_seed = T::RandomnessSource::random_seed();
			let rand = T::RandomnessSource::random(&nonce);

            let random_hash = (rand, &sender, nonce).using_encoded(sp_io::hashing::blake2_128);

            let new_kitty = Kitty {
                 id: random_hash,
                 dna: random_hash,
                 price: 0,
                 gen: 0,
            };

            Self::mint(sender, random_hash, new_kitty)?;


            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        fn set_price(origin, kitty_id: T::Hash, new_price: T::Balance) -> dispatch::DispatchResult  {
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        fn transfer(origin, to: T::AccountId, kitty_id: T::Hash) -> dispatch::DispatchResult {
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        fn buy_kitty(origin, kitty_id: T::Hash, max_price: T::Balance) -> dispatch::DispatchResult {
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        fn breed_kitty(origin, kitty_id_1: T::Hash, kitty_id_2: T::Hash) -> dispatch::DispatchResult {
            Ok(())
        }
	}
}


impl<T: Trait> Module<T> {

    fn mint(to: T::AccountId, kitty_id: T::Hash, new_kitty: Kitty<T::Hash, T::Balance>) -> dispatch::DispatchResult {
        Ok(())
    }

    fn transfer_from(from: T::AccountId, to: T::AccountId, kitty_id: T::Hash) ->  dispatch::DispatchResult {
        Ok(())
    }

    fn get_nonce() -> Vec<u8> {
        let nonce = Nonce::get();
        Nonce::put(nonce.wrapping_add(1));
        nonce.encode()
    }

}
