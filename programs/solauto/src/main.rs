// use std::str::FromStr;

// use borsh::BorshDeserialize;
// use marginfi_sdk::generated::accounts::{Bank, MarginfiAccount};
// use solana_client::rpc_client::RpcClient;
// use solana_sdk::pubkey::Pubkey;
// use fixed::types::I80F48;

// fn main() {
//     let rpc_url = String::from("https://api.mainnet-beta.solana.com/");
//     let client = RpcClient::new(rpc_url);

//     // let pubkey = Pubkey::from_str("CCKtUs6Cgwo4aaQUmBPmyoApH2gUDErxNZCAntD6LYGh").unwrap(); // SOL
//     let pubkey = Pubkey::from_str("2s37akK2eyBbp8DZgCm7RtsaEz8eJP3Nxd4urLHQv7yB").unwrap(); // USDC
//     // let pubkey = Pubkey::from_str("Guu5uBc8k1WK1U2ihGosNaCy57LSgCkpWAabtzQqrQf8").unwrap(); // JUP
//     match client.get_account(&pubkey) {
//         Ok(account_info) => {
//             let bank = Bank::deserialize(&mut account_info.data.as_slice()).unwrap();
//             println!("{:?}", bank);
//             println!("total asset shares {}", I80F48::from_le_bytes(bank.total_asset_shares.value));
//             println!("asset share value {}", I80F48::from_le_bytes(bank.asset_share_value.value));
//             println!("collected insurance fees outstanding {}", I80F48::from_le_bytes(bank.collected_insurance_fees_outstanding.value));
//             println!("collected group fees outstanding {}", I80F48::from_le_bytes(bank.collected_group_fees_outstanding.value));
//             println!("emissions remaining {}", I80F48::from_le_bytes(bank.emissions_remaining.value));
//             println!("asset weight init {}", I80F48::from_le_bytes(bank.config.asset_weight_init.value));
//             println!("asset weight maint {}", I80F48::from_le_bytes(bank.config.asset_weight_maint.value));
//             println!("liability weight init {}", I80F48::from_le_bytes(bank.config.liability_weight_init.value));
//             println!("liability weight maint {}", I80F48::from_le_bytes(bank.config.liability_weight_maint.value));
//         }
//         Err(e) => println!("An error occurred: {}", e),
//     }

//     let pubkey = Pubkey::from_str("3BExFoAiVG7k7QtNvZU1zh7zkSQf2K6P8QwYmRnXFe8F").unwrap();
//     match client.get_account(&pubkey) {
//         Ok(account_info) => {
//             let marginfi_account = MarginfiAccount::deserialize(&mut account_info.data.as_slice()).unwrap();
//             println!("{:?}", marginfi_account);
//             println!("account asset shares {}", I80F48::from_le_bytes(marginfi_account.lending_account.balances[0].asset_shares.value));
//         }
//         Err(e) => println!("An error occurred: {}", e),
//     }

// }

// use std::ops::{ Div, Mul, Sub };
// use solauto::utils::math_utils;

fn main() {
    //     let mut supply = 447805.0;
    //     let mut debt = 201545.0;
    //     let supply_weight = 0.899999976158142;
    //     let debt_weight = 1.100000023841858;
    //     let liq_threshold = supply_weight.div(debt_weight);

    //     let debt_adjustment_usd = math_utils::get_std_debt_adjustment_usd(
    //         liq_threshold,
    //         supply,
    //         debt,
    //         5500,
    //         0
    //     );

    //     supply += debt_adjustment_usd;
    //     debt += debt_adjustment_usd;

    //     println!(
    //         "{}",
    //         1.0 - (supply.mul(supply_weight).sub(debt.mul(debt_weight)).div(supply.mul(supply_weight)))
    //     );
    //     println!(
    //         "{}",
    //         debt.div(supply.mul(liq_threshold))
    //     );
    //     println!("${}", debt_adjustment_usd);
}