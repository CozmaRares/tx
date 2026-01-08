use crate::{
    cli::PreviewArgs,
    data::{TmuxSession, TxDirectory, TxLayout},
};

pub fn handle_preview(args: PreviewArgs) -> anyhow::Result<()> {
    if let Some(session) = args.session {
        return TmuxSession::preview(&session);
    }

    if let Some(layout) = args.layout {
        return TxLayout::new(layout).preview();
    }

    if let Some(dir) = args.directory {
        let Some(dir) = TxDirectory::find(&dir) else {
            anyhow::bail!("Directory not found");
        };
        return dir.preview()
    }

    anyhow::bail!("No preview type specified");
}
