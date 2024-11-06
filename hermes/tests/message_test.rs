use std::{net::SocketAddr, str::FromStr, time::Duration};

use hermes::{
    db::Db,
    grpc::{
        msg_service_client::MsgServiceClient, msg_service_server::MsgServiceServer, RecieveRequest,
        SendMsgReq, SendMsgRes,
    },
    services::MessageServer,
    Config,
};
use tokio::time::timeout;
use tokio_stream::StreamExt;
use tonic::transport::Server;

#[tokio::test()]
pub async fn test_message_stream() {
    let conf = Config::default();

    let db = Db::connect(conf.db_url.as_str()).await.unwrap();

    let soc = SocketAddr::from_str("[::1]:50051").expect("weird socket addr");

    let test_duration = Duration::from_secs(2);

    let server = tokio::spawn(async move {
        // we dont care about this time out
        let _ = timeout(
            test_duration,
            Server::builder()
                .add_service(MsgServiceServer::new(MessageServer::new(db)))
                .serve(soc),
        )
        .await;
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let client = tokio::spawn(async move {
        let soc = "http://[::]:50051";
        let mut client = MsgServiceClient::connect(soc).await.unwrap();

        let message_count = 1_000;

        let mut msg_stream = client
            .recieve_incoming(RecieveRequest {
                author: "doesnt matter".to_string(),
            })
            .await
            .unwrap()
            .into_inner();

        for _ in 0..message_count {
            let _msg = client
                .send(SendMsgReq {
                    convo: None,
                    author: None,
                    body: "doesnt matter".to_string(),
                    unix_time: 123456789,
                })
                .await
                .unwrap();
        }

        tokio::spawn(async move {
            let mut count = 0;
            while let Some(y) = timeout(test_duration, msg_stream.next())
                .await
                .ok()
                .flatten()
            {
                count += 1;
                eprintln!("{:#?}", y.unwrap());
            }
            assert_eq!(count, message_count); // making sure we didnt lose any messages
        })
        .await
        .unwrap();
    });

    let (server, client) = tokio::join!(server, client);

    server.expect("Server Panic");
    client.expect("Client Panic");
}
