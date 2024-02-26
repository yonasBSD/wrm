use crate::{utils::*, FilesInTrash, Result, FILES_IN_TRASH, WRM_PATH};
use filey::Filey;

pub fn finish() -> Result<()> {
    if !Filey::new(absolutize(WRM_PATH)?).exists() {
        return Ok(());
    }

    let mut files_in_trash = FilesInTrash::read(absolutize(FILES_IN_TRASH)?)?;

    for file in files_in_trash.clone().files_in_trash() {
        if !Filey::new(file.path()).exists() {
            files_in_trash.remove(file);
        }
    }

    files_in_trash.write(absolutize(FILES_IN_TRASH)?)?;

    Ok(())
}
