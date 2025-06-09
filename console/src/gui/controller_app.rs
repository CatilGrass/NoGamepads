pub struct MyApp {
    show_hello: bool, // 控制是否显示 "Hello World!"
}

impl Default for MyApp {
    fn default() -> Self {
        Self { show_hello: false }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 添加一个按钮
            if ui.button("Click me!").clicked() {
                self.show_hello = !self.show_hello; // 切换状态
            }

            // 如果按钮被点击过，显示 "Hello World!"
            if self.show_hello {
                ui.label("Hello World!");
            }
        });
    }
}

impl MyApp {
    pub fn start () {
        let options = eframe::NativeOptions::default();
        let _ = eframe::run_native(
            "My egui App",
            options,
            Box::new(|_cc| {
                Ok(Box::<MyApp>::default())
            }),
        );
    }
}