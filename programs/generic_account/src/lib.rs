use anchor_lang::prelude::*;

mod states;
use crate::states::*;

declare_id!("AyBqnrH1y4U9Cqrupj3pKFYoAtoXb5RxUfoDFPUuKDrs");

#[program]
mod generic_account {
    use super::*;

    pub fn create_type_1(ctx: Context<CreateType1>) -> Result<()> {
        let data_info = &mut ctx.accounts.data_info;

        let type_1 = DataType1 {
            name: "type 1".to_string()
        };

        data_info.data_core = type_1;
        data_info.type_data = 0;

        Ok(())
    }

    pub fn create_type_2(ctx: Context<CreateType2>) -> Result<()> {
        let data_info = &mut ctx.accounts.data_info;

        let type_2 = DataType2 {
            name: "type 2".to_string()
        };

        data_info.data_core = type_2;
        data_info.type_data = 1;

        Ok(())
    }

    pub fn print_type(ctx: Context<PrintType>) -> Result<()> {
        let data_info = &ctx.accounts.data_info;

        match parse_data_core(data_info) {
            TypeDefine::DataType1 => {
                let data = Account::<DataInfo<DataType1>>::try_from(data_info).unwrap();

                data.print_name();
            },
            TypeDefine::DataType2 => {
                let data = Account::<DataInfo<DataType2>>::try_from(data_info).unwrap();

                data.print_name();
            }
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateType1<'info> {
    #[account(mut)]
    pub root: Signer<'info>,

    #[account(
        init,
        payer = root,
        space =  100
    )]
    pub data_info: Account<'info, DataInfo<DataType1>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateType2<'info> {
    #[account(mut)]
    pub root: Signer<'info>,

    #[account(
        init,
        payer = root,
        space =  100
    )]
    pub data_info: Account<'info, DataInfo<DataType2>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PrintType<'info> {
    #[account(mut)]
    pub root: Signer<'info>,

    /// CHECK: skip
    pub data_info: AccountInfo<'info>,
}
