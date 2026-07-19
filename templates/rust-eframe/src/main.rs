// Prevent winit opening console before opening GUI in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() {
    _ = eframe::run_native(
        "Eframe App",
        eframe::NativeOptions {
            wgpu_options: eframe::WgpuConfiguration {
                surface: eframe::SurfaceConfig {
                    present_mode: eframe::wgpu::PresentMode::Immediate,
                    desired_maximum_frame_latency: Some(1),
                },
                ..Default::default()
            },
            viewport: egui::ViewportBuilder::default()
                .with_min_inner_size([400.0, 400.0])
                .with_inner_size([800.0, 600.0])
                .with_transparent(true)
                .with_decorations(false)
                .with_resizable(true),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(App::new(cc))))
    );
}

#[derive(Debug, Default)]
struct App {}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
                custom_window_frame(ui, "App", |ui| {
                    draw_resize_handle(ui);
                });
            });
        handle_borderless_resizing(ui);
    }
}

fn custom_window_frame(
    ui: &mut egui::Ui,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    let panel_frame = egui::Frame::new()
        .fill(ui.style().visuals.window_fill());

    panel_frame.show(ui, |ui| {
        let app_rect = ui.max_rect();
        ui.expand_to_include_rect(app_rect);

        let title_bar_height = 40.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };

        title_bar_ui(ui, title_bar_rect, title);

        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        };

        let mut content_ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect));
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: egui::Rect, title: &str) {
    let painter = ui.painter();

    let title_bar_response = ui.interact(
        title_bar_rect, 
        egui::Id::new("title_bar"), 
        egui::Sense::click_and_drag()
    );

    painter.text(
        title_bar_rect.center(),
        egui::Align2::CENTER_CENTER,
        title,
        eframe::egui::FontId::proportional(20.0),
        egui::Color32::from_rgb(180, 180, 180),
    );

    if title_bar_response.double_clicked() {
        let is_max = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.send_viewport_cmd(egui::ViewportCommand::Maximized(!is_max));
    }

    if title_bar_response.drag_started_by(egui::PointerButton::Primary) {
        ui.send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }

    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(title_bar_rect)
            .layout(egui::Layout::right_to_left(egui::Align::Center)), 
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            close_maximize_minimize(ui);
        }
    );
}

fn close_maximize_minimize(ui: &mut egui::Ui) {
    let btn_height = 14.0;

    let close_hover_color_fg = egui::Color32::from_rgb(220, 235, 255);
    let close_hover_color_bg = egui::Color32::from_rgb(220, 38, 38);
    let icon_color = egui::Color32::from_rgb(180, 180, 180);
    let hover_color_bg = egui::Color32::from_rgb(240, 240, 240);
    let hover_color_fg = egui::Color32::from_rgb(0, 0, 0);

    let close_response = render_window_control_button_with_drawn_icon(
        ui,
        icon_color,
        close_hover_color_bg,
        close_hover_color_fg,
        btn_height,
        draw_close_icon
    );

    if close_response.clicked() {
        ui.send_viewport_cmd(egui::ViewportCommand::Close);
    }


    if ui.input(|i| i.viewport().maximized.unwrap_or(false)) {
        let restore_response = render_window_control_button_with_drawn_icon(
            ui,
            icon_color,
            hover_color_bg,
            hover_color_fg,
            btn_height,
            draw_restore_icon
        );

        if restore_response.clicked() {
            ui.send_viewport_cmd(egui::ViewportCommand::Maximized(false));
        }
    } else {
        let maximise_response = render_window_control_button_with_drawn_icon(
            ui,
            icon_color,
            hover_color_bg,
            hover_color_fg,
            btn_height,
            draw_maximize_icon
        );

        if maximise_response.clicked() {
            ui.send_viewport_cmd(egui::ViewportCommand::Maximized(true));
        }
    }

    let minimise_response = render_window_control_button_with_drawn_icon(
        ui,
        icon_color,
        hover_color_bg,
        hover_color_fg,
        btn_height,
        draw_minimize_icon
    );

    if minimise_response.clicked() {
        ui.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
    }

}

fn draw_resize_handle(ui: &mut egui::Ui) {
    let size = egui::vec2(40.0, 40.0);
    let rect = egui::Rect::from_min_size(
        ui.max_rect().max - size,
        size,
    );

    let response = ui.allocate_rect(rect, egui::Sense::drag());

    if response.hovered() || response.dragged() {
        ui.set_cursor_icon(egui::CursorIcon::ResizeSouthEast);
    }

    let painter = ui.painter();
    
    for i in 1..3 {
        let offset = i as f32 * 4.0;
        let p1 = rect.max - egui::vec2(offset + 8.0, 8.0);
        let p2 = rect.max - egui::vec2(8.0, offset + 8.0);
        painter.line_segment([p1, p2], (1.0, egui::Color32::from_rgb(180, 180, 180)));
    }

    if response.dragged() {
        ui.send_viewport_cmd(egui::ViewportCommand::BeginResize(egui::ResizeDirection::SouthEast));
    }
}

fn draw_close_icon(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let center = rect.center();
    let size = rect.width().min(rect.height()) * 0.6;
    let half_size = size / 2.0;

    let stroke = egui::Stroke::new(1.0, color);
    painter.line_segment(
        [
            center + egui::Vec2::new(-half_size, -half_size),
            center + egui::Vec2::new(half_size, half_size),
        ],
        stroke,
    );
    painter.line_segment(
        [
            center + egui::Vec2::new(half_size, -half_size),
            center + egui::Vec2::new(-half_size, half_size),
        ],
        stroke,
    );
}

