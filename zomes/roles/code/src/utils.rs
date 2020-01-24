use hdk::prelude::*;
use std::convert::TryFrom;

pub fn find_entry<T, P>(chain_entries: &Vec<Entry>, entry_type: &str, predicate: P) -> Option<T>
where
    T: TryFrom<JsonString> + Clone,
    P: Fn(T) -> ZomeApiResult<bool>,
{
    chain_entries.iter().find_map(|entry| match entry {
        Entry::App(current_entry_type, entry_content) => {
            if current_entry_type.to_string() != entry_type {
                return None;
            }

            let content = T::try_from(entry_content.clone());
            match content {
                Ok(c) => {
                    let found = predicate(c.clone());
                    match found {
                        Ok(true) => Some(c),
                        _ => None,
                    }
                }
                Err(_) => None,
            }
        }
        _ => None,
    })
}

pub fn find_entry_with_address<T>(
    chain_entries: &Vec<Entry>,
    entry_type: &str,
    address: Address,
) -> ZomeApiResult<Option<T>>
where
    T: TryFrom<JsonString> + Clone,
{
    for entry in chain_entries {
        if let Entry::App(current_entry_type, entry_content) = entry {
            let entry_address = hdk::entry_address(&entry)?;
            if current_entry_type.to_string() == entry_type && entry_address == address {
                let content = T::try_from(entry_content.clone());

                if let Ok(c) = content {
                    return Ok(Some(c));
                } else {
                    return Err(ZomeApiError::from(String::from("Error converting entry")));
                }
            }
        }
    }

    Ok(None)
}
