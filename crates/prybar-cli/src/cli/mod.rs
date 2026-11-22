//! Argument definitions. Help text is written for agents too: every flag
//! says what it expects and what comes back.

mod model;
mod package;
mod tools;

pub use model::{DecompileArgs, InspectArgs};
pub use package::{ListArgs, PackArgs, UnpackArgs};
pub use tools::{CompileArgs, GamesArgs, PublishArgs, VersionArgs, ViewArgs};

use clap::{Parser, Subcommand};

const EXIT_CODES: &str = "\
Exit codes:
  0  success
  1  internal error
  2  usage error (bad arguments, unknown game, missing tool path)
  3  input not found
  4  invalid input (malformed model, package or Workshop item)
  5  I/O error
  6  external tool failed (studiomdl, hlmv, Steam)
  7  batch partly failed (see the report's failures)";

/// Decompile Source engine models (MDL 04-53) and list, unpack or pack
/// VPK, GMA, APK and HFS packages. Built for scripts and AI agents.
#[derive(Debug, Parser)]
#[command(name = "prybar", version, after_help = EXIT_CODES)]
pub struct Cli {
    /// Print one machine-readable JSON document on stdout.
    #[arg(long, global = true)]
    pub json: bool,
    /// No progress and no human output (errors still go to stderr).
    #[arg(long, short, global = true)]
    pub quiet: bool,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Summarize a model: version, bones, meshes, materials, sequences.
    Inspect(InspectArgs),
    /// Decompile models to QC, SMD, VTA and VRD.
    Decompile(DecompileArgs),
    /// List the files in a VPK, GMA, APK or HFS package.
    List(ListArgs),
    /// Extract every file of a package.
    Unpack(UnpackArgs),
    /// Build a VPK or GMA from a folder.
    Pack(PackArgs),
    /// Compile a QC with the game's studiomdl.
    Compile(CompileArgs),
    /// Open a model in the game's hlmv.
    View(ViewArgs),
    /// Create or update a Steam Workshop item, or list yours.
    Publish(PublishArgs),
    /// Show configured games; detect Steam installs.
    Games(GamesArgs),
    /// Print the version; `--check` asks GitHub for the latest release.
    Version(VersionArgs),
    /// Download, verify and install the latest release (or print the
    /// package-manager command when prybar is managed by one).
    SelfUpdate,
}
