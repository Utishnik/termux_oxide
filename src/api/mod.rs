use battery_status::TermuxBatteryStatus;
use brightness::TermuxBrightness;
use camera::TermuxCamera;
use clipboard::TermuxClipboard;
use dialog::TermuxDialog;
use notification::TermuxNotification;
use toast::TermuxToast;

pub mod audio;
pub mod battery_status;
pub mod brightness;
pub mod call;
pub mod camera;
pub mod clipboard;
pub mod contact_list;
pub mod dialog;
pub mod download;
pub mod errors;
pub mod notification;
pub mod run;
pub mod toast;

pub struct Termux {
    pub battery: TermuxBatteryStatus,
    pub brightness: TermuxBrightness,
    pub notification: TermuxNotification,
    pub toast: TermuxToast,
    pub camera: TermuxCamera,
    pub clipboard: TermuxClipboard,
    pub dialog: TermuxDialog,
}

impl Termux {
    pub fn new() -> Termux {
        Termux {
            battery: TermuxBatteryStatus::new(),
            brightness: TermuxBrightness::new(),
            notification: TermuxNotification::new(),
            toast: TermuxToast::new(),
            camera: TermuxCamera::new(),
            clipboard: TermuxClipboard::new(),
            dialog: TermuxDialog::new(),
        }
    }
}

impl Default for Termux {
    fn default() -> Self {
        Self::new()
    }
}
