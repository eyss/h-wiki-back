use hdk::prelude::*;
use holochain_anchors;

pub fn anchor(anchor_type: &'static str, anchor_text: &'static str) -> ZomeApiResult<Address> {
    holochain_anchors::create_anchor(anchor_type.into(), anchor_text.into())
}

pub fn get_entry(address: Address) -> ZomeApiResult<JsonString> {
    match hdk::get_entry(&address) {
        Ok(Some(Entry::App(_, json_string))) => Ok(json_string),
        _ => Err(ZomeApiError::Internal("No hay entrada".into())),
    }
}

// pub fn validate_agent_can_edit(validation_data: hdk::ValidationData) -> Result<(), String> {
//     let agent_address = validation_data.sources()[0].clone();
//     let timestamp = validation_data.package.chain_header.timestamp();

//     let is_admin = holochain_roles::validation::had_agent_role(
//         &agent_address,
//         &String::from(holochain_roles::ADMIN_ROLE_NAME),
//         timestamp,
//     )?;
//     let is_editor = holochain_roles::validation::had_agent_role(
//         &agent_address,
//         &String::from(crate::EDITOR_ROLE_NAME),
//         timestamp,
//     )?;

//     match (is_admin, is_editor) {
//         (false, false) => Err(String::from("Only admins or editors can create pages")),
//         _ => Ok(()),
//     }
//}
