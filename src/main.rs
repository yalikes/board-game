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
    main_grid
        .child_at(2, 2)
        .unwrap()
        .add_css_class(constants::SELECTED_CSS);
    gamemodel.lock().unwrap().select_1 = Some((2, 2));
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
    if let Some(pos1) = (*gamemodel_ref).select_1 {
        if pos1 == pos {
            (*gamemodel_ref).select_1 = None;
            grid.child_at(pos.1 as i32, pos.0 as i32)
                .unwrap()
                .remove_css_class(constants::SELECTED_CSS);
        } else if (*gamemodel_ref).gamemap.is_valid_action(pos1, pos) {
            grid.child_at(pos1.1 as i32, pos1.0 as i32)
                .unwrap()
                .remove_css_class(constants::SELECTED_CSS);
            (*gamemodel_ref).gamemap.swap_pos(pos1, pos);
            (*gamemodel_ref).select_1 = None;
            (*gamemodel_ref).select_2 = None;

            let label_text1 = match (*gamemodel_ref).gamemap.map[pos1.0][pos1.1] {
                Some(i) => format!("{}", i),
                None => String::from(" "),
            };

            let label_text2 = match (*gamemodel_ref).gamemap.map[pos.0][pos.1] {
                Some(i) => format!("{}", i),
                None => String::from(" "),
            };

            grid.child_at(pos1.1 as i32, pos1.0 as i32)
                .unwrap()
                .downcast::<Button>()
                .expect("downcast widget to button failed!")
                .set_label(&label_text1);
            grid.child_at(pos.1 as i32, pos.0 as i32)
                .unwrap()
                .downcast::<Button>()
                .expect("downcast widget to button failed!")
                .set_label(&label_text2);
        }
    } else if (*gamemodel_ref).gamemap.is_valid_selection(pos) {
        (*gamemodel_ref).select_1 = Some(pos);
        grid.child_at(pos.1 as i32, pos.0 as i32)
            .unwrap()
            .downcast::<Button>()
            .expect("downcast widget to button failed!")
            .add_css_class(constants::SELECTED_CSS);
    }
    if (*gamemodel_ref).gamemap.is_completed() {
        println!("YOU WIN!");
    }
}
