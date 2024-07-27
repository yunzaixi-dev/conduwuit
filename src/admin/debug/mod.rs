mod commands;
pub(crate) mod tester;

use clap::Subcommand;
use conduit::Result;
use ruma::{EventId, OwnedRoomOrAliasId, RoomId, ServerName};

use self::tester::TesterCommand;
use crate::admin_command_dispatch;

#[admin_command_dispatch]
#[derive(Debug, Subcommand)]
pub(super) enum DebugCommand {
	/// - Echo input of admin command
	Echo {
		message: Vec<String>,
	},

	/// - Get the auth_chain of a PDU
	GetAuthChain {
		/// An event ID (the $ character followed by the base64 reference hash)
		event_id: Box<EventId>,
	},

	/// - Parse and print a PDU from a JSON
	///
	/// The PDU event is only checked for validity and is not added to the
	/// database.
	///
	/// This command needs a JSON blob provided in a Markdown code block below
	/// the command.
	ParsePdu,

	/// - Retrieve and print a PDU by ID from the conduwuit database
	GetPdu {
		/// An event ID (a $ followed by the base64 reference hash)
		event_id: Box<EventId>,
	},

	/// - Attempts to retrieve a PDU from a remote server. Inserts it into our
	///   database/timeline if found and we do not have this PDU already
	///   (following normal event auth rules, handles it as an incoming PDU).
	GetRemotePdu {
		/// An event ID (a $ followed by the base64 reference hash)
		event_id: Box<EventId>,

		/// Argument for us to attempt to fetch the event from the
		/// specified remote server.
		server: Box<ServerName>,
	},

	/// - Same as `get-remote-pdu` but accepts a codeblock newline delimited
	///   list of PDUs and a single server to fetch from
	GetRemotePduList {
		/// Argument for us to attempt to fetch all the events from the
		/// specified remote server.
		server: Box<ServerName>,

		/// If set, ignores errors, else stops at the first error/failure.
		#[arg(short, long)]
		force: bool,
	},

	/// - Gets all the room state events for the specified room.
	///
	/// This is functionally equivalent to `GET
	/// /_matrix/client/v3/rooms/{roomid}/state`, except the admin command does
	/// *not* check if the sender user is allowed to see state events. This is
	/// done because it's implied that server admins here have database access
	/// and can see/get room info themselves anyways if they were malicious
	/// admins.
	///
	/// Of course the check is still done on the actual client API.
	GetRoomState {
		/// Room ID
		room_id: OwnedRoomOrAliasId,
	},

	/// - Get and display signing keys from local cache or remote server.
	GetSigningKeys {
		server_name: Option<Box<ServerName>>,

		#[arg(short, long)]
		cached: bool,
	},

	/// - Sends a federation request to the remote server's
	///   `/_matrix/federation/v1/version` endpoint and measures the latency it
	///   took for the server to respond
	Ping {
		server: Box<ServerName>,
	},

	/// - Forces device lists for all local and remote users to be updated (as
	///   having new keys available)
	ForceDeviceListUpdates,

	/// - Change tracing log level/filter on the fly
	///
	/// This accepts the same format as the `log` config option.
	ChangeLogLevel {
		/// Log level/filter
		filter: Option<String>,

		/// Resets the log level/filter to the one in your config
		#[arg(short, long)]
		reset: bool,
	},

	/// - Verify json signatures
	///
	/// This command needs a JSON blob provided in a Markdown code block below
	/// the command.
	SignJson,

	/// - Verify json signatures
	///
	/// This command needs a JSON blob provided in a Markdown code block below
	/// the command.
	VerifyJson,

	/// - Prints the very first PDU in the specified room (typically
	///   m.room.create)
	FirstPduInRoom {
		/// The room ID
		room_id: Box<RoomId>,
	},

	/// - Prints the latest ("last") PDU in the specified room (typically a
	///   message)
	LatestPduInRoom {
		/// The room ID
		room_id: Box<RoomId>,
	},

	/// - Forcefully replaces the room state of our local copy of the specified
	///   room, with the copy (auth chain and room state events) the specified
	///   remote server says.
	///
	/// A common desire for room deletion is to simply "reset" our copy of the
	/// room. While this admin command is not a replacement for that, if you
	/// know you have split/broken room state and you know another server in the
	/// room that has the best/working room state, this command can let you use
	/// their room state. Such example is your server saying users are in a
	/// room, but other servers are saying they're not in the room in question.
	///
	/// This command will get the latest PDU in the room we know about, and
	/// request the room state at that point in time via
	/// `/_matrix/federation/v1/state/{roomId}`.
	ForceSetRoomStateFromServer {
		/// The impacted room ID
		room_id: Box<RoomId>,
		/// The server we will use to query the room state for
		server_name: Box<ServerName>,
	},

	/// - Runs a server name through conduwuit's true destination resolution
	///   process
	///
	/// Useful for debugging well-known issues
	ResolveTrueDestination {
		server_name: Box<ServerName>,

		#[arg(short, long)]
		no_cache: bool,
	},

	/// - Print extended memory usage
	MemoryStats,

	/// - Print general tokio runtime metric totals.
	RuntimeMetrics,

	/// - Print detailed tokio runtime metrics accumulated since last command
	///   invocation.
	RuntimeInterval,

	/// - Print the current time
	Time,

	/// - List dependencies
	ListDependencies {
		#[arg(short, long)]
		names: bool,
	},

	/// - Developer test stubs
	#[command(subcommand)]
	#[allow(non_snake_case)]
	Tester(TesterCommand),
}
