use clap;
use clap::Shell;

#[derive(Debug, StructOpt)]
#[structopt(name = "subs", about = "Subsonic player",
            setting_raw = "clap::AppSettings::VersionlessSubcommands")]
pub struct App {
    /// Sets the verbosity
    #[structopt(short = "v", long = "verbose", global_raw = "true")]
    pub verbosity: u64,

    #[structopt(subcommand)]
    pub cmd: AppCommand,
}

#[derive(Debug, StructOpt)]
pub enum AppCommand {
    /// Load a playlist as the current playlist
    #[structopt(name = "load")]
    Load {
        /// Name of playlist to load
        name: String,
    },

    /// List information from the library
    #[structopt(name = "list", alias = "ls")]
    List {
        #[structopt(subcommand)]
        cmd: ListCommand,
    },

    /// Search the library; default returns only songs
    #[structopt(name = "search")]
    Search {
        /// Query to search with
        query: Vec<String>,

        /// Search only for artists
        #[structopt(short = "a", long = "artists",
                    conflicts_with = "only_albums")]
        only_artists: bool,

        /// Search only for albums
        #[structopt(short = "b", long = "albums",
                    conflicts_with = "only_artists")]
        only_albums: bool,

        /// Number of results to return
        #[structopt(short = "n", default_value = "20")]
        number: usize,
    },

    /// Play the current playlist
    #[structopt(name = "play")]
    Play,

    /// Suspend playback of the current playlist
    #[structopt(name = "pause")]
    Pause,

    /// Toggle between playing or paused states
    #[structopt(name = "toggle")]
    Toggle,

    /// Play the next song in the current playlist
    #[structopt(name = "next")]
    Next,

    /// Play the previous song in the current playlist
    #[structopt(name = "prev")]
    Prev,

    /// Shuffle the curent playlist
    #[structopt(name = "shuffle")]
    Shuffle,

    /// Load a number of random songs
    #[structopt(name = "random")]
    Random {
        #[structopt(default_value = "20")]
        number: usize,
    },

    /// Display the currently playing song
    #[structopt(name = "current")]
    Current,

    /// Clear the current playlist
    #[structopt(name = "clear")]
    Clear,

    /// Remove all but the currently playing song
    #[structopt(name = "crop")]
    Crop,

    /// Initiate a scan of the library
    #[structopt(name = "update")]
    Update,

    /// Add a song to the current playlist
    #[structopt(name = "add")]
    Add {
        /// Adds the first result for the query
        query: Vec<String>,
    },

    /// Add a song to play after the current song
    #[structopt(name = "addnext")]
    AddNext {
        /// Adds the first result for the query
        query: Vec<String>,
    },

    /// Display the status of the daemon
    #[structopt(name = "status")]
    Status,

    /// Control the client daemon
    #[structopt(name = "daemon")]
    Daemon {
        #[structopt(subcommand)]
        cmd: DaemonCommand,
    },

    /// Generate shell completions
    #[structopt(name = "completions")]
    Completions {
        /// Shell to generate completions for
        shell: Shell,
    }
}

#[derive(Debug, StructOpt)]
pub enum ListCommand {
    /// Display all playlists
    #[structopt(name = "playlist")]
    Playlist,

    /// Display all artists
    #[structopt(name = "artists")]
    Artist {
        /// Maximum number of artists to display
        #[structopt(short = "n", default_value = "20")]
        number: usize,
    },
}

#[derive(Debug, StructOpt)]
pub enum DaemonCommand {
    /// Starts the daemon
    #[structopt(name = "start")]
    Start,

    /// Stops the daemon
    #[structopt(name = "stop")]
    Stop,

    /// Restarts the daemon
    #[structopt(name = "restart")]
    Restart,
}
