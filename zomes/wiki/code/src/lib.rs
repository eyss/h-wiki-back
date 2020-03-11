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
use hdk::prelude::*;
use hdk_proc_macros::zome;
mod page;
mod section;
mod user;
mod utils;
pub const EDITOR_ROLE_NAME: &'static str = "Editor";

#[zome]
mod wiki {
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
        holochain_roles::role_assignment_entry_def()
    }

    #[entry_def]
    fn anchor_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }

    #[entry_def]
    fn page_def() -> ValidatingEntryType {
        page::page_def()
    }

    #[entry_def]
    fn page_section_def() -> ValidatingEntryType {
        section::page_section_def()
    }

    #[entry_def]
    fn user_def() -> ValidatingEntryType {
        user::user_def()
    }

    #[zome_fn("hc_public")]
    fn assign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        holochain_roles::handlers::assign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn unassign_role(role_name: String, agent_address: Address) -> ZomeApiResult<()> {
        holochain_roles::handlers::unassign_role(&role_name, &agent_address)
    }

    #[zome_fn("hc_public")]
    fn get_agent_roles(agent_address: Address) -> ZomeApiResult<Option<String>> {
        let roles = holochain_roles::handlers::get_agent_roles(&agent_address)?;

        let admin_role = String::from(holochain_roles::ADMIN_ROLE_NAME);

        if roles.len() > 1 && roles.contains(&admin_role) {
            return Ok(Some(admin_role));
        } else {
            return Ok(roles.get(0).map(|s| s.clone()));
        }
    }

    #[zome_fn("hc_public")]
    fn create_page(title: String, timestamp: String) -> ZomeApiResult<String> {
        page::create_page_if_non_existent(title.clone(), timestamp)?;
        Ok(title)
    }

    #[zome_fn("hc_public")]
    fn create_page_with_sections(
        sections: Vec<section::Section>,
        title: String,
        timestamp: String,
    ) -> ZomeApiResult<String> {
        page::create_page_with_sections(sections, title, timestamp)
    }

    #[zome_fn("hc_public")]
    fn update_page(
        sections: Vec<Address>,
        title: String,
        timestamp: String,
    ) -> ZomeApiResult<String> {
        page::update_page(sections, title, timestamp)
    }

    #[zome_fn("hc_public")]
    fn get_page(title: String) -> ZomeApiResult<page::Page> {
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
    fn update_section(address: Address, section: section::Section) -> ZomeApiResult<Address> {
        section::update_section(address, section)
    }

    #[zome_fn("hc_public")]
    fn delete_section(address: Address) -> ZomeApiResult<String> {
        section::delete_section(address)
    }

    #[zome_fn("hc_public")]
    fn add_section(title: String, section: section::Section) -> ZomeApiResult<Address> {
        section::add_section(title, section)
    }

    #[zome_fn("hc_public")]
    fn get_usernames() -> ZomeApiResult<Vec<String>> {
        user::get_usernames()
    }

    #[zome_fn("hc_public")]
    fn get_username() -> ZomeApiResult<Option<String>> {
        Ok(user::get_user_by_agent_id(&hdk::AGENT_ADDRESS)?.pop())
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
    fn get_users(data: String) -> ZomeApiResult<Vec<String>> {
        user::get_users(data)
    }
}
