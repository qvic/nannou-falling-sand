use std::fs::File;
use std::ops::Add;

use nannou::color::rgb_u32;
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;

use crate::{
    game::Simulation,
    materials::IndexShift,
    materials::Material,
    materials::MaterialId,
    materials::Materials,
    materials::Movement,
    materials::MovementRule,
    materials::MaterialColor,
    view::GameView,
};

mod game;
mod materials;
mod view;

const WINDOW_TITLE: &'static str = "Falling sand";

const WINDOW_WIDTH_PX: u32 = 800;
const WINDOW_HEIGHT_PX: u32 = 835;

const GRID_WIDTH_CELLS: usize = 200;
const GRID_HEIGHT_CELLS: usize = 200;

const MOUSE_SPAWN_RADIUS: u8 = 5;

const BACKGROUND: u32 = 0xEEE6CE;

const REDRAW_FPS_FRAMES: u64 = 16;

const TOP_BAR_PAD: f32 = 35.0;
const TOP_BAR_INFO_WIDTH: f32 = 100.0;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .title(WINDOW_TITLE)
        .size(WINDOW_WIDTH_PX, WINDOW_HEIGHT_PX)
        .resizable(false)
        .clear_color(rgb_u32(BACKGROUND))
        .view(view)
        .event(event)
        .build()
        .unwrap();

    let file = File::open("materials.json")
        .expect("Unable to open configuration file 'materials.json'");
    let materials: Materials = serde_json::from_reader(file)
        .expect("Unable to parse configuration file 'materials.json'");

    // let materials = Materials::new(MaterialColor::new(BACKGROUND), vec![
    //     Material::new(0, "Sand", "Key1", MaterialColor::new(0x2EB086), vec![
    //         MovementRule::new(Movement::Move(IndexShift::new(1, 0)), vec![IndexShift::new(1, 0)], vec![]),
    //         MovementRule::new(Movement::Move(IndexShift::new(1, -1)), vec![IndexShift::new(1, -1)], vec![]),
    //         MovementRule::new(Movement::Move(IndexShift::new(1, 1)), vec![IndexShift::new(1, 1)], vec![]),
    //     ]),
    //     Material::new(1, "Water", "Key2", MaterialColor::new(0x2666CF), vec![
    //         MovementRule::new(Movement::Move(IndexShift::new(1, 0)), vec![IndexShift::new(1, 0)], vec![]),
    //         MovementRule::new(Movement::Move(IndexShift::new(1, -1)), vec![IndexShift::new(1, -1)], vec![]),
    //         MovementRule::new(Movement::Move(IndexShift::new(1, 1)), vec![IndexShift::new(1, 1)], vec![]),
    //         MovementRule::new(Movement::Move(IndexShift::new(0, -1)), vec![IndexShift::new(0, -1)], vec![]),
    //         MovementRule::new(Movement::Move(IndexShift::new(0, 1)), vec![IndexShift::new(0, 1)], vec![]),
    //     ]),
    //     Material::new(2, "Wall", "Key3", MaterialColor::new(0xB8405E), vec![]),
    //     Material::new(3, "Plague", "Key4", MaterialColor::new(0x8A39E1), vec![
    //         MovementRule::new(Movement::Move(IndexShift::new(1, 0)), vec![IndexShift::new(1, 0)], vec![]),
    //         MovementRule::new(Movement::Move(IndexShift::new(1, -1)), vec![IndexShift::new(1, -1)], vec![]),
    //         MovementRule::new(Movement::Move(IndexShift::new(1, 1)), vec![IndexShift::new(1, 1)], vec![]),
    //         MovementRule::new(Movement::Copy(IndexShift::new(0, -1)), vec![], vec![IndexShift::new(0, -1)]),
    //         MovementRule::new(Movement::Copy(IndexShift::new(0, 1)), vec![], vec![IndexShift::new(0, 1)]),
    //         MovementRule::new(Movement::Copy(IndexShift::new(-1, 0)), vec![], vec![IndexShift::new(-1, 0)]),
    //         MovementRule::new(Movement::Copy(IndexShift::new(-1, -1)), vec![], vec![IndexShift::new(-1, -1)]),
    //         MovementRule::new(Movement::Copy(IndexShift::new(-1, 1)), vec![], vec![IndexShift::new(-1, 1)]),
    //         MovementRule::new(Movement::Copy(IndexShift::new(1, 0)), vec![], vec![IndexShift::new(1, 0)]),
    //         MovementRule::new(Movement::Copy(IndexShift::new(1, -1)), vec![], vec![IndexShift::new(1, -1)]),
    //         MovementRule::new(Movement::Copy(IndexShift::new(1, 1)), vec![], vec![IndexShift::new(1, 1)]),
    //     ]),
    //     Material::new(4, "Acidic sand", "Key5", MaterialColor::new(0xF76E11), vec![
    //         MovementRule::new(Movement::Move(IndexShift::new(1, 0)), vec![IndexShift::new(1, 0)], vec![]),
    //         MovementRule::new(Movement::Move(IndexShift::new(1, -1)), vec![IndexShift::new(1, -1)], vec![]),
    //         MovementRule::new(Movement::Move(IndexShift::new(1, 1)), vec![IndexShift::new(1, 1)], vec![]),
    //         MovementRule::new(Movement::Move(IndexShift::new(1, 0)), vec![], vec![IndexShift::new(1, 0)]),
    //         MovementRule::new(Movement::Move(IndexShift::new(1, -1)), vec![], vec![IndexShift::new(1, -1)]),
    //         MovementRule::new(Movement::Move(IndexShift::new(1, 1)), vec![], vec![IndexShift::new(1, 1)]),
    //         MovementRule::new(Movement::Move(IndexShift::new(-1, 0)), vec![], vec![IndexShift::new(-1, 0)]),
    //         MovementRule::new(Movement::Move(IndexShift::new(-1, -1)), vec![], vec![IndexShift::new(-1, -1)]),
    //         MovementRule::new(Movement::Move(IndexShift::new(-1, 1)), vec![], vec![IndexShift::new(-1, 1)]),
    //         MovementRule::new(Movement::Move(IndexShift::new(0, -1)), vec![], vec![IndexShift::new(0, -1)]),
    //         MovementRule::new(Movement::Move(IndexShift::new(0, 1)), vec![], vec![IndexShift::new(0, 1)]),
    //     ]),
    // ]);

    // println!("{}", serde_json::to_string_pretty(&materials).unwrap());

    let grid_bounds = app.window_rect().pad_top(TOP_BAR_PAD);

    Model {
        fps: 0.0,
        brush: Brush::new(MOUSE_SPAWN_RADIUS),
        game: GameView::new(GRID_WIDTH_CELLS, GRID_HEIGHT_CELLS, grid_bounds, materials),
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(button) => {
            model.brush.active = true;
            if button == MouseButton::Right {
                model.brush.fill = None;
            }
        }
        MouseReleased(_) => {
            model.brush.active = false;
        }
        KeyPressed(key) => {
            let materials = model.game.materials();
            if let Some(material_id) = materials.get_id_by_key(resolve_key_name(key).as_str()) {
                model.brush.fill = Some(material_id);
            }
        }
        _ => {}
    }
}

