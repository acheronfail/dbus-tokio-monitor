use std::time::Duration;

use dbus::{message::MatchRule, nonblock};

use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (resource, con) = dbus_tokio::connection::new_session_sync()?;
    tokio::spawn(async move {
        use std::error::Error;
        let err = resource.await;
        dbg!(err.source());
        panic!("Lost connecton to dbus: {}", err);
    });

    let rule = MatchRule::new()
        .with_type(dbus::MessageType::MethodCall)
        .with_path("/org/freedesktop/Notifications")
        .with_interface("org.freedesktop.DBus.Properties")
        .with_member("Set");

    // tell dbus we want to become a monitor
    {
        let dbus_proxy = nonblock::Proxy::new(
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            Duration::from_secs(5),
            con.clone(),
        );

        let _: () = dbus_proxy
            .method_call(
                "org.freedesktop.DBus.Monitoring",
                "BecomeMonitor",
                (vec![rule.match_str()], 0u32),
            )
            .await?;
    }

    let (_match, match_rx) = con.add_match(rule).await?.msg_stream();
    let stream = match_rx.for_each(|m| {
        dbg!(m);
        async {}
    });

    stream.await;

    Ok(())
}
