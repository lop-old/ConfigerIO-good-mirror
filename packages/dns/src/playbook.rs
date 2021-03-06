
use log::{ info, debug };

use serde_json::json;

use configer_common::FileDAO;
use configer_common::{
	load_tpl,
	render_tpl,
};
use configer_common::utils::{
	get_timestamp,
};

use crate::Configuration;
use crate::configuration::{
	DomainDetails,
};



pub const SERVICE_NAME:  &str = "dns";
pub const SERVICE_TITLE: &str = "bind/named";



pub fn load_templates(cfg: &Configuration, tpl_path: String) -> Vec<FileDAO> {
	let mut book: Vec<FileDAO> = Vec::new();
	// /etc/named.conf
	book.push(FileDAO::new(
		"/etc/named.conf".to_string(),
		tpl_path.clone()
	));
	// /etc/named/<domain>.zone
	{
		let mut f = |dom: String| {
			let dest_file = format!("/etc/named/{}.zone", dom.clone());
			let tpl_file  = format!("{}/etc-named-domain.zone.tpl", tpl_path.clone());
			book.push(FileDAO::new(
				dest_file.clone(),
				tpl_file.clone(),
			));
		};
		for (domain, _) in &cfg.dns_internal {
			f(domain.clone());
		}
		for (domain, _) in &cfg.dns_external {
			f(domain.clone());
		}
	}
	book
}



pub fn generate_configs(cfg: &Configuration, book: &Vec<FileDAO>) {
	info!("Generating configs for: {}", SERVICE_TITLE);
	let timestamp = get_timestamp();

	// /etc/named.conf
	{
		let dao = FileDAO::get_by_dest(&book, "/etc/named.conf".to_string());
		debug!("Generating: {} as: {}", dao.dest_file.clone(), dao.tmp_file.clone());
		let tpl = load_tpl(dao.tpl_file.clone());
		let tags = json!({
			"timestamp": timestamp.clone(),
			"cfg": &cfg,
		});
		render_tpl(&dao, &tpl, &tags);
	}

	// /etc/named/<domain>.zone
	{
		let f = |dom: String, det: &DomainDetails, is_int: bool| {
			let dest_file = format!("/etc/named/{}.zone", dom.clone());
			let dao = FileDAO::get_by_dest(&book, dest_file.clone());
			debug!("Generating: {} as: {}", dao.dest_file.clone(), dao.tmp_file.clone());
			let tpl = load_tpl(dao.tpl_file.clone());
			let tags = json!({
				"timestamp": timestamp.clone(),
				"is_internal":   is_int,
				"is_external": ! is_int,
				"domain":  dom.clone(),
				"details": &det,
			});
			render_tpl(&dao, &tpl, &tags);
		};
		for (domain, details) in &cfg.dns_internal {
			f(domain.clone(), details, true);
		}
		for (domain, details) in &cfg.dns_external {
			f(domain.clone(), details, false);
		}
	}

}
