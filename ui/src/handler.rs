use std::fs::write;
use std::fs::File;
use eframe::epi::App;
use egui::Color32;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum PageNumErr {
    MaxErr,
    MinErr,
    ValueErr,
}


pub struct Event {
    pub name_buffer: String,
    pub min_page: String,
    pub max_page: String,
    pub raw_data: String,
    pub name_file: File,
    pub rawdata_file: File,
    pub minpage_file: File,
    pub maxpage_file: File,
    pub page_err: Option<PageNumErr>,
    pub name_err: bool,
    pub rawdata_err: bool,
}

impl App for Event {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        egui::TopBottomPanel::top("header").default_height(100.).show(ctx, |ui| {
            ui.add_sized([100.0, 100.0],  egui::Label::new("Property Agent Scrapper").underline());
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(5.);
            let master_button = ui.button("Edit All");
            if master_button.clicked() {
                if check_page_number(&self.min_page) {
                    self.page_err = Some(PageNumErr::MinErr);
                } else if check_page_number(&self.max_page) {
                    self.page_err = Some(PageNumErr::MaxErr);
                } else if check_value(&self.min_page, &self.max_page) {
                    self.page_err = Some(PageNumErr::ValueErr);
                } else {
                    self.page_err = None;
                    //write to page file
                    let _writed = write("minpage.txt", &self.min_page).unwrap();

                    let _writed = write("maxpage.txt", &self.max_page).unwrap();

                    write("name.txt", &self.name_buffer).unwrap();
                    write("rawdata.txt", &self.raw_data).unwrap();
                    
                }
            }
            //referesh button
            

            ui.add_space(5.);
            egui::ScrollArea::vertical().show(ui, |ui| {
                /* 
                //name section
                ui.label("insert name");
                if self.name_err {
                    ui.colored_label(Color32::RED, "Something wrong with your name");
                }
                ui.add_sized([500., 10.],  egui::TextEdit::singleline(&mut self.name_buffer));
                let name_button = ui.button("Edit Name Only");
                if name_button.clicked() {
                    write("name.txt", &self.name_buffer).unwrap();
                }
                ui.add_space(20.);
                */

                //page section
                ui.label("Number ID");
                if self.page_err == Some(PageNumErr::MaxErr) {
                    ui.colored_label(Color32::RED, "your min number should be all digit; eg: 100000");
                }
                if self.page_err == Some(PageNumErr::MinErr) {
                    ui.colored_label(Color32::RED, " you max number should be all digit; eg: 100000");
                }
                if self.page_err == Some(PageNumErr::ValueErr) {
                    ui.colored_label(Color32::RED, "min digit is higher than max digit");
                }
                ui.horizontal(|ui| {
                    ui.add_sized([100., 10.],  egui::TextEdit::singleline(&mut self.min_page));
                    ui.label("-");
                    ui.add_sized([100., 10.],  egui::TextEdit::singleline(&mut self.max_page));
                });
                let page_button = ui.button("Edit Number ID Only");
                if page_button.clicked() {
                    if check_page_number(&self.min_page) {
                        self.page_err = Some(PageNumErr::MinErr);
                    } else if check_page_number(&self.max_page) {
                        self.page_err = Some(PageNumErr::MaxErr);
                    } else if check_value(&self.min_page, &self.max_page) {
                        self.page_err = Some(PageNumErr::ValueErr);
                    } else {
                        self.page_err = None;
                        //write to page file
                        let _writed = write("minpage.txt", &self.min_page).unwrap();

                        let _writed = write("maxpage.txt", &self.max_page).unwrap();
                        
                    }
                }
                ui.add_space(20.);

                //rawdata section
                ui.label("Raw Data");
                if self.rawdata_err {
                    ui.colored_label(Color32::RED, "Something wrong with your rawdata");
                }
                ui.add_sized([900., 10.],  egui::TextEdit::multiline(&mut self.raw_data).desired_rows(20));
                let rawdata_button = ui.button("Edit RawData Only");
                if rawdata_button.clicked() {
                    write("rawdata.txt", &self.raw_data).unwrap();
                }
            });
        });
    }

    fn name(&self) -> &str {
        "Property Agent Scrapper"
    }
}

fn check_page_number(page: &str) -> bool {
    match page.trim().parse::<i32>() {
        Ok(_min_page) => {
            return false
        },
        Err(_) => {
            return true
        },
    }
    
}

fn check_value(minpage: &str, maxpage: &str) -> bool {
    let minpage = minpage.trim().parse::<i32>().unwrap();
    let maxpage = maxpage.trim().parse::<i32>().unwrap(); 

    if minpage > maxpage {
        return true
    }

    false
}

