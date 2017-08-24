pub trait ViewController {
    fn on_mouse_down(&mut self);
    fn on_file_drop(&mut self, path: String);
}

pub struct Controller {
    controller: Box<ViewController>,
}

impl Controller {
    pub fn new(controller: Box<ViewController>) -> Self {
        Controller { controller: controller }
    }
}

impl ViewController for Controller {
    fn on_mouse_down(&mut self) {
        self.controller.on_mouse_down();
    }

    fn on_file_drop(&mut self, path: String) {
        self.controller.on_file_drop(path);
    }
}
