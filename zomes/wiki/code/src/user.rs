extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    entry::Entry,
    // agent::AgentId, dna::entry_types::Sharing, entry::Entry, link::LinkMatch,
    link::LinkMatch,
};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::{Address, AddressableContent};
use hdk::prelude::Entry::App;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    AGENT_ADDRESS,
    // AGENT_ADDRESS, AGENT_ID_STR,
};

use crate::utils;
use holochain_anchors;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct User {
    r#type: String,
    data: String,
}
impl User {
    fn from(data: String) -> User {
        User {
            r#type: "username".to_string(),
            data,
        }
    }
    fn entry(self) -> Entry {
        App("user".into(), self.into())
    }
}

pub fn user_def() -> ValidatingEntryType {
    entry!(
        name: "user",
        description: "this is an entry representing some profile info for an agent",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<User>| {
            Ok(())
        },
        links: [
            from!(
                holochain_anchors::ANCHOR_TYPE,
                link_type: "anchor->User",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                "%agent_id",
                link_type: "agent->User",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "%agent_id",
                link_type: "User->agent",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}
pub fn create_user_if_non_existent(data: String) -> ZomeApiResult<Address> {
    let address = User::from(data.clone()).entry().address();
    match hdk::get_entry(&address)? {
        None => {
            let page_anchor = utils::anchor("users", "all_users")?;
            let address = hdk::utils::commit_and_link(
                &User::from(data.clone()).entry(),
                &page_anchor,
                "anchor->User",
                "",
            )?;
            hdk::api::link_entries(&address, &AGENT_ADDRESS, "User->agent", "")?;
            hdk::api::link_entries(&AGENT_ADDRESS, &address, "agent->User", "")?;
            Ok(address)
        }
        Some(_) => Ok(address),
    }
}
pub fn get_usernames() -> ZomeApiResult<Vec<String>> {
    let anchor_address = utils::anchor("users", "all_users")?;
    Ok(hdk::utils::get_links_and_load_type::<User>(
        &anchor_address,
        LinkMatch::Exactly("anchor->User".into()),
        LinkMatch::Any,
    )?
    .into_iter()
    .map(|user| user.data)
    .collect())
}
pub fn get_user_by_agent_id(agent_id: Address) -> ZomeApiResult<Vec<String>> {
    Ok(hdk::utils::get_links_and_load_type::<User>(
        &agent_id,
        LinkMatch::Exactly("agent->User".into()),
        LinkMatch::Any,
    )?
    .into_iter()
    .map(|user| user.data)
    .collect())
}
pub fn get_agent_user(user_name: String) -> ZomeApiResult<Address> {
    Ok(hdk::get_links(
        &User::from(user_name.clone()).entry().address(),
        LinkMatch::Exactly("User->agent".into()),
        LinkMatch::Any,
    )?
    .addresses()[0]
        .clone())
}
