#![feature(proc_macro_hygiene)]
#![feature(vec_remove_item)]

extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::prelude::*;
use hdk_proc_macros::zome;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct File {
    name: String,
    size: u64,
    r#type: String,
    last_modified: u64,
    chunks: Vec<Address>,
}

// #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
// pub struct File2 {
//     file: Vec<u8>,
//     name: String,
// }

impl File {
    pub fn entry(self) -> Entry {
        Entry::App("file".into(), self.into())
    }
}

#[zome]
mod file_storage {
    #[init]
    fn init() {
        Ok(())
    }
    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }
    #[entry_def]
    pub fn file_def() -> ValidatingEntryType {
        entry!(
            name: "file",
            description: "this is an entry representing some profile info for an agent",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<File>| {
               Ok(())
            }
        )
    }
    #[entry_def]
    pub fn chunk_def() -> ValidatingEntryType {
        entry!(
            name: "chunk",
            description: "this is an entry representing some profile info for an agent",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<String>| {
               Ok(())
            }
        )
    }
    /*
    #[zome_fn("hc_public")]
    fn create_file(file: Vec<u8>, name: String) -> ZomeApiResult<Address> {
        hdk::commit_entry(
            &File::from(
                file.chunks(32000)
                    .map(|chunk| {
                        let entry = Chunk::from(chunk.to_vec()).entry();
                        create_if_exist(entry.clone(), entry.address())
                    })
                    .filter_map(Result::ok)
                    .collect(),
                name,
            )
            .entry(),
        )
    }
    #[zome_fn("hc_public")]
    fn get_file(address: Address) -> ZomeApiResult<File2> {
        let file: File = hdk::utils::get_as_type(address)?;
        Ok(File2 {
            file: file
            .chunks
            .into_iter()
            .map(|chunk_address| hdk::utils::get_as_type::<Chunk>(chunk_address))
            .filter_map(Result::ok)
            .map(|chunk| chunk.chunk)
            .collect::<Vec<Vec<u8>>>()
            .concat(),
            name: file.name,
        })
    }
    */
    #[zome_fn("hc_public")]
    fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }
    #[zome_fn("hc_public")]
    fn create_chunk(chunk: String) -> ZomeApiResult<Address> {
        let entry = Entry::App("chunk".into(), JsonString::from_json(chunk.as_str()));
        create_if_exist(entry.clone(), entry.address())
    }
    #[zome_fn("hc_public")]
    fn create_file(file: File) -> ZomeApiResult<Address> {
        let entry = file.entry();
        hdk::commit_entry(&entry)
    }
}

fn create_if_exist(entry: Entry, address: Address) -> ZomeApiResult<Address> {
    if let Ok(Some(_)) = hdk::get_entry(&address) {
        Ok(address)
    } else {
        hdk::commit_entry(&entry)
    }
}
