#![windows_subsystem = "windows"]

use eframe::egui::{self, CentralPanel, Context, FontDefinitions, FontData, Pos2, Rect, Vec2, Label, Button};
use eframe::{self, App, Frame};
use std::fs;

fn create_rect(x: f32, y: f32, width: f32, height: f32) -> Rect {
    Rect::from_min_size(Pos2::new(x, y), Vec2::new(width, height))
}
fn nadd(i: i32, mut n: i32)-> i32 {
    n = n * 10;
    n += i;
    return n;
}

struct Dentaku {
    hyouji: i32,
    enzan: i32,
    kirikae: bool,
    data1: i32,
    data2: i32
}

impl Default for Dentaku {
    fn default() -> Self {
        Self {
            hyouji: 0,
            enzan: 0,
            kirikae: false,
            data1: 0,
            data2: 0
        }
    }
}

impl App for Dentaku {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        // カスタムフォントの設定を呼び出す
        set_custom_fonts(ctx);
        CentralPanel::default().show(ctx, |ui| {
            let bwh = 47.0;

            ui.put(create_rect(0.0, 0.0, 200.0, 50.0), Label::new(format!("{}", self.hyouji)));
            
            // Labelを使って、ウィジェットとして配置
            if ui.put(create_rect(0.0, 50.0, bwh, bwh), Button::new("7")).clicked(){
                self.hyouji = nadd(7, self.hyouji);
            }
            if ui.put(create_rect(50.0, 50.0, bwh, bwh), Button::new("8")).clicked(){
                self.hyouji = nadd(8, self.hyouji);
            }
            if ui.put(create_rect(100.0, 50.0, bwh, bwh), Button::new("9")).clicked(){
                self.hyouji = nadd(9, self.hyouji);
            }

            if ui.put(create_rect(0.0, 100.0, bwh, bwh), Button::new("4")).clicked(){
                self.hyouji = nadd(4, self.hyouji);
            }
            if ui.put(create_rect(50.0, 100.0, bwh, bwh), Button::new("5")).clicked(){
                self.hyouji = nadd(5, self.hyouji);
            }
            if ui.put(create_rect(100.0, 100.0, bwh, bwh), Button::new("6")).clicked(){
                self.hyouji = nadd(6, self.hyouji);
            }

            if ui.put(create_rect(0.0, 150.0, bwh, bwh), Button::new("1")).clicked(){
                self.hyouji = nadd(1, self.hyouji);
            }
            if ui.put(create_rect(50.0, 150.0, bwh, bwh), Button::new("2")).clicked(){
                self.hyouji = nadd(2, self.hyouji);
            }
            if ui.put(create_rect(100.0, 150.0, bwh, bwh), Button::new("3")).clicked(){
                self.hyouji = nadd(3, self.hyouji);
            }
            
            if ui.put(create_rect(0.0, 200.0, bwh, bwh), Button::new("C")).clicked(){
                self.hyouji = 0;
                self.enzan = 0;
                self.kirikae = false;
                self.data1 = 0;
                self.data2 = 0;
            }
            if ui.put(create_rect(50.0, 200.0, bwh, bwh), Button::new("0")).clicked(){
                self.hyouji = nadd(0, self.hyouji);
            }
            if ui.put(create_rect(100.0, 200.0, bwh, bwh), Button::new("=")).clicked(){
                self.data2 = self.hyouji;
                match self.enzan {
                    1 => self.hyouji = self.data1 + self.data2,
                    2 => self.hyouji = self.data1 - self.data2,
                    3 => self.hyouji = self.data1 * self.data2,
                    4 => self.hyouji = self.data1 / self.data2,
                    _ => (),
                }
            }

            if ui.put(create_rect(150.0, 50.0, bwh, bwh), Button::new("/")).clicked(){
                self.enzan = 4;
                if self.kirikae == false {
                    self.data1 = self.hyouji;
                    self.hyouji = 0;
                    self.kirikae = true;
                }
            }
            if ui.put(create_rect(150.0, 100.0, bwh, bwh), Button::new("*")).clicked(){
                self.enzan = 3;
                if self.kirikae == false {
                    self.data1 = self.hyouji;
                    self.hyouji = 0;
                    self.kirikae = true;
                }
            }
            if ui.put(create_rect(150.0, 150.0, bwh, bwh), Button::new("-")).clicked(){
                self.enzan = 2;
                if self.kirikae == false {
                    self.data1 = self.hyouji;
                    self.hyouji = 0;
                    self.kirikae = true;
                }
            }
            if ui.put(create_rect(150.0, 200.0, bwh, bwh), Button::new("+")).clicked(){
                self.enzan = 1;
                if self.kirikae == false {
                    self.data1 = self.hyouji;
                    self.hyouji = 0;
                    self.kirikae = true;
                }
            }
        });
    }
}

// システムフォントを読み込んで設定する関数
fn set_custom_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    // Windowsの游ゴシックフォントのパス
    let font_path = r"C:\Windows\Fonts\YuGothR.ttc";  // Windowsの游ゴシックの例

    // フォントファイルを読み込む
    let font_data = fs::read(font_path).expect("フォントファイルの読み込みに失敗しました");

    // フォントデータを egui に登録
    fonts.font_data.insert(
        "my_font".to_owned(),
        FontData::from_owned(font_data),
    );

    // Proportional（可変幅フォント）の先頭にカスタムフォントを追加
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Monospace（等幅フォント）にもカスタムフォントを追加
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // フォントをセット
    ctx.set_fonts(fonts);
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(Vec2::new(200.0,250.0)),
        ..Default::default()
    };

    eframe::run_native(
        "電卓",
        options,
        Box::new(|_cc| Ok(Box::new(Dentaku::default()))),
    )
}
