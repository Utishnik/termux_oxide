use confirm::TermuxConfirmDialog;

pub mod confirm;

pub struct TermuxDialog {
    pub confirm: TermuxConfirmDialog,
}

impl Default for TermuxDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl TermuxDialog {
    pub fn new() -> Self {
        TermuxDialog {
            confirm: TermuxConfirmDialog::new(),
        }
    }
}
