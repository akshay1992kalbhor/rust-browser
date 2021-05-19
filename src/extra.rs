use druid_shell::kurbo::{Line, Size};
use druid_shell::piet::{Color, RenderContext};
use druid_shell::{
    Application, Cursor, FileDialogOptions, FileDialogToken, FileInfo, FileSpec, HotKey, Menu,
    Region, SysMods, WinHandler, WindowBuilder, WindowHandle,
};

#[derive(Default)]
struct HelloState {
    size: Size,
    handle: WindowHandle,
}
impl WinHandler for HelloState {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn paint(&mut self, piet: &mut druid_shell::piet::Piet, _: &Region) {
        let rect = self.size.to_rect();
        piet.fill(rect, &BG_COLOR);
        piet.stroke(Line::new((10.0, 50.0), (90.0, 90.0)), &FG_COLOR, 1.0);
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn prepare_paint(&mut self) {}

    fn size(&mut self, size: Size) {
        self.size = size;
    }

    fn rebuild_resources(&mut self) {}

    fn command(&mut self, id: u32) {
        match id {
            0x100 => {
                self.handle.close();
                Application::global().quit();
            }
            0x101 => {
                let options = FileDialogOptions::new().show_hidden().allowed_types(vec![
                    FileSpec::new("Rust Files", &["rs", "toml"]),
                    FileSpec::TEXT,
                    FileSpec::JPG,
                ]);
                self.handle.open_file(options);
            }
            _ => println!("Unexpected id: {}", id),
        }
    }

    fn save_as(&mut self, token: druid_shell::FileDialogToken, file: Option<druid::FileInfo>) {}

    fn open_file(&mut self, token: druid_shell::FileDialogToken, file: Option<druid::FileInfo>) {
        println!("open file result: {:?}", file);
    }

    fn key_down(&mut self, event: druid::KeyEvent) -> bool {
        println!("keydown: {:?}", event);
        false
    }

    fn key_up(&mut self, event: druid::KeyEvent) {
        println!("keyup: {:?}", event);
    }

    fn wheel(&mut self, event: &druid_shell::MouseEvent) {}

    fn zoom(&mut self, delta: f64) {}

    fn mouse_move(&mut self, event: &druid_shell::MouseEvent) {}

    fn mouse_down(&mut self, event: &druid_shell::MouseEvent) {}

    fn mouse_up(&mut self, event: &druid_shell::MouseEvent) {}

    fn mouse_leave(&mut self) {}

    fn got_focus(&mut self) {}

    fn lost_focus(&mut self) {}

    fn request_close(&mut self) {
        self.handle.close();
    }

    fn destroy(&mut self) {
        Application::global().quit();
    }
}
