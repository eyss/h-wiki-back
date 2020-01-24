#![feature(vec_remove_item)]
#![feature(proc_macro_hygiene)]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde_derive::{Deserialize, Serialize};

use hdk::{AGENT_ADDRESS, entry_definition::ValidatingEntryType, error::{ZomeApiResult, ZomeApiError}};

use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

use crate::role::Role;
use crate::assignment::Assignment;

pub const ROLE_TYPE: &str = "role";
pub const ASSIGNMENT_TYPE: &str = "role_assignment";
pub const ROLE_ASSIGNMENT_LINK_TYPE: &str = "role->role_assignment";
pub const AGENT_ASSIGNMENT_LINK_TYPE: &str = "agent->role_assignment";
pub const ADMINISTRATOR_ROLE: &str = "administrator";

pub mod assignment;
pub mod role;
pub mod utils;
pub mod validation;

#[zome]
pub mod roles_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn role_entry_def() -> ValidatingEntryType {
        role::role_entry_definition()
    }

    #[entry_def]
    fn assignment_entry_def() -> ValidatingEntryType {
        assignment::assignment_entry_definition()
    }

    #[zome_fn("hc_public")]
    pub fn create_role(name: String) -> ZomeApiResult<Address> {
        let role = Role::from(name);
        hdk::commit_entry(&role.entry())
    }

    #[zome_fn("hc_public")]
    pub fn assign_role(name: String, agent_address: Address) -> ZomeApiResult<Address> {
        let role = Role::from(name);

        let role_address = role.address()?;

        let maybe_role_entry = hdk::get_entry(&role_address)?;

        match maybe_role_entry {
            None => Err(ZomeApiError::from(String::from("Cannot assign a role that has not been created yet"))),
            Some(role_entry) => {
                hdk::commit_entry(&role_entry)?;

                let assignment = Assignment::from(&role_address, &agent_address, &AGENT_ADDRESS);
                let assignment_address = hdk::commit_entry(&assignment.entry())?;

                hdk::link_entries(&role_address, &assignment_address, ROLE_ASSIGNMENT_LINK_TYPE, "")?;
                hdk::link_entries(&agent_address, &assignment_address, AGENT_ASSIGNMENT_LINK_TYPE, "")?;

                Ok(assignment_address)
            }
        }
    }
}
