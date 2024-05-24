
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;


	#[pallet::pallet]
	pub struct Pallet<T>(_);

	
	#[pallet::config]
	pub trait Config: frame_system::Config {
	
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		
		type WeightInfo: WeightInfo;

		type MaxNameLength: Get<u32>;
	}

	
	#[pallet::storage]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type Candidates<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<u8, T::MaxNameLength>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		
		SomethingStored {
			
			something: u32,
			
			who: T::AccountId,
		},
		CandidateAddedOrUpdated(T::AccountId),
		CandidateRemoved(T::AccountId)
	}


	#[pallet::error]
	pub enum Error<T> {
		
		NoneValue,
		StorageOverflow,
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
        pub fn vote(origin: OriginFor<T>, candidate: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

			// Agregar lógica de votos
            
            Ok(())
        }

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn add_or_update_candidate(origin: OriginFor<T>, account_id: T::AccountId, name: BoundedVec<u8, T::MaxNameLength>) -> DispatchResult {
			let who = ensure_signed(origin)?;
	
			// Crear o actualizar la información del candidato
			Candidates::<T>::insert(&account_id,  name );
	
			// Emitir evento (opcional)
			Self::deposit_event(Event::CandidateAddedOrUpdated(account_id));
	
			Ok(())
		}


		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn remove_candidate(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
		
			// Eliminar el candidato
			Candidates::<T>::remove(&account_id);
		
			// Emitir evento (opcional)
			Self::deposit_event(Event::CandidateRemoved(account_id));
		
			Ok(())
		}

		
	}
}
