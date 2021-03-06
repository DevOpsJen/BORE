use anyhow::Result;
use bore_cli::auth::Authenticator;
use tokio::io::{self, BufReader};

#[tokio::test]
async fn auth_handshake() -> Result<()> {
    let auth = Authenticator::new("some secret string");

    let (client, server) = io::duplex(8); // Ensure correctness with limited capacity.
    let mut client = BufReader::new(client);
    let mut server = BufReader::new(server);

    tokio::try_join!(
        auth.client_handshake(&mut client),
        auth.server_handshake(&mut server),
    )?;

    Ok(())
}

#[tokio::test]
async fn auth_handshake_fail() {
    let auth = Authenticator::new("client secret");
    let auth2 = Authenticator::new("different server secret");

    let (client, server) = io::duplex(8); // Ensure correctness with limited capacity.
    let mut client = BufReader::new(client);
    let mut server = BufReader::new(server);

    let result = tokio::try_join!(
        auth.client_handshake(&mut client),
        auth2.server_handshake(&mut server),
    );
    assert!(result.is_err());
}
