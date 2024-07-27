use clap::Subcommand;
use conduit::Result;
use ruma::{events::room::message::RoomMessageEventContent, UserId};

use crate::Command;

#[derive(Debug, Subcommand)]
/// All the getters and iterators from src/database/key_value/presence.rs
pub(crate) enum PresenceCommand {
	/// - Returns the latest presence event for the given user.
	GetPresence {
		/// Full user ID
		user_id: Box<UserId>,
	},

	/// - Iterator of the most recent presence updates that happened after the
	///   event with id `since`.
	PresenceSince {
		/// UNIX timestamp since (u64)
		since: u64,
	},
}

/// All the getters and iterators in key_value/presence.rs
pub(super) async fn process(subcommand: PresenceCommand, context: &Command<'_>) -> Result<RoomMessageEventContent> {
	let services = context.services;

	match subcommand {
		PresenceCommand::GetPresence {
			user_id,
		} => {
			let timer = tokio::time::Instant::now();
			let results = services.presence.db.get_presence(&user_id)?;
			let query_time = timer.elapsed();

			Ok(RoomMessageEventContent::notice_markdown(format!(
				"Query completed in {query_time:?}:\n\n```rs\n{results:#?}\n```"
			)))
		},
		PresenceCommand::PresenceSince {
			since,
		} => {
			let timer = tokio::time::Instant::now();
			let results = services.presence.db.presence_since(since);
			let presence_since: Vec<(_, _, _)> = results.collect();
			let query_time = timer.elapsed();

			Ok(RoomMessageEventContent::notice_markdown(format!(
				"Query completed in {query_time:?}:\n\n```rs\n{presence_since:#?}\n```"
			)))
		},
	}
}
