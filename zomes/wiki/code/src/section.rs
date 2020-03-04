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
};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::*;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    //,
    // AGENT_ADDRESS, AGENT_ID_STR,
};

use crate::page::Page;
use crate::utils::validate_agent_can_edit;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Section {
    anchor_address: Option<Address>,
    r#type: String,
    content: String,
    rendered_content: String,
    timestamp: String,
}

impl Section {
    pub fn from(self, anchor_address: Address) -> Section {
        Section {
            anchor_address: Some(anchor_address),
            r#type: self.r#type,
            content: self.content,
            rendered_content: self.rendered_content,
            timestamp: self.timestamp,
        }
    }
    pub fn entry(self) -> Entry {
        Entry::App("pageSection".into(), self.into())
    }
}
pub fn page_section_def() -> ValidatingEntryType {
    entry!(
        name: "pageSection",
        description: "this is an entry representing some profile info for an agent",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Section>| {
            match _validation_data {
                hdk::EntryValidationData::Create { validation_data, .. } => validate_agent_can_edit(validation_data),
                hdk::EntryValidationData::Modify { validation_data, .. } => validate_agent_can_edit(validation_data),
                hdk::EntryValidationData::Delete { validation_data, .. } => validate_agent_can_edit(validation_data)
            }
        }
    )
}
pub fn update_element(address: Address, mut section: Section) -> ZomeApiResult<Address> {
    let old_element = hdk::utils::get_as_type::<Section>(address.clone())?;
    let anchor_address_option = old_element.anchor_address;
    section.anchor_address = anchor_address_option.clone();
    let new_address = hdk::api::update_entry(section.entry(), &address)?;
    if let Some(anchor_address) = anchor_address_option {
        let page_address = hdk::get_links(
            &anchor_address,
            LinkMatch::Exactly("anchor->page"),
            LinkMatch::Any,
        )?
        .addresses()[0]
            .clone();
        let page: Page = hdk::utils::get_as_type(page_address.clone())?;
        let sections = page
            .clone()
            .sections
            .into_iter()
            .filter_map(|d_address| {
                if d_address != address {
                    Some(d_address)
                } else {
                    Some(new_address.clone())
                }
            })
            .collect();

        hdk::api::update_entry(
            Page::from(page.title.clone(), sections, page.timestamp).entry(),
            &page_address,
        )?;
    };
    Ok(new_address)
}
pub fn delete_element(address: Address) -> ZomeApiResult<String> {
    let anchor_address_option = hdk::utils::get_as_type::<Section>(address.clone())?.anchor_address;
    //hdk::api::remove_entry(&address)?;
    if let Some(anchor_address) = anchor_address_option {
        let page_address = hdk::get_links(
            &anchor_address,
            LinkMatch::Exactly("anchor->page"),
            LinkMatch::Any,
        )?
        .addresses()[0]
            .clone();
        let page: Page = hdk::utils::get_as_type(page_address.clone())?;
        let sections = page
            .clone()
            .sections
            .into_iter()
            .filter_map(|d_address| {
                if d_address != address {
                    Some(d_address)
                } else {
                    None
                }
            })
            .collect();

        hdk::api::update_entry(
            Page::from(page.title.clone(), sections, page.timestamp).entry(),
            &page_address,
        )?;
        hdk::api::remove_entry(&address)?;
        Ok(page.title)
    } else {
        Ok("t".to_string())
    }
}
pub fn add_section(title: String, section: Section) -> ZomeApiResult<Address> {
    let anchor_address = holochain_anchors::anchor("wiki_pages".into(), title)?;
    hdk::api::commit_entry(&section.from(anchor_address).entry())
}
