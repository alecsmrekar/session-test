use goose::prelude::*;
use goose_eggs::validate_page;

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("Anon User").register_transaction(transaction!(hp_anon).set_name("anon hp")),
        )
        .register_scenario(
            scenario!("Auth User")
                .register_transaction(transaction!(login).set_name("login").set_on_start())
                .register_transaction(transaction!(hp_auth).set_name("auth hp")),
        )
        .execute()
        .await?;
    Ok(())
}

async fn hp_anon(user: &mut GooseUser) -> TransactionResult {
    let goose = user.get("").await?;
    let validation = goose_eggs::Validate::builder().not_text("Log out").build();
    let _html = validate_page(user, goose, &validation).await?;
    Ok(())
}

async fn hp_auth(user: &mut GooseUser) -> TransactionResult {
    let goose = user.get("").await?;
    let validation = goose_eggs::Validate::builder().text("Log out").build();
    let _html = validate_page(user, goose, &validation).await?;
    Ok(())
}

async fn login(user: &mut GooseUser) -> TransactionResult {
    let login = goose_eggs::drupal::Login::builder()
        .username("admin")
        .password("admin")
        .url("user/login")
        .build();
    goose_eggs::drupal::log_in(user, &login).await?;
    Ok(())
}
