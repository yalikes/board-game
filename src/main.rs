use std::sync::Arc;
use std::sync::Mutex;

use gdk::Display;
use glib::clone;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, CssProvider, Grid, Label, StyleContext};

mod constants;
mod pushbox_utils;
use pushbox_utils::GameModel;

fn main() {
    let gamemodel: Arc<Mutex<GameModel>> = Arc::new(Mutex::new(GameModel::new()));
    let app = Application::builder()
        .application_id("org.yalikes.pushbox")
        .build();
    app.connect_startup(|_| load_css());
    let gamemodel_ref = Arc::clone(&gamemodel);
    app.connect_activate(move |app: &Application| build_ui(app, &gamemodel_ref));
    app.run();
}

fn build_ui(app: &Application, gamemodel: &Arc<Mutex<GameModel>>) {
    let main_grid = Grid::builder()
        .column_homogeneous(true)
        .row_homogeneous(true)
        .column_spacing(1)
        .row_spacing(1)
        .build();
    for i in 0..3 {
        for j in 0..3 {
            let label_text = match gamemodel.lock().unwrap().gamemap.map[i][j] {
                Some(i) => format!("{}", i),
                None => String::from(" "),
            };
            let button = Button::builder().label(&label_text).build();

            let gamemodel_ref = Arc::clone(&gamemodel);
            button.connect_clicked(
                clone!(@weak main_grid => move |_| board_clicked(&gamemodel_ref, main_grid, (i,j))),
            );
            main_grid.attach(&button, j as i32, i as i32, 1, 1);
        }
    }
    let window = ApplicationWindow::builder()
        .application(app)
        .title("push box")
        .resizable(false)
        .default_height(800)
        .default_width(800)
        .child(&main_grid)
        .build();
    window.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("style.css");
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn board_clicked(gamemodel: &Arc<Mutex<GameModel>>, grid: Grid, pos: (usize, usize)) {
    let mut gamemodel_ref = gamemodel.lock().unwrap();

    if (*gamemodel_ref).is_valid_action(pos) {
        let empty_pos = (*gamemodel_ref).empty_pos;
        let i = (*gamemodel_ref).gamemap.map[pos.0][pos.1].unwrap();
        (*gamemodel_ref).empty_pos = pos;
        (*gamemodel_ref).gamemap.swap_pos(empty_pos, pos);
        let empty_text = " ";

        grid.child_at(pos.1 as i32, pos.0 as i32)
            .unwrap()
            .downcast::<Button>()
            .expect("downcast widget to button failed!")
            .set_label(&empty_text);
        grid.child_at(empty_pos.1 as i32, empty_pos.0 as i32)
            .unwrap()
            .downcast::<Button>()
            .expect("downcast widget to button failed!")
            .set_label(&format!("{}", i));
    }
    if (*gamemodel_ref).gamemap.is_completed() {
        println!("YOU WIN!");
    }
}
