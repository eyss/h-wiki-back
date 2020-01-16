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
use hdk::holochain_persistence_api::cas::content::Address;
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

pub mod page;
pub mod section;

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
pub struct WikiPage {
    page_name: Address,
    redered_page_element: Vec<Address>,
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
    fn page_def() -> ValidatingEntryType {
        page::entry_definition()
    }

    #[entry_def]
    fn section_def() -> ValidatingEntryType {
        section::entry_definition()
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
    // #[zome_fn("hc_public")]
    // fn create_user(user:User)->ZomeApiResult<Address>{
    //     let user_entry=Entry::App("user".into(),user.into());
    //     let anchor_address = holochain_anchors::create_anchor("model".into(), "soft-tail".into())?;
    //     let user_adress=hdk::utils::commit_and_link(&user_entry,&anchor_address,"anchor->user","")?;
    //     hdk::api::link_entries(&AGENT_ADDRESS,&user_adress,"agent->user","")?;
    //     Ok(user_adress)
    // }
    #[zome_fn("hc_public")]
    fn create_page(title: String) -> ZomeApiResult<Address> {
        page::create_page_if_non_existent(title)
    }

    #[zome_fn("hc_public")]
    fn create_page_with_elements(
        contents: Vec<PageElement>,
        title: String,
    ) -> ZomeApiResult<String> {
        let page_anchor_address = get_anchor_address("wikiPage".into(), title.clone().into())?;

        let vector: Vec<Address> = contents
            .into_iter()
            .map(|mut element| {
                element.parent_page_anchor = Some(page_anchor_address.clone());
                Entry::App("pageElement".into(), element.into())
            })
            .map(|elemente_entry| {
                hdk::utils::commit_and_link(
                    &elemente_entry,
                    &page_anchor_address,
                    "anchor->pageElement",
                    &title,
                )
            })
            .filter_map(Result::ok)
            .collect();
        let page_entry = Entry::App(
            "wikiPage".into(),
            WikiPage {
                page_name: page_anchor_address.clone(),
                redered_page_element: vector,
            }
            .into(),
        );

        hdk::utils::commit_and_link(&page_entry, &page_anchor_address, "anchor->wikiPage", "")?;
        Ok(title)
    }
    // fn get_page_by_anchor(address: Address) -> ZomeApiResult<Address> {
    //     hdk::api::get_links(
    //         &address,
    //         LinkMatch::Exactly("anchor->wikiPage".into()),
    //         LinkMatch::Any,
    //     )?
    //     .addresses()
    //     .pop()
    //     .ok_or(hdk::error::ZomeApiError::Internal("error".into()))
    // }
    fn get_page_address_by_title(title: String) -> ZomeApiResult<Address> {
        let anchor_address = get_anchor_address("wikiPage".into(), title.into())?;
        hdk::api::get_links(
            &anchor_address,
            LinkMatch::Exactly("anchor->wikiPage".into()),
            LinkMatch::Any,
        )?
        .addresses()
        .pop()
        .ok_or(hdk::error::ZomeApiError::Internal("error".into()))
    }
    fn get_page_address(anchor_address: Address) -> ZomeApiResult<Address> {
        hdk::api::get_links(
            &anchor_address,
            LinkMatch::Exactly("anchor->wikiPage".into()),
            LinkMatch::Any,
        )?
        .addresses()
        .pop()
        .ok_or(hdk::error::ZomeApiError::Internal("error".into()))
    }

    fn get_anchor_pages(anchor_type: String) -> ZomeApiResult<Vec<Address>> {
        holochain_anchors::get_anchors()?
            .into_iter()
            .map(|address| {
                if let Ok(Some(App(_, json))) = hdk::api::get_entry(&address) {
                    if let Ok(anchor) = serde_json::from_str::<Anchor>(&Into::<String>::into(json))
                    {
                        Ok(if anchor.anchor_type == anchor_type {
                            Ok(address)
                        } else {
                            Err(hdk::error::ZomeApiError::Internal("error".into()))
                        })
                    } else {
                        Err(hdk::error::ZomeApiError::Internal("error".into()))
                    }
                } else {
                    Err(hdk::error::ZomeApiError::Internal("error".into()))
                }
            })
            .filter_map(Result::ok)
            .collect()
    }
    fn get_title(address: Address) -> ZomeApiResult<String> {
        if let Ok(Some(App(_, json))) = hdk::api::get_entry(&address) {
            if let Ok(anchor) = serde_json::from_str::<Anchor>(&Into::<String>::into(json)) {
                Ok(anchor.anchor_text.unwrap())
            } else {
                Err(hdk::error::ZomeApiError::Internal("vacio".into()))
            }
        } else {
            Err(hdk::error::ZomeApiError::Internal("vacio".into()))
        }
    }
    fn get_titles() -> ZomeApiResult<Vec<String>> {
        match get_anchor_pages("wikiPage".to_string())?.pop() {
            Some(address) => Ok(
                hdk::api::get_links(&address, LinkMatch::Any, LinkMatch::Any)?
                    .addresses()
                    .into_iter()
                    .map(|address| get_title(address))
                    .filter_map(Result::ok)
                    .collect(),
            ),
            None => Err(hdk::error::ZomeApiError::Internal("vacio".into())),
        }
    }
    fn get_anchor_address(anchor_type: String, title: String) -> ZomeApiResult<Address> {
        match get_anchor_pages(anchor_type.clone())?.pop() {
            Some(address) => {
                match hdk::api::get_links(&address, LinkMatch::Any, LinkMatch::Exactly(&title))?
                    .addresses()
                    .pop()
                {
                    Some(t) => Ok(t),
                    None => holochain_anchors::create_anchor(anchor_type.into(), title.into()),
                }
            }
            None => holochain_anchors::create_anchor(anchor_type.into(), title.into()),
        }
    }
    #[zome_fn("hc_public")]
    fn get_page(title: String) -> ZomeApiResult<Page> {
        match hdk::utils::get_as_type::<WikiPage>(get_page_address_by_title(title.clone())?) {
            Ok(t) => Ok(Page {
                title: title,
                sections: t
                    .redered_page_element
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
    #[zome_fn("hc_public")]
    fn get_home_page() -> ZomeApiResult<HomePage> {
        let vec = get_titles()?
            .into_iter()
            .map(|strin| PageElement {
                parent_page_anchor: None,
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
        let element = get_element(element_address.clone());
        get_page_address(element?.parent_page_anchor.unwrap())
    }
    fn get_title_by_element_address(element_address: Address) -> ZomeApiResult<String> {
        let element = get_element(element_address.clone());
        get_title(element?.parent_page_anchor.unwrap())
    }
    #[zome_fn("hc_public")]
    fn delete_element(element_address: Address) -> ZomeApiResult<String> {
        let page_address = get_page_address_by_element_address(element_address.clone())?;
        if let Ok(t) = hdk::utils::get_as_type::<WikiPage>(page_address.clone()) {
            let mut t = t;
            inner_delete_element(&mut t.redered_page_element, element_address.clone())?;
            hdk::api::update_entry(Entry::App("wikiPage".into(), t.into()), &page_address)?;
        }
        get_title_by_element_address(element_address)
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
        let old_element = get_element(address.clone())?;
        let element_entry = Entry::App(
            "pageElement".into(),
            PageElement {
                parent_page_anchor: old_element.parent_page_anchor,
                r#type: element.r#type,
                content: element.content,
                rendered_content: element.rendered_content,
            }
            .into(),
        );
        hdk::api::update_entry(element_entry, &address)
    }
    #[zome_fn("hc_public")]
    fn get_elements_page(title: String) -> ZomeApiResult<Vec<PageElement>> {
        let address = get_page_address_by_title(title)?;
        inner_get_elements_page(address)
    }
    fn inner_get_elements_page(address: Address) -> ZomeApiResult<Vec<PageElement>> {
        let vector = hdk::utils::get_as_type::<WikiPage>(address)?.redered_page_element;
        vector
            .into_iter()
            .map(|address: Address| hdk::utils::get_as_type::<PageElement>(address))
            .collect()
    }
    fn get_element(address: Address) -> ZomeApiResult<PageElement> {
        hdk::utils::get_as_type(address)
    }

    #[zome_fn("hc_public")]
    fn add_page_element(element: PageElement, title: String) -> ZomeApiResult<String> {
        let page_adress = get_page_address_by_title(title.clone())?;
        let elements_anchor_address = get_anchor_address("wikiPage".into(), title.clone().into())?;
        let mut element = element;
        element.parent_page_anchor = Some(elements_anchor_address.clone());
        let element_entry = Entry::App("pageElement".into(), element.into());

        let address = hdk::utils::commit_and_link(
            &element_entry,
            &elements_anchor_address,
            "anchor->pageElement",
            "",
        )?;
        match hdk::utils::get_as_type::<WikiPage>(page_adress.clone()) {
            Ok(t) => {
                let mut vector = vec![address.clone()];
                vector.extend(t.redered_page_element);
                hdk::api::update_entry(
                    Entry::App(
                        "wikiPage".into(),
                        WikiPage {
                            page_name: t.page_name,
                            redered_page_element: vector,
                        }
                        .into(),
                    ),
                    &page_adress,
                )?;
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
        let title = get_title_by_element_address(before_element_address.clone())?;
        let page_address = get_page_address_by_element_address(before_element_address.clone())?;
        let elements_anchor_address = get_anchor_address("wikiPage".into(), title.clone().into())?;
        let mut element = element;
        element.parent_page_anchor = Some(elements_anchor_address.clone());
        let element_entry = Entry::App("pageElement".into(), element.into());
        let address = hdk::utils::commit_and_link(
            &element_entry,
            &elements_anchor_address,
            "anchor->pageElement",
            "",
        )?;

        match hdk::utils::get_as_type::<WikiPage>(page_address.clone()) {
            Ok(t) => {
                let mut vector = t.redered_page_element;

                ordenar(&mut vector, address.clone(), before_element_address);

                hdk::api::update_entry(
                    Entry::App(
                        "wikiPage".into(),
                        WikiPage {
                            page_name: t.page_name,
                            redered_page_element: vector,
                        }
                        .into(),
                    ),
                    &page_address,
                )?;
                Ok(title)
            }
            Err(r) => Err(r),
        }
    }
}
