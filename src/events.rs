use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum Event {
    /// The window was closed.
    Closed,

    /// A file has been dropped into the window.
    DroppedFile(PathBuf),

    /// A native UI button was clicked.
    ButtonClicked,

    /// A notifier was sent from a WKWebView object.
    WebEvent(String, String), // change to String, String

    SliderUpdated(f32),
}
