extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use crate::utils::validate_agent_can_edit;
use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    entry::Entry,
    // agent::AgentId, dna::entry_types::Sharing, entry::Entry, link::LinkMatch,
    link::LinkMatch,
};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::{Address, AddressableContent};
use hdk::prelude::*;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    //,
    // AGENT_ADDRESS, AGENT_ID_STR,
};

use crate::section::Section;

use holochain_anchors;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Page {
    pub title: String,
    pub sections: Vec<Address>,
    pub timestamp: String,
}

impl Page {
    pub fn from(title: String, sections: Vec<Address>, timestamp: String) -> Page {
        Page {
            title,
            sections,
            timestamp,
        }
    }
    pub fn entry(self) -> Entry {
        Entry::App("wikiPage".into(), self.into())
    }
}

pub fn page_def() -> ValidatingEntryType {
    entry!(
        name: "wikiPage",
        description: "this is an entry representing some profile info for an agent",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Page>| {
            match _validation_data {
                hdk::EntryValidationData::Create { validation_data, ..} => validate_agent_can_edit(validation_data)
                ,
                hdk::EntryValidationData::Modify { validation_data,new_entry,old_entry,..} => {
                    validate_agent_can_edit(validation_data)?;
                    if old_entry.title==new_entry.title{
                        Ok(())
                    }else{
                        Err("no se puede actualizar un titulo".to_string())
                    }

                },
                hdk::EntryValidationData::Delete { validation_data, ..} => validate_agent_can_edit(validation_data)
            }
        },
        links: [
            from!(
                holochain_anchors::ANCHOR_TYPE,
                link_type: "anchor->page",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    match _validation_data {
                        hdk::LinkValidationData::LinkAdd { validation_data, .. } => validate_agent_can_edit(validation_data),
                        hdk::LinkValidationData::LinkRemove { validation_data, .. } => validate_agent_can_edit(validation_data),
                    }
                }
            )
        ]

    )
}

pub fn create_page_if_non_existent(title: String, timestamp: String) -> ZomeApiResult<Address> {
    let anchor_address = holochain_anchors::anchor("wiki_pages".to_string(), title.clone())?;
    let option_address = hdk::get_links(
        &anchor_address,
        LinkMatch::Exactly("anchor->page"),
        LinkMatch::Any,
    )?;
    if let Some(address) = option_address.addresses().last() {
        Ok(address.clone())
    } else {
        let page_entry = Page::from(title.clone(), vec![], timestamp).entry();

        let address = hdk::commit_entry(&page_entry)?;
        hdk::link_entries(&anchor_address, &address, "anchor->page", "")?;
        Ok(address)
    }
}

pub fn create_page_with_sections(
    sections_entry: Vec<Section>,
    title: String,
    timestamp: String,
) -> ZomeApiResult<String> {
    let anchor_address = holochain_anchors::anchor("wiki_pages".to_string(), title.clone())?;
    let sections_address: ZomeApiResult<Vec<Address>> = sections_entry
        .into_iter()
        .map(|section| {
            let sections_entry = section.from(anchor_address.clone()).entry();
            hdk::commit_entry(&sections_entry)
        })
        .collect();
    let page_address = create_page_if_non_existent(title.clone(), timestamp.clone())?;
    let new_page_entry = Page::from(title.clone(), sections_address?, timestamp).entry();
    let new_address = hdk::update_entry(new_page_entry, &page_address)?;
    hdk::link_entries(
        &holochain_anchors::anchor("wiki_pages".into(), title.clone())?,
        &new_address,
        "anchor->page",
        "",
    )?;
    Ok(title)
}

pub fn update_page(
    sections: Vec<Address>,
    title: String,
    timestamp: String,
) -> ZomeApiResult<String> {
    let page_address = create_page_if_non_existent(title.clone(), timestamp.clone())?;
    let new_page_entry = Page::from(title.clone(), sections, timestamp).entry();
    let new_address = hdk::update_entry(new_page_entry, &page_address)?;
    hdk::link_entries(
        &holochain_anchors::anchor("wiki_pages".into(), title.clone())?,
        &new_address,
        "anchor->page",
        "",
    )?;
    Ok(title)
}

pub fn get_page(title: String) -> ZomeApiResult<Page> {
    let option_address = hdk::get_links(
        &holochain_anchors::anchor("wiki_pages".into(), title)?,
        LinkMatch::Exactly("anchor->page"),
        LinkMatch::Any,
    )?;
    if let Some(address) = option_address.addresses().last() {
        hdk::utils::get_as_type(address.clone())
    } else {
        Err(ZomeApiError::Internal("This page no exist".to_string()))
    }
}

pub fn get_titles() -> ZomeApiResult<Vec<String>> {
    get_titles_filtered("".to_string())
}

pub fn get_titles_filtered(data: String) -> ZomeApiResult<Vec<String>> {
    let anchor_address = Entry::App(
        holochain_anchors::ANCHOR_TYPE.into(),
        holochain_anchors::Anchor {
            anchor_type: "wiki_pages".to_string(),
            anchor_text: None,
        }
        .into(),
    )
    .address();

    let titles = hdk::get_links_with_options(
        &anchor_address,
        LinkMatch::Exactly("holochain_anchors::anchor_link").into(),
        LinkMatch::Any,
        GetLinksOptions::default(),
    )?
    .tags()
    .into_iter()
    .filter_map(|text| {
        if text.clone().contains(&data) {
            Some(text)
        } else {
            None
        }
    })
    .collect();

    Ok(titles)
}
