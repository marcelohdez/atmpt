use std::{borrow::Cow, path::PathBuf};

use anyhow::bail;
use atmpt::{templates::Templates, Atmpt};
use clap::Parser;
use directories::ProjectDirs;

fn main() -> anyhow::Result<()> {
    let Some(dirs) = ProjectDirs::from("me", "marcelohdez", "Atmpt") else {
        bail!("Could not generate any directories for this OS!");
    };

    let args = Atmpt::parse();
    let action = args.after_action();

    let mut data_dir = Cow::Borrowed(dirs.data_dir());
    if let Some(new_dir) = &args.data_dir {
        data_dir = Cow::Owned(PathBuf::from(new_dir));
    };

    if let Some(template) = args.required.template {
        let Some(editor) = args.editor else {
            bail!("No editor to use! Set your $VISUAL variable or pass a command to --editor");
        };

        atmpt::try_template(&template, &editor, &data_dir, action)?;
    } else if args.required.list_template_dir {
        println!("{}", data_dir.display());
    } else {
        println!("{}", Templates::try_from(data_dir.as_ref())?);
    }

    Ok(())
}
