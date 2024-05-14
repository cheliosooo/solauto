pub mod test_utils;

#[cfg(test)]
mod update_position {
    use chrono::Utc;
    use solana_program_test::tokio;
    use solana_sdk::signer::Signer;
    use solauto_sdk::generated::{
        accounts::SolautoPosition,
        types::{ DCADirection, DCASettings, SolautoSettingsParameters },
    };

    use crate::{ assert_instruction_error, test_utils::* };

    #[tokio::test]
    async fn update_position_settings() {
        let args = GeneralArgs::new();
        let mut data = MarginfiTestData::new(&args).await;
        data.test_prefixtures().await
            .unwrap()
            .general.create_referral_state_accounts().await
            .unwrap();
        data.general
            .create_ata(
                data.general.ctx.payer.pubkey(),
                data.general.debt_liquidity_mint.unwrap()
            ).await
            .unwrap();

        let dca_amount = 50_000;
        data.general
            .mint_tokens_to_ta(
                data.general.debt_liquidity_mint.unwrap(),
                data.general.signer_debt_liquidity_ta.unwrap(),
                data.general.ctx.payer.pubkey(),
                dca_amount
            ).await
            .unwrap();

        data.open_position(
            Some(data.general.default_setting_params.clone()),
            None
        ).await.unwrap();

        let new_settings = SolautoSettingsParameters {
            boost_to_bps: 2000,
            boost_gap: 1000,
            repay_to_bps: 8500,
            repay_gap: 1000,
        };
        data.update_position(Some(new_settings.clone()), None).await.unwrap();

        let solauto_position = data.general.deserialize_account_data::<SolautoPosition>(
            data.general.solauto_position
        ).await;
        assert!(
            solauto_position.position.as_ref().unwrap().setting_params.as_ref().unwrap() ==
                &new_settings
        );
    }

    // pub async fn test_settings(data: &mut MarginfiTestData<'_>, settings: SolautoSettingsParameters) {
    //     let tx = Transaction::new_signed_with_payer(
    //         &[data.open_position_ix(Some(settings), None).instruction()],
    //         Some(&data.general.ctx.payer.pubkey()),
    //         &[&data.general.ctx.payer],
    //         data.general.ctx.last_blockhash
    //     );
    //     let err = data.general.ctx.banks_client.process_transaction(tx).await.unwrap_err();
    //     assert_instruction_error!(err, InstructionError::Custom(4));
    // }

    // #[tokio::test]
    // async fn invalid_settings() {
    //     let args = GeneralArgs::new();
    //     let mut data = MarginfiTestData::new(&args).await;
    //     data
    //         .test_prefixtures().await
    //         .unwrap()
    //         .create_referral_state_accounts().await
    //         .unwrap();

    //     test_settings(&mut data, SolautoSettingsParameters {
    //         boost_to_bps: 4499,
    //         boost_from_bps: 4500,
    //         repay_to_bps: 9000,
    //         repay_from_bps: 9500,
    //     }).await;
    // }
}