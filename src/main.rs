#![deny(warnings)]
#![warn(clippy::all)]

use egui::*;
use egui_glium::storage::FileStorage;

#[derive(serde::Deserialize, serde::Serialize)]
struct MyApp {
    string_digits: String,
    window_size: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            string_digits: "1234567".to_owned(),
            window_size: 13,
        }
    }
}

impl egui::app::App for MyApp {
    /// This function will be called whenever the Ui needs to be shown,
    /// which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut dyn egui::app::Backend) {
        let MyApp {
            string_digits,
            window_size,
        } = self;

        ui.horizontal(|ui| {
            ui.label("Enter a bunch of numbers");
            ui.text_edit(string_digits);
        });
        ui.horizontal(|ui| {
            ui.label("Enter a window size");
            ui.add(Slider::usize(window_size, 1..=200000).text(""));
            *window_size = (*window_size).max(1);
        });

        let digits = string_to_digits(string_digits.trim());
        match digits {
            Some(digits) => {
                ui.label(format!("My digits {:?}", digits));
                ui.label(format!(
                    "The largest product of the window is {}",
                    calculate_largest_window(*window_size, digits)
                ));
            }
            None => {
                ui.label(Label::new("Please input numbers only").text_color(color::RED));
            }
        }
    }

    fn on_exit(&mut self, storage: &mut dyn egui::app::Storage) {
        egui::app::set_value(storage, egui::app::APP_KEY, self);
    }
}

fn main() {
    let title = "My Egui Window";
    let storage = FileStorage::from_path(".egui_example_glium.json".into()); // Where to persist app state
    let app: MyApp = egui::app::get_value(&storage, egui::app::APP_KEY).unwrap_or_default(); // Restore `MyApp` from file, or create new `MyApp`.
    egui_glium::run(title, storage, app);
}

//THIS FUNCTION CALCULATES THE WINDOW OF N SIZE WITH THE LARGEST PRODUCT IN A GIVEN SET OF NUMBERS
fn calculate_largest_window(window_size: usize, digits_vec: Vec<u64>) -> u64 {
    let mut largest_product = 0;

    for window in digits_vec.windows(window_size) {
        let window_product = window.iter().product::<u64>();

        if window_product > largest_product {
            largest_product = window_product;
        }
    }
    largest_product
}

fn string_to_digits(string: &str) -> Option<Vec<u64>> {
    let mut vector = Vec::new();
    for chr in string.chars() {
        let x = chr.to_digit(10); //to_digit returns Some(u32) if char = 0-9 (using radix 10) or None if none.
        match x {
            Some(d) => vector.push(d as u64), //if x has Some(value)then push the value to vector.
            None => return None, //if x is a None value then the function stops immediately and returns None
        }
    }
    Some(vector)
}