fn resolve_key_name(key: VirtualKeyCode) -> String {
    format!("{:?}", key)
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.fps = 1000.0 / update.since_last.as_millis() as f64;

    model.game.prepare();

    if model.brush.active {
        model.game.spawn(app.mouse.position(), model.brush.radius, model.brush.fill);
    }

    model.game.step();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.game.display(&draw);

    if frame.nth() % REDRAW_FPS_FRAMES == 0 {
        draw_fps(app, &draw, model);
    }

    draw_material_info(app, &draw, model);

    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    fps: f64,
    brush: Brush,
    game: GameView,
}

struct Brush {
    active: bool,
    radius: u8,
    fill: Option<MaterialId>,
}

impl Brush {
    pub fn new(radius: u8) -> Self {
        Self { active: false, radius, fill: Some(MaterialId(0)) }
    }
}

fn draw_material_info(app: &App, draw: &Draw, model: &Model) {
    let material_color = model.game.materials().get_color(model.brush.fill);
    let material_name = model.game.materials().get_name(model.brush.fill);

    let material_color_xy = app.window_rect().top_right().add(vec2(-TOP_BAR_INFO_WIDTH / 2.0, -TOP_BAR_PAD / 2.0));
    let material_color_wh = vec2(TOP_BAR_INFO_WIDTH, TOP_BAR_PAD);
    let material_name_xy = vec2(0.0, app.window_rect().top() - TOP_BAR_PAD / 2.0);
    let material_name_wh = vec2(app.window_rect().w() - 2.0 * TOP_BAR_INFO_WIDTH, TOP_BAR_PAD);
    let font_size = 22;

    draw.rect()
        .xy(material_color_xy)
        .wh(material_color_wh)
        .color(Into::<Rgb8>::into(material_color));

    draw.rect()
        .xy(material_name_xy)
        .wh(material_name_wh)
        .color(WHITE);

    draw.text(&material_name)
        .xy(material_name_xy)
        .wh(material_name_wh)
        .font_size(font_size)
        .color(BLACK);
}

fn draw_fps(app: &App, draw: &Draw, model: &Model) {
    let text_xy = app.window_rect().top_left().add(vec2(TOP_BAR_INFO_WIDTH / 2.0, -TOP_BAR_PAD / 2.0));
    let text_wh = vec2(TOP_BAR_INFO_WIDTH, TOP_BAR_PAD);
    let font_size = 18;
    let text = format!("{:.2} FPS", model.fps);

    draw.rect()
        .color(BLACK)
        .xy(text_xy)
        .wh(text_wh);

    draw.text(&text)
        .color(WHITE)
        .xy(text_xy)
        .wh(text_wh)
        .font_size(font_size);
}
