use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::{dna::entry_types::Sharing, entry::Entry},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};
use holochain_anchors;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Page {
    pub title: String,
    pub sections: Vec<Address>,
}

impl Page {
    pub fn initial(title: String) -> Page {
        Page {
            title,
            sections: vec![],
        }
    }

    pub fn from(title: String, sections: Vec<Address>) -> Page {
        Page { title, sections }
    }
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: "wikiPage",
        description: "this is an entry representing some profile info for an agent",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Page>| {
            Ok(())
        },
        links: [
            from!(
                holochain_anchors::ANCHOR_TYPE,
                link_type: "anchor->wikiPage",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]

    )
}

/** Helper functions */

pub fn page_entry(page: Page) -> Entry {
    Entry::App("wikiPage".into(), page.into())
}

pub fn page_address(page: Page) -> ZomeApiResult<Address> {
    hdk::entry_address(&page_entry(page))
}

pub fn page_address_by_title(title: String) -> ZomeApiResult<Address> {
    let page = Page::initial(title);

    page_address(page)
}

pub fn pages_anchor() -> ZomeApiResult<Address> {
    holochain_anchors::create_anchor("wiki_pages".into(), "all_pages".into())
}

/** Public handlers */

pub fn create_page_if_non_existent(title: String) -> ZomeApiResult<Address> {
    let page = Page::initial(title);

    let entry = page_entry(page);

    let address = hdk::entry_address(&entry)?;

    match hdk::get_entry(&address)? {
        None => {
            let entry_address = hdk::commit_entry(&entry)?;
            let page_anchor = pages_anchor()?;

            hdk::link_entries(&page_anchor, &entry_address, "anchor->wikiPage", "")?;

            Ok(entry_address)
        }
        Some(_) => Ok(address),
    }
}

pub fn update_page(title: String, sections: Vec<Address>) -> ZomeApiResult<()> {
    let address = page_address_by_title(title.clone())?;
    let page = Page::from(title, sections);
    let page_entry = page_entry(page);

    hdk::update_entry(page_entry, &address)?;

    Ok(())
}
