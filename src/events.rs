use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum Event {
    /// The window will close.
    WindowWillClose,

    /// A file has been dropped into the window.
    DroppedFile(PathBuf),

    /// A file has been dragged over the window and dragged out again.
    DraggingExited,

    /// A file has been dragged over the window.
    DraggingEntered(PathBuf),

    /// A native UI button was clicked.
    ButtonClicked(String),

    /// A notifier was sent from a WKWebView object.
    WebEvent(String, String), // change to String, String
    WebViewStartedLoading,
    WebViewFinishedLoading,

    SliderUpdated(String, f32),
}
