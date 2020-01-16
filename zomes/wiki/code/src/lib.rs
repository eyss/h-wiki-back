#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

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
    // AGENT_ADDRESS,
    // AGENT_ADDRESS, AGENT_ID_STR,
};
use hdk_proc_macros::zome;
use holochain_anchors;
// see https://developer.holochain.org/api/0.0.25-alpha1/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct User {
    username: String,
    agent_adress: Address,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum Content {
    Text(String),
    Binarys(Vec<u8>),
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PageElement {
    page_address: Option<Address>,
    r#type: String,
    content: String,
    rendered_content: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct WikiPage {
    title: String,
    sections: Vec<Address>,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Section {
    address: Address,
    section: PageElement,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Page {
    title: String,
    sections: Vec<Section>,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct HomePage {
    title: String,
    sections: Vec<PageElement>,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct Anchor {
    anchor_type: String,
    anchor_text: Option<String>,
}
impl WikiPage {
    fn entry(self) -> Entry {
        App("wikiPage".into(), self.into())
    }
    fn update(self, address: Address) -> ZomeApiResult<Address> {
        let entry = App("wikiPage".into(), self.into());
        hdk::api::update_entry(entry, &address)
    }
}

impl PageElement {
    fn entry(self) -> Entry {
        App("pageElement".into(), self.into())
    }
    fn update(self, address: Address) -> ZomeApiResult<Address> {
        let entry = App("pageElement".into(), self.into());
        hdk::api::update_entry(entry, &address)
    }
}
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
    fn anchor_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }
    #[entry_def]
    fn user_def() -> ValidatingEntryType {
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
                    "%agent_id",
                    link_type: "agent->user",
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
    #[entry_def]
    fn page_def() -> ValidatingEntryType {
        entry!(
            name: "wikiPage",
            description: "this is an entry representing some profile info for an agent",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<WikiPage>| {
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
    #[entry_def]
    fn page_element_def() -> ValidatingEntryType {
        entry!(
            name: "pageElement",
            description: "this is an entry representing some profile info for an agent",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<PageElement>| {
                Ok(())
            },
            links: [
                from!(
                    holochain_anchors::ANCHOR_TYPE,
                    link_type: "wikiPage->pageElement",
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
    // #[zome_fn("hc_public")]
    // fn create_user(user:User)->ZomeApiResult<Address>{
    //     let user_entry=Entry::App("user".into(),user.into());
    //     let anchor_address = holochain_anchors::create_anchor("model".into(), "soft-tail".into())?;
    //     let user_adress=hdk::utils::commit_and_link(&user_entry,&anchor_address,"anchor->user","")?;
    //     hdk::api::link_entries(&AGENT_ADDRESS,&user_adress,"agent->user","")?;
    //     Ok(user_adress)
    // }
    #[zome_fn("hc_public")]
    fn create_page(title: String) -> ZomeApiResult<String> {
        inner_create_page(title.clone())?;
        Ok(title)
    }
    fn inner_create_page(title: String) -> ZomeApiResult<Address> {
        let anchor_address = holochain_anchors::create_anchor("WikiPage".into(), "".into())?;
        hdk::utils::commit_and_link(
            &WikiPage {
                title: title.clone(),
                sections: Vec::<Address>::new(),
            }
            .entry(),
            &anchor_address,
            "anchor->wikiPage",
            &title,
        )
    }
    #[zome_fn("hc_public")]
    fn create_page_with_elements(
        contents: Vec<PageElement>,
        title: String,
    ) -> ZomeApiResult<String> {
        let page_address = inner_create_page(title.clone())?;

        let vector: Vec<Address> = contents
            .into_iter()
            .map(|mut element| {
                element.page_address = Some(page_address.clone());
                hdk::utils::commit_and_link(
                    &element.entry(),
                    &page_address,
                    "wikiPage->pageElement",
                    "",
                )
            })
            .filter_map(Result::ok)
            .collect();

        WikiPage {
            title: title.clone(),
            sections: vector,
        }
        .update(page_address)?;
        Ok(title)
    }
    #[zome_fn("hc_public")]
    fn get_page(title: String) -> ZomeApiResult<Page> {
        match hdk::utils::get_as_type::<WikiPage>(
            WikiPage {
                title: title.clone(),
                sections: Vec::<Address>::new(),
            }
            .entry()
            .address(),
        ) {
            Ok(t) => Ok(Page {
                title: title,
                sections: t
                    .sections
                    .into_iter()
                    .map(
                        |address| match hdk::utils::get_as_type::<PageElement>(address.clone()) {
                            Ok(r) => Ok(Section {
                                address: address,
                                section: r,
                            }),
                            Err(r) => Err(r),
                        },
                    )
                    .filter_map(Result::ok)
                    .collect(),
            }),
            Err(r) => Err(r),
        }
    }
    fn get_titles() -> ZomeApiResult<Vec<String>> {
        let anchor_address = holochain_anchors::create_anchor("WikiPage".into(), "".into())?;
        Ok(hdk::utils::get_links_and_load_type::<WikiPage>(
            &anchor_address,
            LinkMatch::Exactly("anchor->wikiPage".into()),
            LinkMatch::Any,
        )?
        .into_iter()
        .map(|page| page.title)
        .collect())
    }
    #[zome_fn("hc_public")]
    fn get_home_page() -> ZomeApiResult<HomePage> {
        let vec = get_titles()?
            .into_iter()
            .map(|strin| PageElement {
                page_address: None,
                r#type: "text".to_string(),
                content: format!("[{}](#)", strin),
                rendered_content: format!("<a href='#'>{}</a>", strin),
            })
            .collect();
        Ok(HomePage {
            title: "home page".to_string(),
            sections: vec,
        })
    }
    fn get_page_address_by_element_address(element_address: Address) -> ZomeApiResult<Address> {
        let element = hdk::utils::get_as_type::<PageElement>(element_address)?;
        Ok(element.page_address.unwrap())
    }
    #[zome_fn("hc_public")]
    fn delete_element(element_address: Address) -> ZomeApiResult<String> {
        let page_address = get_page_address_by_element_address(element_address.clone())?;
        if let Ok(t) = hdk::utils::get_as_type::<WikiPage>(page_address.clone()) {
            let mut t = t;
            inner_delete_element(&mut t.sections, element_address.clone())?;
            t.clone().update(page_address)?;
            Ok(t.title)
        } else {
            Err(hdk::error::ZomeApiError::Internal("error".into()))
        }
    }
    fn inner_delete_element(vector: &mut Vec<Address>, address: Address) -> ZomeApiResult<Address> {
        match vector.pop() {
            Some(t) => {
                if t == address {
                    hdk::api::remove_entry(&address)
                } else {
                    inner_delete_element(vector, address.clone())?;
                    vector.push(t);
                    Ok(address)
                }
            }
            _ => return Ok(address),
        }
    }
    #[zome_fn("hc_public")]
    fn update_element(address: Address, element: PageElement) -> ZomeApiResult<Address> {
        let old_element = hdk::utils::get_as_type::<PageElement>(address.clone())?;
        PageElement {
            page_address: old_element.page_address,
            r#type: element.r#type,
            content: element.content,
            rendered_content: element.rendered_content,
        }
        .update(address)
    }
    #[zome_fn("hc_public")]
    fn add_page_element(element: PageElement, title: String) -> ZomeApiResult<String> {
        let page_adress = WikiPage {
            title: title.clone(),
            sections: Vec::<Address>::new(),
        }
        .entry()
        .address();
        let mut element = element;
        element.page_address = Some(page_adress.clone());

        let address = hdk::utils::commit_and_link(
            &element.entry(),
            &page_adress,
            "wikiPage->pageElement",
            "",
        )?;
        match hdk::utils::get_as_type::<WikiPage>(page_adress.clone()) {
            Ok(t) => {
                let mut vector = vec![address.clone()];
                vector.extend(t.sections);
                WikiPage {
                    title: t.title,
                    sections: vector,
                }
                .update(page_adress)?;
                Ok(title)
            }
            Err(r) => Err(r),
        }
    }
    fn ordenar(mut vector: &mut Vec<Address>, address: Address, before_element_address: Address) {
        match vector.pop() {
            Some(t) => {
                if t == before_element_address {
                    vector.push(t);
                    vector.push(address);
                } else {
                    ordenar(&mut vector, address, before_element_address);
                    vector.push(t);
                };
            }
            _ => return,
        }
    }
    #[zome_fn("hc_public")]
    fn add_page_element_ordered(
        element: PageElement,
        before_element_address: Address,
    ) -> ZomeApiResult<String> {
        inner_add_page_element_ordered(element, before_element_address)
    }
    fn inner_add_page_element_ordered(
        element: PageElement,
        before_element_address: Address,
    ) -> ZomeApiResult<String> {
        let page_address = get_page_address_by_element_address(before_element_address.clone())?;
        let title = hdk::utils::get_as_type::<WikiPage>(page_address.clone())?.title;
        let mut element = element;
        element.page_address = Some(page_address.clone());
        let address = hdk::utils::commit_and_link(
            &element.entry(),
            &page_address,
            "wikiPage->pageElement",
            "",
        )?;

        match hdk::utils::get_as_type::<WikiPage>(page_address.clone()) {
            Ok(t) => {
                let mut vector = t.sections;

                ordenar(&mut vector, address.clone(), before_element_address);
                WikiPage {
                    title: t.title,
                    sections: vector,
                }
                .update(page_address)?;
                Ok(title)
            }
            Err(r) => Err(r),
        }
    }
}
