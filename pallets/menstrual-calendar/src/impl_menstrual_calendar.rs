use crate::*;

/// MenstrualCalendar Interface Implementation
impl<T: Config> MenstrualCalendarInterface<T> for Pallet<T> {
	type Error = Error<T>;
	type MenstrualCycleLog = MenstrualCycleLogOf<T>;
	type MenstrualCalendar = MenstrualCalendarOf<T>;
	type Date = MomentOf<T>;

	fn add_menstrual_calendar(
		address_id: &T::AccountId,
		average_cycle: u8,
	) -> Result<Self::MenstrualCalendar, Self::Error> {
		let now = pallet_timestamp::Pallet::<T>::get();
		let total_count = MenstrualCalendarCountByOwner::<T>::get(address_id).unwrap_or(0);
		let id = Self::generate_id(address_id, total_count, None);
		let menstrual_calendar = MenstrualCalendar::new(id, address_id.clone(), average_cycle, now);

		// Store to MenstrualCalendarById storage
		MenstrualCalendarById::<T>::insert(&id, &menstrual_calendar);

		Self::add_menstrual_calendar_by_owner(address_id, &id);
		Self::add_menstrual_calendar_count();
		Self::add_menstrual_calendar_count_by_owner(address_id);

		Ok(menstrual_calendar)
	}

	fn update_menstrual_calendar(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		average_cycle: u8,
	) -> Result<Self::MenstrualCalendar, Self::Error> {
		let mut menstrual_calendar = MenstrualCalendarById::<T>::get(menstrual_calendar_id)
			.ok_or(Error::<T>::MenstrualCalendarDoesNotExist)?;

		if &menstrual_calendar.address_id != address_id {
			return Err(Error::<T>::NotMenstrualCalendarOwner)
		}

		let now = pallet_timestamp::Pallet::<T>::get();

		menstrual_calendar.average_cycle = average_cycle;
		menstrual_calendar.updated_at = now;

		// Store to MenstrualCalendarById storage
		MenstrualCalendarById::<T>::insert(menstrual_calendar_id, &menstrual_calendar);

		Ok(menstrual_calendar)
	}

	fn add_menstrual_cycle_log(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		date: &Self::Date,
		symptoms: &[Symptom],
		menstruation: bool,
	) -> Result<Self::MenstrualCycleLog, Self::Error> {
		let menstrual_calendar = MenstrualCalendarById::<T>::get(menstrual_calendar_id)
			.ok_or(Error::<T>::MenstrualCalendarDoesNotExist)?;

		if &menstrual_calendar.address_id != address_id {
			return Err(Error::<T>::NotMenstrualCalendarOwner)
		}

		let owner_menstrual_cycle_log_count =
			MenstrualCycleLogCountByOwner::<T>::get(menstrual_calendar_id).unwrap_or(0);

		let menstrual_cycle_log_id = Self::generate_id(
			address_id,
			owner_menstrual_cycle_log_count,
			Some(*menstrual_calendar_id),
		);

		let now = pallet_timestamp::Pallet::<T>::get();

		// Store to MenstrualCycleLogById storage
		let _menstrual_cycle_log = MenstrualCycleLog::new(
			menstrual_cycle_log_id,
			*menstrual_calendar_id,
			*date,
			menstruation,
			symptoms.to_vec(),
			now,
		);

		MenstrualCycleLogById::<T>::insert(menstrual_cycle_log_id, &_menstrual_cycle_log);

		Self::add_menstrual_cycle_log_by_owner(menstrual_calendar_id, &menstrual_cycle_log_id);
		Self::add_menstrual_cycle_log_count();
		Self::add_menstrual_cycle_log_count_by_owner(menstrual_calendar_id);

		Ok(_menstrual_cycle_log)
	}

	fn update_menstrual_cycle_log(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		menstrual_cycle_log_id: &T::Hash,
		date: &Self::Date,
		symptoms: &[Symptom],
		menstruation: bool,
	) -> Result<Self::MenstrualCycleLog, Self::Error> {
		let menstrual_calendar = MenstrualCalendarById::<T>::get(menstrual_calendar_id)
			.ok_or(Error::<T>::MenstrualCalendarDoesNotExist)?;

		if &menstrual_calendar.address_id != address_id {
			return Err(Error::<T>::NotMenstrualCalendarOwner)
		}

		let mut menstrual_cycle_log = MenstrualCycleLogById::<T>::get(menstrual_cycle_log_id)
			.ok_or(Error::<T>::MenstrualCycleLogDoesNotExist)?;

		if &menstrual_cycle_log.menstrual_calendar_id != menstrual_calendar_id {
			return Err(Error::<T>::NotMenstrualCycleLogOwner)
		}

		let now = pallet_timestamp::Pallet::<T>::get();

		menstrual_cycle_log.date = *date;
		menstrual_cycle_log.menstruation = menstruation;
		menstrual_cycle_log.symptoms = symptoms.to_vec();
		menstrual_cycle_log.updated_at = now;

		// Store to MenstrualCycleLogById storage
		MenstrualCycleLogById::<T>::insert(menstrual_cycle_log_id, &menstrual_cycle_log);

		Ok(menstrual_cycle_log)
	}

	fn remove_menstrual_cycle_log(
		address_id: &T::AccountId,
		menstrual_calendar_id: &T::Hash,
		menstrual_cycle_log_id: &T::Hash,
	) -> Result<(), Self::Error> {
		let menstrual_calendar = MenstrualCalendarById::<T>::get(menstrual_calendar_id)
			.ok_or(Error::<T>::MenstrualCalendarDoesNotExist)?;

		if &menstrual_calendar.address_id != address_id {
			return Err(Error::<T>::NotMenstrualCalendarOwner)
		}

		let menstrual_cycle_log = MenstrualCycleLogById::<T>::get(menstrual_cycle_log_id)
			.ok_or(Error::<T>::MenstrualCycleLogDoesNotExist)?;

		if &menstrual_cycle_log.menstrual_calendar_id != menstrual_calendar_id {
			return Err(Error::<T>::NotMenstrualCycleLogOwner)
		}

		// Remove menstrual_cycle_log from storage
		MenstrualCycleLogById::<T>::remove(menstrual_cycle_log_id);

		Self::sub_menstrual_cycle_log_by_owner(menstrual_calendar_id, menstrual_cycle_log_id);
		Self::sub_menstrual_cycle_log_count();
		Self::sub_menstrual_cycle_log_count_by_owner(menstrual_calendar_id);

		Ok(())
	}
}

/// MenstrualCalendarProvider Trait Implementation
impl<T: Config> MenstrualCalendarProvider<T> for Pallet<T> {
	type Error = Error<T>;
	type MenstrualCalendar = MenstrualCalendarOf<T>;

	fn menstrual_calendar_by_id(id: &T::Hash) -> Option<MenstrualCalendarOf<T>> {
		MenstrualCalendarById::<T>::get(id)
	}
}
