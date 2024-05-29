
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
	pub type Candidates<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		
		Voted{

			who: T::AccountId,
			candidate: T::AccountId,
		},
		CandidateAdded(T::AccountId),
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
		pub fn add_candidate(origin: OriginFor<T>, candidate_id: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
	
			Candidates::<T>::insert(&candidate_id,  0 );

			Self::deposit_event(Event::CandidateAdded(candidate_id));
	
			Ok(())
		}


		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn remove_candidate(origin: OriginFor<T>, candidate_id: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
		
			Candidates::<T>::remove(&candidate_id);

			Self::deposit_event(Event::CandidateRemoved(candidate_id));
		
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn vote(origin: OriginFor<T>, candidate: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
		
			let result: Result<(), Error<T>> = Candidates::<T>::mutate(candidate.clone(), |vote_count| {
				*vote_count = match vote_count {
				
					Some(count) => match count.checked_add(1) {
						Some(new_count) => Some(new_count),
						None => return Err(Error::<T>::StorageOverflow.into()), 
					},
				
					None => Some(1),
				};
				Ok(())
			});
		
			
			result?;
		
			
			Self::deposit_event(Event::Voted{who, candidate});
		
			Ok(())
		}
		


	}
}
