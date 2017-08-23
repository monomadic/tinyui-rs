pub trait ViewController {
    fn on_mouse_down(&mut self);
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
}
