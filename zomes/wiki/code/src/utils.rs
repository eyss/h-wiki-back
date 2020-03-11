use hdk::prelude::*;

pub fn get_entry(address: Address) -> ZomeApiResult<JsonString> {
    match hdk::get_entry(&address) {
        Ok(Some(Entry::App(_, json_string))) => Ok(json_string),
        _ => Err(ZomeApiError::Internal("No hay entrada".into())),
    }
}

pub fn validate_agent_can_edit(validation_data: hdk::ValidationData) -> Result<(), String> {
    let editor = holochain_roles::validation::validate_required_role(
        &validation_data,
        &String::from(crate::EDITOR_ROLE_NAME),
    );
    let admin = holochain_roles::validation::validate_required_role(
        &validation_data,
        &String::from(holochain_roles::ADMIN_ROLE_NAME),
    );

    match (editor, admin) {
        (Err(_), Err(_)) => Err(String::from("Only admins and editors edit content")),
        _ => Ok(()),
    }
}
