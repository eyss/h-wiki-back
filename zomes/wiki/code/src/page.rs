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
    error::ZomeApiResult,
    //,
    // AGENT_ADDRESS, AGENT_ID_STR,
};

use crate::section::{Section, Section2};
use holochain_anchors;

use crate::utils;
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Page {
    pub title: String,
    pub sections: Vec<Address>,
    pub timestamp: Option<u64>,
}
impl Page {
    pub fn initial(title: String) -> Page {
        Page {
            title,
            sections: vec![],
            timestamp: None,
        }
    }

    pub fn from(title: String, sections: Vec<Address>, timestamp: Option<u64>) -> Page {
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
                hdk::EntryValidationData::Create { validation_data, ..} => validate_agent_can_edit(validation_data),
                hdk::EntryValidationData::Modify { validation_data, ..} => validate_agent_can_edit(validation_data),
                hdk::EntryValidationData::Delete { validation_data, ..} => validate_agent_can_edit(validation_data)
            }
        },
        links: [
            from!(
                holochain_anchors::ANCHOR_TYPE,
                link_type: "anchor->Page",
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

pub fn create_page_if_non_existent(title: String) -> ZomeApiResult<Address> {
    let address = Page::initial(title.clone()).entry().address();
    match hdk::get_entry(&address)? {
        None => {
            let page_anchor = utils::anchor("wiki_pages", "all_pages")?;
            hdk::utils::commit_and_link(
                &Page::initial(title.clone()).entry(),
                &page_anchor,
                "anchor->Page",
                &title,
            )
        }
        Some(_) => Ok(address),
    }
}

pub fn create_page_with_sections(
    sections: Vec<Section2>,
    title: String,
    timestamp: u64,
) -> ZomeApiResult<String> {
    let page_address = create_page_if_non_existent(title.clone())?;
    let sections: Vec<Address> = sections
        .into_iter()
        .map(|element| {
            hdk::api::commit_entry(&Section::from(element, page_address.clone()).entry())
        })
        .filter_map(Result::ok)
        .collect();

    hdk::api::update_entry(
        Page::from(title.clone(), sections, Some(timestamp)).entry(),
        &page_address,
    )?;

    Ok(title)
}
pub fn update_page(sections: Vec<Address>, title: String, timestamp: u64) -> ZomeApiResult<String> {
    let page_address = create_page_if_non_existent(title.clone())?;
    hdk::api::update_entry(
        Page::from(title.clone(), sections, Some(timestamp)).entry(),
        &page_address,
    )?;
    Ok(title)
}
pub fn get_page(title: String) -> ZomeApiResult<JsonString> {
    utils::get_entry(Page::initial(title).entry().address())
}

pub fn get_titles() -> ZomeApiResult<Vec<String>> {
    let anchor_address = utils::anchor("wiki_pages", "all_pages")?;
    Ok(hdk::utils::get_links_and_load_type::<Page>(
        &anchor_address,
        LinkMatch::Exactly("anchor->Page".into()),
        LinkMatch::Any,
    )?
    .into_iter()
    .map(|page| page.title)
    .collect())
}

pub fn get_titles_filtered(data: String) -> ZomeApiResult<Vec<String>> {
    let anchor_address = utils::anchor("wiki_pages", "all_pages")?;
    Ok(hdk::utils::get_links_and_load_type::<Page>(
        &anchor_address,
        LinkMatch::Exactly("anchor->Page".into()),
        LinkMatch::Regex(&("^".to_owned() + &data)),
    )?
    .into_iter()
    .map(|page| page.title)
    .collect())
}
