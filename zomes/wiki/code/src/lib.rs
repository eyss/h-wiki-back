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
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    // AGENT_ADDRESS,
    // AGENT_ADDRESS, AGENT_ID_STR,
};

use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
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
pub struct PageElement {
    parent_page_anchor: Option<Address>,
    element_type: String,
    element_content: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct WikiPage {
    page_name: Address,
    titulo: String,
    redered_page_element: Vec<Address>,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Hola {
    page_name: Address,
    page: WikiPage,
    vector_address: Vec<Address>,
    redered_page_element: Vec<PageElement>,
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
                    link_type: "anchor->pageElement",
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
    fn create_page(titulo: String) -> ZomeApiResult<Address> {
        let page_anchor_address = holochain_anchors::create_anchor("wikiPage".into(), "".into())?;
        let vec: Vec<Address> = Vec::new();
        let page_entry = Entry::App(
            "wikiPage".into(),
            WikiPage {
                page_name: page_anchor_address.clone(),
                titulo: titulo,
                redered_page_element: vec,
            }
            .into(),
        );
        let page_adress =
            hdk::utils::commit_and_link(&page_entry, &page_anchor_address, "anchor->wikiPage", "")?;
        Ok(page_adress)
    }
    #[zome_fn("hc_public")]
    fn create_page_with_elements(
        contents: Vec<PageElement>,
        titulo: String,
    ) -> ZomeApiResult<Address> {
        let page_anchor_address = holochain_anchors::create_anchor("wikiPage".into(), "".into())?;
        let elements_anchor_address =
            holochain_anchors::create_anchor("Elements".into(), "".into())?;

        let vector: Vec<Address> = contents
            .into_iter()
            .map(|mut element| {
                element.parent_page_anchor = Some(elements_anchor_address.clone());
                Entry::App("pageElement".into(), element.into())
            })
            .map(|elemente_entry| {
                hdk::utils::commit_and_link(
                    &elemente_entry,
                    &elements_anchor_address,
                    "anchor->pageElement",
                    "",
                )
            })
            .filter_map(Result::ok)
            .collect();
        let page_entry = Entry::App(
            "wikiPage".into(),
            WikiPage {
                page_name: page_anchor_address.clone(),
                titulo: titulo,
                redered_page_element: vector,
            }
            .into(),
        );
        let page_adress =
            hdk::utils::commit_and_link(&page_entry, &page_anchor_address, "anchor->wikiPage", "")?;
        Ok(page_adress)
    }
    #[zome_fn("hc_public")]
    fn get_page(address: Address) -> ZomeApiResult<WikiPage> {
        hdk::utils::get_as_type(address)
    }
    #[zome_fn("hc_public")]
    fn get_pages() -> ZomeApiResult<Vec<WikiPage>> {
        hdk::utils::get_links_and_load_type(
            &holochain_anchors::create_anchor("wikiPage".into(), "".into())?,
            LinkMatch::Exactly("anchor->wikiPage".into()),
            LinkMatch::Any,
        )
    }
    #[zome_fn("hc_public")]
    fn get_pages_address() -> ZomeApiResult<Vec<Address>> {
        Ok(hdk::api::get_links(
            &holochain_anchors::create_anchor("wikiPage".into(), "".into())?,
            LinkMatch::Exactly("anchor->wikiPage".into()),
            LinkMatch::Any,
        )?
        .addresses())
    }
    #[zome_fn("hc_public")]
    fn delete_page(address: Address) -> ZomeApiResult<Address> {
        if let Ok(t) = hdk::utils::get_as_type::<WikiPage>(address.clone()) {
            let _hola = t
                .clone()
                .redered_page_element
                .into_iter()
                .map(|element: Address| hdk::api::remove_entry(&element));
        };
        hdk::api::remove_entry(&address)
    }
    #[zome_fn("hc_public")]
    fn delete_element(page_adress: Address, element_address: Address) -> ZomeApiResult<Address> {
        if let Ok(t) = hdk::utils::get_as_type::<WikiPage>(page_adress.clone()) {
            let mut t = t;
            let nuevo_vector =
                inner_delete_element(t.clone().redered_page_element, element_address.clone());
            t.redered_page_element = nuevo_vector;
            hdk::api::update_entry(Entry::App("wikiPage".into(), t.into()), &page_adress)?;
        }
        Ok(element_address)
    }
    fn inner_delete_element(mut vector: Vec<Address>, address: Address) -> Vec<Address> {
        match vector.pop() {
            Some(t) => {
                if t == address {
                    let _hola = hdk::api::remove_entry(&address);
                } else {
                    vector = inner_delete_element(vector, address);
                    vector.push(t);
                }
            }
            _ => return vector,
        }
        return vector;
    }
    #[zome_fn("hc_public")]
    fn update_element(address: Address, element: PageElement) -> ZomeApiResult<Address> {
        let element_entry = Entry::App("pageElement".into(), element.into());
        hdk::api::update_entry(element_entry, &address)
    }
    #[zome_fn("hc_public")]
    fn get_all(address: Address) -> ZomeApiResult<Hola> {
        let page = hdk::utils::get_as_type(address.clone())?;
        let vector = hdk::utils::get_as_type::<WikiPage>(address.clone())?.redered_page_element;

        Ok(Hola {
            page_name: address.clone(),
            page: page,
            vector_address: vector,
            redered_page_element: inner_get_elements_page(address.clone())?,
        })
    }
    #[zome_fn("hc_public")]
    fn update_tittle(tittle: String, address: Address) -> ZomeApiResult<Address> {
        match hdk::utils::get_as_type::<WikiPage>(address.clone()) {
            Ok(t) => {
                hdk::api::update_entry(
                    Entry::App(
                        "wikiPage".into(),
                        WikiPage {
                            page_name: t.page_name,
                            titulo: tittle,
                            redered_page_element: t.redered_page_element,
                        }
                        .into(),
                    ),
                    &address,
                )?;
                Ok(address)
            }
            Err(r) => Err(r),
        }
    }
    #[zome_fn("hc_public")]
    fn get_inner_get_elements_page(address: Address) -> ZomeApiResult<Vec<PageElement>> {
        inner_get_elements_page(address)
    }
    fn inner_get_elements_page(address: Address) -> ZomeApiResult<Vec<PageElement>> {
        let vector = hdk::utils::get_as_type::<WikiPage>(address)?.redered_page_element;
        vector
            .into_iter()
            .map(|address: Address| hdk::utils::get_as_type::<PageElement>(address))
            .collect()
    }
    #[zome_fn("hc_public")]
    fn get_element(address: Address) -> ZomeApiResult<PageElement> {
        hdk::utils::get_as_type(address)
    }

    #[zome_fn("hc_public")]
    fn add_page_element(element: PageElement, page_adress: Address) -> ZomeApiResult<Address> {
        let elements_anchor_address =
            holochain_anchors::create_anchor("Elements".into(), "".into())?;
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
                let mut vector = t.redered_page_element;
                vector.push(address.clone());
                hdk::api::update_entry(
                    Entry::App(
                        "wikiPage".into(),
                        WikiPage {
                            page_name: t.page_name,
                            titulo: t.titulo,
                            redered_page_element: vector,
                        }
                        .into(),
                    ),
                    &page_adress,
                )?;
                Ok(address)
            }
            Err(r) => Err(r),
        }
    }
    fn ordenar(
        mut vector: Vec<Address>,
        address: Address,
        before_element_address: Address,
    ) -> Vec<Address> {
        match vector.pop() {
            Some(t) => {
                if t == before_element_address {
                    vector.push(address);
                    vector.push(t);
                } else {
                    vector = ordenar(vector, address, before_element_address);
                    vector.push(t);
                }
            }
            _ => return vector,
        }
        return vector;
    }
    #[zome_fn("hc_public")]
    fn add_page_element_ordered(
        element: PageElement,
        before_element_address: Address,
        page_adress: Address,
    ) -> ZomeApiResult<Address> {
        let elements_anchor_address =
            holochain_anchors::create_anchor("Elements".into(), "".into())?;
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
                let vector = t.redered_page_element;

                let vector = ordenar(vector, address.clone(), before_element_address);

                hdk::api::update_entry(
                    Entry::App(
                        "wikiPage".into(),
                        WikiPage {
                            page_name: t.page_name,
                            titulo: t.titulo,
                            redered_page_element: vector,
                        }
                        .into(),
                    ),
                    &page_adress,
                )?;
                Ok(address)
            }
            Err(r) => Err(r),
        }
    }
}