fn draw_maximize_icon(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let center = rect.center();
    let size = rect.width().min(rect.height()) * 0.75;
    let stroke = egui::Stroke::new(1.0, color);
    let square_rect = egui::Rect::from_center_size(center, egui::Vec2::new(size, size));
    painter.rect_stroke(square_rect, 1.0, stroke, egui::StrokeKind::Inside);
}

fn draw_restore_icon(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let button_size = rect.width().min(rect.height());
    let square_size = button_size * 0.85;
    let icon_rect = egui::Rect::from_center_size(rect.center(), egui::Vec2::new(square_size, square_size));

    let center = icon_rect.center();
    let half_size = square_size / 2.0;

    let stroke = egui::Stroke::new(1.0, color);

    let main_square_size = square_size * 0.7;
    let main_square_center = center + egui::Vec2::new(-half_size * 0.2, 0.0);
    let main_square = egui::Rect::from_center_size(
        main_square_center,
        egui::Vec2::new(main_square_size, main_square_size),
    );
    painter.rect_stroke(main_square, 0.0, stroke, egui::StrokeKind::Inside);

    let spacing = half_size * 0.12;

    let horizontal_start = center + egui::Vec2::new(-half_size * 0.3, -half_size + spacing);
    let horizontal_end = center + egui::Vec2::new(half_size - spacing, -half_size + spacing);

    let vertical_start = center + egui::Vec2::new(half_size - spacing, -half_size + spacing);
    let vertical_end = center + egui::Vec2::new(half_size - spacing, half_size * 0.2);

    painter.line_segment([horizontal_start, horizontal_end], stroke);
    painter.line_segment([vertical_start, vertical_end], stroke);
}

fn draw_minimize_icon(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let center = rect.center();
    let size = rect.width().min(rect.height()) * 0.8;
    let half_size = size / 2.0;

    let stroke = egui::Stroke::new(1.0, color);
    painter.line_segment(
        [
            center + egui::Vec2::new(-half_size, 0.0),
            center + egui::Vec2::new(half_size, 0.0),
        ],
        stroke,
    );
}

pub fn render_window_control_button_with_drawn_icon(
    ui: &mut egui::Ui,
    icon_color: egui::Color32,
    hover_color_bg: egui::Color32,
    hover_color_fg: egui::Color32,
    icon_size: f32,
    paint: impl FnOnce(&egui::Painter, egui::Rect, egui::Color32),
) -> egui::Response {
    let desired_size = egui::Vec2::new(46.0, 40.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, hover_color_bg);
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    let icon_rect = egui::Rect::from_center_size(rect.center(), egui::Vec2::new(icon_size, icon_size));

    let final_icon_color = if response.hovered() {
        hover_color_fg
    } else {
        icon_color
    };

    paint(ui.painter(), icon_rect, final_icon_color);

    response
}

fn handle_borderless_resizing(ui: &mut egui::Ui) {
    // 1. Define how wide your invisible hit-test border should be (e.g., 6 pixels)
    let border_thickness = 6.0;
    let window_rect = ui.viewport_rect();
    let mouse_pos = ui.input(|i| i.pointer.interact_pos());

    if let Some(pos) = mouse_pos {
        // 2. Check if mouse is near any window edges
        let left = pos.x < window_rect.min.x + border_thickness;
        let right = pos.x > window_rect.max.x - border_thickness;
        let top = pos.y < window_rect.min.y + border_thickness;
        let bottom = pos.y > window_rect.max.y - border_thickness;

        // 3. Determine the correct winit resize direction
        use egui::viewport::ResizeDirection;
        let direction = match (top, bottom, left, right) {
            (true, false, true, false) => Some(ResizeDirection::NorthWest),
            (true, false, false, true) => Some(ResizeDirection::NorthEast),
            (false, true, true, false) => Some(ResizeDirection::SouthWest),
            (false, true, false, true) => Some(ResizeDirection::SouthEast),
            (true, false, false, false) => Some(ResizeDirection::North),
            (false, true, false, false) => Some(ResizeDirection::South),
            (false, false, true, false) => Some(ResizeDirection::West),
            (false, false, false, true) => Some(ResizeDirection::East),
            _ => None,
        };

        if let Some(dir) = direction {
            // Change cursor icon to show the user they can resize
            ui.set_cursor_icon(match dir {
                ResizeDirection::North | ResizeDirection::South => egui::CursorIcon::ResizeVertical,
                ResizeDirection::East | ResizeDirection::West => egui::CursorIcon::ResizeHorizontal,
                ResizeDirection::NorthWest => egui::CursorIcon::ResizeNorthWest,
                ResizeDirection::SouthEast => egui::CursorIcon::ResizeSouthEast,
                ResizeDirection::NorthEast => egui::CursorIcon::ResizeNorthEast,
                ResizeDirection::SouthWest => egui::CursorIcon::ResizeSouthWest,
            });

            // If the user clicks down on the border, tell the OS to take over the drag-resize
            if ui.input(|i| i.pointer.any_pressed()) {
                ui.send_viewport_cmd(egui::ViewportCommand::BeginResize(dir));
            }
        }
    }
}
