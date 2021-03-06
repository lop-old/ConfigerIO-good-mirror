
use log::{ info, debug, warn };
use log::Level;
use log::log_enabled;

use clap::{ Parser, Subcommand };

use configer_common::utils::init_log_verbosity;
use configer_common::display::{ display_cat, display_diff };
use configer_common::FileDAO;

use crate::configuration::Configuration;
use crate::playbook::{
	load_templates,
	generate_configs,
	SERVICE_TITLE,
};

mod configuration;
mod playbook;



pub const DEFAULT_CONFIG_PERMISSIONS: u32 = 0o644;



#[derive(Parser)]
#[clap(author, version, about)]
#[clap(subcommand_required=true, arg_required_else_help=true)]
struct Args {

	/// Perform a trial run with no changes made
	#[clap(short, long, global=true, display_order=1)]
	dry: bool,

	/// Use newly generated config without testing
	#[clap(short, long, global=true, display_order=2)]
	force: bool,

	/// Display the generated config files
	#[clap(short='C', long, global=true, display_order=3)]
	cat: bool,

	/// Display differences between the existing and newly generated config files
	#[clap(short='D', long, global=true, display_order=4)]
	diff: bool,

	/// More output per occurrence
	#[clap(short, long, global=true, parse(from_occurrences), display_order=25)]
	verbose: i8,

	/// Less output per occurrence
	#[clap(short, long, global=true, parse(from_occurrences), display_order=26)]
	quiet: i8,

	#[clap(subcommand)]
	command: Option<Commands>,

}

#[derive(Subcommand)]
enum Commands {

	/// Generate config files for bind/named
	#[clap(visible_alias="gen")]
	Generate {

		/// Archive the existing config files
		#[clap(short, long, display_order=11)]
		backup: bool,

		/// Install the generated config files
		#[clap(short, long, display_order=12)]
		install: bool,

		/// Set the path to configer.json
		#[clap(long, takes_value=true, value_name="json", alias="configer-config", display_order=13)]
		configer_file: Option<String>,

		/// Set the path to template files
		#[clap(long, takes_value=true, value_name="path", display_order=14)]
		tpl_path: Option<String>,

	},

}



fn main() {
	let args: Args = Args::parse();

	// log verbose/quiet
	init_log_verbosity(args.verbose, args.quiet);

	log_panics::init();
	if log_enabled!(Level::Warn) {
		if args.dry   { if log_enabled!(Level::Info) { info!("DRY"  ); } else { println!(" [DRY] "  ); }}
		if args.force { if log_enabled!(Level::Info) { info!("FORCE"); } else { println!(" [FORCE] "); }}
	}

	// handle command
	match &args.command {

		Some(Commands::Generate { backup, install, configer_file, tpl_path }) => {
			// configer.json file
			let cfg_file_str = configer_common::find_configer_file(configer_file);
			// templates path
			let tpl_path_str = configer_common::find_templates_path(tpl_path, playbook::SERVICE_NAME.to_string());
			// load config
			let cfg: Configuration = Configuration::load( cfg_file_str.clone() );
			// load templates
			let book = load_templates(&cfg, tpl_path_str.clone());
			// generate config files
			generate_configs(&cfg, &book);
			// --cat
			if args.cat {
				display_cat(&book);
			}
			// --diff
			if args.diff {
				display_diff(&book);
			}
			if *install {
				// backup configs
				if *backup {
					if args.dry { warn!("Skipping backup.."); } else {
						backup_configs(&book);
					}
				}
				// install configs
				if args.dry { warn!("Skipping install.."); } else {
					install_configs(&book);
				}
			}
		}

		None => { },

	};
}



pub fn backup_configs(_book: &Vec<FileDAO>) {
	warn!("BACKUP UNFINISHED");
}



use std::os::unix::fs::PermissionsExt;
pub fn install_configs(book: &Vec<FileDAO>) {
	info!("Installing configs for: {}", SERVICE_TITLE);
	for dao in book {
		debug!("Install: {} From: {}", dao.dest_file.clone(), dao.tmp_file.clone());
		std::fs::copy( dao.tmp_file.clone(), dao.dest_file.clone() )
			.unwrap_or_else(|e| panic!("Failed to install config: {} {}", dao.dest_file.clone(), e));
		// set permissions
		std::fs::set_permissions(
			dao.dest_file.clone(),
			std::fs::Permissions::from_mode(DEFAULT_CONFIG_PERMISSIONS)
		).unwrap();
		// set owner/group
//TODO
	}
}
