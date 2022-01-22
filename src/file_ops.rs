use crate::paste_id;

use std::path::Path;
use rocket::Data;
use std::fs::File;

#[post("/", data="<paste>")]
pub fn upload(paste: Data) -> Result<String, std::io::Error>{
    let id = paste_id::PasteId::new(3);
    let file_name = format!("upload/{id}", id=id);
    let url = format!("{host}/{id}\n", host="http://localhost:8000", id=id);

    paste.stream_to_file(Path::new(&file_name))?;
    Ok(url)
}

#[get("/<id>")]
pub fn retrieve(id: paste_id::PasteId) -> Option<File> {
    let filename = format!("upload/{id}", id=id);
    File::open(&filename).ok()
}