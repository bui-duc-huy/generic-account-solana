use anchor_lang::prelude::*;
use arrayref::*;

pub trait TypeTrait: AnchorSerialize + AnchorDeserialize {
    fn name(&self) -> String;
}

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct DataType1 {
    pub name: String
}

impl TypeTrait for DataType1 {
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct DataType2 {
    pub name: String
}

impl TypeTrait for DataType2 {
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[account]
pub struct DataInfo<T: TypeTrait> {
    pub type_data: u8,
    pub data_core: T
}

impl <T: TypeTrait> DataInfo<T> {
    pub fn print_name(&self) {
        let name: String = self.data_core.name();

        msg!("Type name is {}", name);
    }
}

pub fn parse_data_core<'info>(info: &AccountInfo<'info>) -> TypeDefine {
    let data: &[u8] = &info.try_borrow_data().unwrap();
    let src = array_ref![data, 0, 9];

    let (_account_discriminator, type_data) = array_refs![src, 8, 1];

    match *type_data {
        [0] => TypeDefine::DataType1,
        [1] => TypeDefine::DataType2,
        _ => TypeDefine::DataType2,
    }
}

pub enum TypeDefine {
    DataType1,
    DataType2,
}
