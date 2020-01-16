use crate::page;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{dna::entry_types::Sharing, entry::Entry},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Section {
    pub page_address: Address,
    pub r#type: String,
    pub content: String,
    pub rendered_content: String,
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: "section",
        description: "this is an entry representing a section inside a page",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Section>| {
            Ok(())
        }
    )
}

/** Helpers */

pub fn section_entry(section: Section) -> Entry {
    Entry::App("section".into(), section.into())
}

/** Public handlers */

pub fn create_section(
    page_title: String,
    r#type: String,
    content: String,
    rendered_content: String,
) -> ZomeApiResult<Address> {
    let page_address = page::page_address_by_title(page_title.clone())?;

    let section = Section {
        page_address,
        r#type,
        content,
        rendered_content,
    };

    let section_entry = section_entry(section);

    let section_address = hdk::commit_entry(&section_entry)?;

    let page: page::Page = hdk::utils::get_as_type(page_address)?;

    let mut sections = page.sections;
    sections.push(section_address.clone());

    page::update_page(page_title.clone(), sections)?;

    Ok(section_address)
}

pub fn update_section(section_address: Address, section: Section) -> ZomeApiResult<()> {
    let section_entry = section_entry(section);
    hdk::update_entry(section_entry, &section_address)?;

    Ok(())
}
