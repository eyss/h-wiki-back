#![feature(proc_macro_hygiene)]
#![feature(vec_remove_item)]

#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hc_roles_mixin::Role;

// use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    //,
    AGENT_ADDRESS,
    //, AGENT_ID_STR,
};
use hdk_proc_macros::zome;
use holochain_anchors;
// see https://developer.holochain.org/api/0.0.25-alpha1/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry
mod page;
mod section;
mod user;
mod utils;
use section::Section2;

#[zome]
mod wiki {
    #[init]
    fn init() {
        hc_roles_mixin::handlers::create_admin_role()?;
        match hc_roles_mixin::handlers::create_role(&"Editor".to_string()) {
            _ => (),
        };
        Ok(())
    }
    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }
    #[entry_def]
    fn role_entry_def() -> ValidatingEntryType {
        hc_roles_mixin::role_entry_def()
    }
    #[entry_def]
    fn anchor_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }
    #[entry_def]
    fn user_def() -> ValidatingEntryType {
        user::user_def()
    }
    #[entry_def]
    fn page_def() -> ValidatingEntryType {
        page::page_def()
    }
    #[entry_def]
    fn page_element_def() -> ValidatingEntryType {
        section::page_element_def()
    }

    #[zome_fn("hc_public")]
    fn assign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        let roles = hc_roles_mixin::handlers::get_agent_roles(&agent_address)?;
        if roles.len() == 0 {
            hc_roles_mixin::handlers::assign_role(&role_name, &agent_address)?;
            Ok(())
        } else {
            Err(ZomeApiError::Internal(
                "No se puede asignar mas de un role".into(),
            ))
        }
    }

    #[zome_fn("hc_public")]
    fn unassign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        hc_roles_mixin::handlers::unassign_role(&role_name, &agent_address)
    }
    #[zome_fn("hc_public")]
    fn get_users(data: String) -> ZomeApiResult<Vec<String>> {
        user::get_users(data)
    }

    #[zome_fn("hc_public")]
    fn get_agent_roles(agent_address: Address) -> ZomeApiResult<Role> {
        let roles = hc_roles_mixin::handlers::get_agent_roles(&agent_address)?;
        if roles.len() > 0 {
            Ok(roles[0].clone())
        } else if hc_roles_mixin::validation::is_agent_admin(&agent_address)? {
            hc_roles_mixin::handlers::assign_role(
                &String::from(hc_roles_mixin::ADMIN_ROLE_NAME),
                &agent_address,
            )?;
            hc_roles_mixin::handlers::get_role(&hc_roles_mixin::ADMIN_ROLE_NAME.to_string())
        } else {
            Err(ZomeApiError::Internal("No tiene rol".into()))
        }
    }
    #[zome_fn("hc_public")]
    fn create_page(title: String) -> ZomeApiResult<String> {
        page::create_page_if_non_existent(title.clone())?;
        Ok(title)
    }
    #[zome_fn("hc_public")]
    fn create_page_with_sections(sections: Vec<Section2>, title: String) -> ZomeApiResult<String> {
        page::create_page_with_sections(sections, title)
    }
    #[zome_fn("hc_public")]
    fn update_page(sections: Vec<Address>, title: String) -> ZomeApiResult<String> {
        page::update_page(sections, title)
    }
    #[zome_fn("hc_public")]
    fn get_page(title: String) -> ZomeApiResult<JsonString> {
        page::get_page(title)
    }
    #[zome_fn("hc_public")]
    fn get_titles() -> ZomeApiResult<Vec<String>> {
        page::get_titles()
    }
    #[zome_fn("hc_public")]
    fn get_titles_filtered(data: String) -> ZomeApiResult<Vec<String>> {
        page::get_titles_filtered(data)
    }
    #[zome_fn("hc_public")]
    fn get_usernames() -> ZomeApiResult<Vec<String>> {
        user::get_usernames()
    }
    #[zome_fn("hc_public")]
    fn get_username() -> ZomeApiResult<Option<String>> {
        Ok(user::get_user_by_agent_id(&AGENT_ADDRESS)?.pop())
    }
    #[zome_fn("hc_public")]
    fn get_agent_user(user_name: String) -> ZomeApiResult<Address> {
        user::get_agent_user(user_name)
    }
    #[zome_fn("hc_public")]
    fn get_section(address: Address) -> ZomeApiResult<JsonString> {
        utils::get_entry(address)
    }
    #[zome_fn("hc_public")]

    fn create_user(data: String) -> ZomeApiResult<String> {
        user::create_user_if_non_existent(data.clone())?;
        Ok(data)
    }

    #[zome_fn("hc_public")]

    fn get_user_by_agent_id(agent_id: Address) -> ZomeApiResult<String> {
        Ok(user::get_user_by_agent_id(&agent_id)?[0].clone())
    }
    #[zome_fn("hc_public")]
    fn update_element(address: Address, element: Section2) -> ZomeApiResult<Address> {
        section::update_element(address, element)
    }
    #[zome_fn("hc_public")]

    pub fn handle_receive_chat_message(message: String) -> ZomeApiResult<()> {
        // ...
        hdk::emit_signal(
            "message_received",
            JsonString::from_json(&format!("{{message: {}}}", message)),
        )?;
        // ...
        Ok(())
    }
    #[zome_fn("hc_public")]
    fn delete_element(address: Address) -> ZomeApiResult<String> {
        section::delete_element(address)
    }
    #[zome_fn("hc_public")]
    fn add_section(title: String, element: Section2) -> ZomeApiResult<Address> {
        section::add_section(title, element)
    }
}
