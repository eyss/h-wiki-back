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
        Entry::App("page_section".into(), self.into())
    }
}

pub fn page_section_def() -> ValidatingEntryType {
    entry!(
        name: "page_section",
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

pub fn update_section(
    old_section_address: Address,
    mut section: Section,
) -> ZomeApiResult<Address> {
    let old_section = hdk::utils::get_as_type::<Section>(old_section_address.clone())?;

    let anchor_address_option = old_section.anchor_address;

    section.anchor_address = anchor_address_option.clone();

    let new_address = hdk::update_entry(section.entry(), &old_section_address)?;

    if let Some(anchor_address) = anchor_address_option {
        let option_page_address = hdk::get_links(
            &anchor_address,
            LinkMatch::Exactly("anchor->page"),
            LinkMatch::Any,
        )?;
        if let Some(page_address) = option_page_address.addresses().last() {
            let page: Page = hdk::utils::get_as_type(page_address.clone())?;

            let sections = page
                .clone()
                .sections
                .into_iter()
                .filter_map(|o_address| {
                    if o_address != old_section_address {
                        Some(o_address)
                    } else {
                        Some(new_address.clone())
                    }
                })
                .collect();

            hdk::update_entry(
                Page::from(page.title.clone(), sections, page.timestamp).entry(),
                &page_address,
            )?;
        }
    };
    Ok(new_address)
}

pub fn delete_section(address: Address) -> ZomeApiResult<String> {
    let anchor_address = hdk::utils::get_as_type::<Section>(address.clone())?
        .anchor_address
        .unwrap();
    //hdk::remove_entry(&address)?;
    let page_address = match hdk::get_links(
        &anchor_address,
        LinkMatch::Exactly("anchor->page"),
        LinkMatch::Any,
    )?
    .addresses()
    .first()
    {
        Some(address) => Ok(address),
        None => Err(ZomeApiError::Internal("This page no exist".to_string())),
    }?
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
    let new_page_entry = Page::from(page.title.clone(), sections, page.timestamp).entry();
    hdk::update_entry(new_page_entry, &page_address)?;
    hdk::remove_entry(&address)?;
    Ok(page.title)
}

pub fn add_section(title: String, section: Section) -> ZomeApiResult<Address> {
    let anchor_address = holochain_anchors::anchor("wiki_pages".into(), title)?;
    hdk::commit_entry(&section.from(anchor_address).entry())
}
