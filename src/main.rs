use vizia::prelude::*;
use vizia::vg::{Color, Paint, Path};
mod svg;

#[derive(Lens)]
struct AppData {}

pub enum AppEvent {
    Hello,
}

#[derive(Default)]
struct SvgZone {
    svg_paths: Vec<(Path, Option<Paint>, Option<Paint>)>,
    width: u32,
    height: u32,
    dpi_factor: f32,
    bg_color: Color,
}

impl SvgZone {
    fn set_svg_paths(&mut self, svg_paths: Vec<(Path, Option<Paint>, Option<Paint>)>) {
        self.svg_paths = svg_paths;
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn set_dpi_factor(mut self, factor: f32) {
        self.dpi_factor = factor;
    }

    fn set_bg_color(&mut self, color: Color) {
        self.bg_color = color;
    }
}

impl View for SvgZone {
    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        canvas.set_size(self.width, self.height, self.dpi_factor);
        canvas.clear_rect(0, 0, self.width, self.height, self.bg_color);

        canvas.save();
        canvas.translate(200.0, 200.0);

        for (path, fill, stroke) in &mut self.svg_paths {
            if let Some(fill) = fill {
                fill.set_anti_alias(true);
                canvas.fill_path(path, *fill);
            }

            if let Some(stroke) = stroke {
                stroke.set_anti_alias(true);
                canvas.stroke_path(path, *stroke);
            }
        }

        canvas.restore();

        canvas.flush();
    }
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Hello => {
                println!("Hello");
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData {}.build(cx);

        let mut svg_zone = SvgZone::default();
        svg_zone.resize(200, 200);

        let tree = usvg::Tree::from_data(
            include_bytes!("Chess_nlt45.svg"),
            &usvg::Options::default().to_ref(),
        )
        .expect("Failed to get data from svg image.");

        let paths = svg::render_svg(tree);
        svg_zone.set_svg_paths(paths);
    })
    .title("Vizia svg experiment")
    .inner_size((400, 100))
    .run()
}
