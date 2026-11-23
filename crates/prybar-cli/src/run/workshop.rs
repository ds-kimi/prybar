use std::io::Read as _;

use super::pretty;
use crate::cli::{PrivacyArgs, TextureArgs, WorkshopArgs, WorkshopCommand};
use crate::output::Output;
use prybar_service::workshop::{self, ItemQuery};
use prybar_service::{privacy, texture, tools, Result, ServiceError};

pub fn workshop(a: &WorkshopArgs, out: &Output) -> Result<u8> {
    match &a.command {
        WorkshopCommand::Items(i) => {
            let q = ItemQuery {
                search: i.search.clone(),
                sort: i.sort.clone(),
                reverse: i.reverse,
                visibility: i.visibility.clone(),
                tag: i.tag.clone(),
            };
            out.emit(
                &workshop::workshop_items(tools::workshop_app_id(&i.game)?, &q)?,
                pretty,
            );
        }
        WorkshopCommand::Item(i) => {
            let app = tools::workshop_app_id(&i.game)?;
            out.emit(&workshop::workshop_item(app, i.item_id)?, pretty);
        }
        WorkshopCommand::Preview(p) => out.emit(&workshop::check_preview(&p.image)?, pretty),
        WorkshopCommand::Bbcode(b) => {
            out.emit(&workshop::check_bbcode(&read_text(&b.file)?), pretty)
        }
    }
    Ok(0)
}

/// `-` reads stdin so an agent can pipe a draft without a temp file.
fn read_text(file: &std::path::Path) -> Result<String> {
    if file.as_os_str() == "-" {
        let mut s = String::new();
        std::io::stdin()
            .read_to_string(&mut s)
            .map_err(|e| ServiceError::io(file, e))?;
        return Ok(s);
    }
    std::fs::read_to_string(file).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => ServiceError::NotFound(file.display().to_string()),
        _ => ServiceError::io(file, e),
    })
}

pub fn texture(a: &TextureArgs, out: &Output) -> Result<u8> {
    let report = match &a.material {
        Some(m) => texture::export_material(&a.input, m, a.level, &a.out)?,
        None => texture::export_file(std::path::Path::new(&a.input), a.level, &a.out)?,
    };
    out.emit(&report, pretty);
    Ok(0)
}

pub fn privacy(a: &PrivacyArgs, out: &Output) -> Result<u8> {
    let set = (a.on || a.off).then_some(a.on);
    out.emit(&privacy::privacy(set, a.clear_log)?, pretty);
    Ok(0)
}
