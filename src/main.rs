use octocrab::Octocrab;
use notify_rust::{Notification, Hint, CloseReason};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

	let octocrab = Octocrab::builder().personal_token(token).build()?;

	let notifications = octocrab
		.activity()
		.notifications()
		.list()
		.send()
		.await?;

	let mut tasks = Vec::new();
	for notification in notifications {
		// TODO on_close which doesn't block so I don't need to spawn all these workers!!!
		tasks.push(tokio::spawn(async move {
			// TODO on_close which can be FnOnce so I can move stuff into it and avoid channels!!!
			let (tx, mut rx) = mpsc::unbounded_channel();
			Notification::new()
				.summary(&format!("{} (GitHub)", notification.repository.name))
				.body(&format!("[{}] {}", notification.subject.r#type, notification.subject.title))
				.appname("github")
				.icon("github")
				.action("clicked", "click here")
				.hint(Hint::Resident(true))
				.show_async().await.unwrap()
				.on_close(|reason| match reason {
					CloseReason::Dismissed => tx.send(true).unwrap(),
					_ => tx.send(false).unwrap(),
				});
			if rx.recv().await.unwrap() {
				if let Some(url) = notification.subject.url {
					// TODO this is awful! Is there no proper way to get html_url using octocrab?????
					let client = reqwest::Client::builder()
						.user_agent("holy-shit-fuck-you-github-why-are-you-401-me-if-i-dont-give-an-user-agent")
						.build().unwrap();
					let response_raw = client.get(url)
						.send()
						.await.unwrap()
						.text().await.unwrap();
					let response : serde_json::Value = serde_json::from_str(&response_raw).unwrap();
					let html_url = response
						.get("html_url").unwrap();
					open::that(html_url.as_str().unwrap()).unwrap();
				} else if let Some(url) = notification.repository.html_url {
					open::that(url.to_string()).unwrap();
				} else {
					open::that(notification.url.to_string()).unwrap();
				};
			}
		}));
	}

	for t in tasks {
		t.await?;
	}

	Ok(())
}
