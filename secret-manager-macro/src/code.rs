use aws_sdk_secretsmanager::Client;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use syn::parse::{Parse, ParseStream};
use syn::LitStr;
use syn::{parse2, Error};

#[derive(Debug)]
struct Input {
    secret_name: LitStr,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Input {
            secret_name: input.parse().unwrap(),
        })
    }
}

// TODO we will accept exact matches but also Camelcase/snake/etc.
// TODO span, better message
fn check_existence(secret_name: &str) -> Result<String, Error> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let results = rt.block_on(list_secrets());
    search_secret(results, secret_name)
}

// TODO test
fn search_secret(possible_secrets: Vec<String>, secret_name: &str) -> Result<String, Error> {
    possible_secrets
        .iter()
        .find(|v| v.eq(&secret_name))
        .map(|v| v.to_string())
        .ok_or_else(|| {
            Error::new(
                Span::call_site(),
                format!(
                    "Did not find secret in linked AWS account for name {}",
                    secret_name
                ),
            )
        })
}

// TODO handle next token; avoid allocation (also elsewhere)
async fn list_secrets() -> Vec<String> {
    let shared_config = aws_config::from_env().load().await;
    let client = Client::new(&shared_config);
    let result = client.list_secrets().send().await.unwrap();
    result
        .secret_list()
        .unwrap()
        .iter()
        .filter_map(|v| v.name())
        .map(|v| v.to_string())
        .collect()
}

// TODO temp
//  also: use secret string? (but extra dependency) - or add a simple wrapper ourselves
#[derive(Debug, Deserialize)]
struct Secrets {
    key1: String,
    key2: String,
}

fn temp(secret_name: &str) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let results = rt.block_on(get_secret(secret_name));
    let secrets: Secrets = serde_json::from_str(&results.unwrap()).unwrap();
    println!("{:?}", secrets);
}

// TODO will happen inside client... so quoting stuff
async fn get_secret(secret_name: &str) -> Option<String> {
    let shared_config = aws_config::from_env().load().await;
    let client = Client::new(&shared_config);
    let result = client
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await;
    // let t = result.unwrap();
    // t.secret_string().take().unwrap()
    let temp = result.unwrap();
    temp.secret_string().map(|v| v.to_string())
}

// TODO error handling
//    ServiceError(ServiceError { source: ListSecretsError { kind: Unhandled(Unhandled { source: Error { code: Some("ExpiredTokenException"), message: Some("The security token included in the request is expired"), request_id: Some("ddb0119d-62ac-4e3e-960f-1c3730758300"), extras: {} } }), meta: Error { code: Some("ExpiredTokenException"), message: Some("The security token included in the request is expired"), request_id: Some("ddb0119d-62ac-4e3e-960f-1c3730758300"), extras: {} } }, raw: Response { inner: Response { status: 400, version: HTTP/1.1, headers: {"x-amzn-requestid": "ddb0119d-62ac-4e3e-960f-1c3730758300", "content-type": "application/x-amz-json-1.1", "content-length": "100", "date": "Wed, 22 Mar 2023 16:48:15 GMT", "connection": "close"}, body: SdkBody { inner: Once(Some(b"{\"__type\":\"ExpiredTokenException\",\"message\":\"The security token included in the request is expired\"}")), retryable: true } }, properties: SharedPropertyBag(Mutex { data: PropertyBag, poisoned: false, .. }) } })
pub fn create_secret_manager(item: TokenStream) -> TokenStream {
    eprintln!("{:?}", item);
    let input: Input = parse2(item).unwrap();

    // 'real' secret name
    let secret_name = check_existence(&input.secret_name.value()).unwrap();
    temp(&secret_name);

    // eprintln!("{}", input.secret_name.to_string());

    quote! {}
}
