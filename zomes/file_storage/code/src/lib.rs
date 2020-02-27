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
pub struct Chunck {
    chunck: Vec<u8>,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct File {
    chuncks: Vec<Address>,
    name: String,
    agent_address: Address,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct File2 {
    file: Vec<u8>,
    name: String,
}
impl Chunck {
    pub fn from(chunck: Vec<u8>) -> Chunck {
        Chunck { chunck }
    }
    pub fn entry(self) -> Entry {
        Entry::App("chunck".into(), self.into())
    }
}
impl File {
    pub fn from(chuncks: Vec<Address>, name: String) -> File {
        File {
            chuncks,
            name,
            agent_address: hdk::api::AGENT_ADDRESS.clone(),
        }
    }
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
            name: "chunck",
            description: "this is an entry representing some profile info for an agent",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<Chunck>| {
               Ok(())
            }
        )
    }
    fn create_if_exist(entry: Entry, address: Address) -> ZomeApiResult<Address> {
        if let Ok(Some(_)) = hdk::get_entry(&address) {
            Ok(address)
        } else {
            hdk::commit_entry(&entry)
        }
    }
    #[zome_fn("hc_public")]
    fn create_file(file: Vec<u8>, name: String) -> ZomeApiResult<Address> {
        hdk::commit_entry(
            &File::from(
                file.chunks(32000)
                    .map(|chunck| {
                        let entry = Chunck::from(chunck.to_vec()).entry();
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
                .chuncks
                .into_iter()
                .map(|chunck_address| hdk::utils::get_as_type::<Chunck>(chunck_address))
                .filter_map(Result::ok)
                .map(|chunck| chunck.chunck)
                .collect::<Vec<Vec<u8>>>()
                .concat(),
            name: file.name,
        })
    }
    #[zome_fn("hc_public")]
    fn create_chunck(chunck: Vec<u8>) -> ZomeApiResult<Address> {
        let entry = Chunck::from(chunck).entry();
        create_if_exist(entry.clone(), entry.address())
    }
    #[zome_fn("hc_public")]
    fn get_chunck(address: Address) -> ZomeApiResult<Chunck> {
        hdk::utils::get_as_type(address)
    }
}
//[network]
//sim2h_url = 'ws://public.sim2h.net:9000'
//type = 'sim2h'
