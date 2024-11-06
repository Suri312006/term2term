use clap::Parser;
use color_eyre::Result;
use t2t_cli::{
    grpc::{SearchUserReq, SearchUserReqEnum},
    AppState, Cli,
};
use tonic::{IntoRequest, Request};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    let mut app = AppState::instantiate().await?;
    cli.run(&mut app).await

    //let req = SearchUserReq {
    //    r#type: SearchUserReqEnum::NameAndSuffix,
    //    name: "suredra".to_string(),
    //    suffix: "123".to_string(),
    //}
    //.into_request().metadata_mut().insert("Bearer", "1234");
}
