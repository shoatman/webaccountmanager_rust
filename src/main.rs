
use windows::{
    core::*,
    Security::Authentication::Web::Core::*, 
};

async fn main_async() -> windows::core::Result<()> {

    let provider = w!("https://login.microsoft.com");
    let authority = w!("https://login.microsoftonline.com/microsoft.com");

    let account_provider = WebAuthenticationCoreManager::FindAccountProviderWithAuthorityAsync(provider, authority)?.await?;

    println!("{}", account_provider.IsSystemProvider()?);
    println!("{}", account_provider.Authority()?);

    let client_id = w!("4e54273c-9fc5-42f4-81b6-60d1b66c9160");

    let web_token_request = WebTokenRequest::Create(&account_provider, w!(""), &client_id)?;
    //let properties = web_token_request.Properties()?;
    //let _ = properties.Insert(w!("resource"), w!("https://graph.windows.net"));

    let web_token_result = WebAuthenticationCoreManager::RequestTokenAsync(&web_token_request)?.await?;

    //let web_token_result = WebAuthenticationCoreManager::GetTokenSilentlyAsync(&web_token_request)?.await?;

    match web_token_result.ResponseStatus()? {
        WebTokenRequestStatus::Success => {println!("{}", "Success".to_string())},
        WebTokenRequestStatus::UserCancel => {println!("{}", "UserCancel".to_string())},
        WebTokenRequestStatus::UserInteractionRequired => {println!("{}", "UserInteractionRequired".to_string())},
        WebTokenRequestStatus::AccountSwitch => {println!("{}", "AccountSwitch".to_string())},
        WebTokenRequestStatus::AccountProviderNotAvailable => {println!("{}", "AccountProviderNotAvailable".to_string())},
        WebTokenRequestStatus::ProviderError => {
            println!("{}", "Provider Error".to_string());
            println!("{:?}", web_token_result.ResponseError()?.ErrorMessage()?);
        },
        _ => {},
    }

    Ok(())
    
}

fn main() -> Result<()> {
    futures::executor::block_on(main_async())
}
