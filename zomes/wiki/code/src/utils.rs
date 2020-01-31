extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use hdk::error::{ZomeApiError, ZomeApiResult};
use hdk::holochain_json_api::json::JsonString;
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::Entry::App;
use holochain_anchors;
pub fn anchor(anchor_type: &'static str, anchor_text: &'static str) -> ZomeApiResult<Address> {
    holochain_anchors::create_anchor(anchor_type.into(), anchor_text.into())
}
pub fn get_entry(address: Address) -> ZomeApiResult<JsonString> {
    match hdk::get_entry(&address) {
        Ok(Some(App(_, json_string))) => Ok(json_string),
        _ => Err(ZomeApiError::Internal("No hay entrada".into())),
    }
}
