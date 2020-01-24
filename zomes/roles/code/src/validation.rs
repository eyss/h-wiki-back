use crate::ADMINISTRATOR_ROLE;
use hdk::prelude::*;

use crate::assignment::Assignment;
use crate::role::Role;
use crate::utils;
use crate::ASSIGNMENT_TYPE;

pub fn agent_has_role(
    agent_address: &Address,
    role_address: &Address,
    chain_entries: &Vec<Entry>,
) -> ZomeApiResult<bool> {
    let maybe_assignment: Option<Assignment> =
        utils::find_entry(chain_entries, ASSIGNMENT_TYPE, |assignment: Assignment| {
            Ok(assignment.role_address == role_address.clone()
                && assignment.agent_address == agent_address.clone())
        });

    match maybe_assignment {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub fn is_some_agent_admin(
    agent_addresses: &Vec<Address>,
    chain_entries: &Vec<Entry>,
) -> ZomeApiResult<Option<Address>> {
    let admin_address = agent_addresses.iter().find_map(|agent_address| {
        match is_agent_admin(&agent_address, &chain_entries) {
            Ok(true) => Some(agent_address.clone()),
            _ => None,
        }
    });

    Ok(admin_address)
}

pub fn is_agent_admin(agent_address: &Address, chain_entries: &Vec<Entry>) -> ZomeApiResult<bool> {
    if is_agent_initial_admin(&agent_address)? {
        return Ok(true);
    }

    let admin_role = Role::from(String::from(ADMINISTRATOR_ROLE));

    agent_has_role(&agent_address, &admin_role.address()?, &chain_entries)
}

pub fn get_initial_admins() -> ZomeApiResult<Vec<Address>> {
    let initial_admins_json = hdk::property("initial_admins")?;
    let initial_admins: Result<Vec<Address>, _> =
        serde_json::from_str(&initial_admins_json.to_string());

    match initial_admins {
        Ok(admins) => Ok(admins),
        Err(_) => Err(ZomeApiError::from(String::from(
            "could not get initial admins list",
        ))),
    }
}

pub fn is_agent_initial_admin(agent_address: &Address) -> ZomeApiResult<bool> {
    let initial_admins = get_initial_admins()?;

    Ok(initial_admins.contains(agent_address))
}
