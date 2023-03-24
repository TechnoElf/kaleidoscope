use std::fs;
use std::env;
use std::path::Path;

use imgui::*;

pub struct FilePicker {
    id: String,
    dir: String,
    content: Vec<String>
}

impl FilePicker {
    pub fn new(id: String) -> Self {
        Self {
            id,
            dir: String::new(),
            content: Vec::new()
        }
    }

    pub fn open(&mut self, ui: &Ui) {
        ui.open_popup(&self.id);
        self.cd(env::current_dir().unwrap().to_string_lossy().into_owned() + "/");
    }

    pub fn update(&mut self, ui: &Ui) -> Option<String> {
        ui.modal_popup_config(&self.id.clone())
            .build(|| {
                ui.text(&self.dir);

                if let Some(_) = ui.begin_table("file_list", 1) {
                    ui.table_setup_column("Name");
                    ui.table_headers_row();

                    ui.table_next_column();
                    if ui.arrow_button("back", Direction::Left) {
                        let path = self.dir[0..(self.dir.len() - 1)].rsplit_once("/").unwrap_or(("", "")).0.to_string();
                        self.cd(path + "/");
                    }
                    ui.same_line();
                    ui.text("Back");


                    let mut choice = None;
                    for e in self.content.iter() {
                        ui.table_next_column();
                        if ui.arrow_button(e, Direction::Right) {
                            choice = Some(e);
                        }
                        ui.same_line();
                        ui.text(e);
                    }
                    if let Some(choice) = choice {
                        let path = self.dir.clone() + choice;
                        if Path::new(&path).is_dir() {
                            self.cd(path + "/");
                        } else {
                            ui.close_current_popup();
                            return Some(path)
                        }
                    }
                }

                None
            }).unwrap_or(None)
    }

    pub fn cd(&mut self, dir: String) {
        self.dir = dir;

        if let Ok(dir) = fs::read_dir(&self.dir) {
            self.content = dir
                .filter(|e| e.is_ok())
                .map(|e| e.unwrap().file_name().into_string().unwrap())
                .filter(|e| !e.starts_with("."))
                .collect();
        }
    }
}
