use vizia::prelude::*;
use vizia::vg::{Color, Paint, Path};
mod svg;

#[derive(Lens)]
struct AppData {}

pub enum AppEvent {
    Hello,
}

struct SvgZone {
    svg_paths: Vec<(Path, Option<Paint>, Option<Paint>)>,
    width: u32,
    height: u32,
    dpi_factor: f32,
    bg_color: Color,
}

#[derive(Default)]
struct SvgZoneBuilder {
    svg_paths: Vec<(Path, Option<Paint>, Option<Paint>)>,
    width: u32,
    height: u32,
    dpi_factor: f32,
    bg_color: Color,
}

impl SvgZoneBuilder {
    fn new() -> Self {
        Self::default()
    }

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

    fn set_background_color(&mut self, color: Color) {
        self.bg_color = color;
    }
}

impl SvgZone {
    pub fn new_from_builder<'a>(cx: &'a mut Context, builder: &SvgZoneBuilder) -> Handle<'a, Self> {
        Self {
            width: builder.width,
            height: builder.height,
            dpi_factor: builder.dpi_factor,
            bg_color: builder.bg_color,
            svg_paths: builder.svg_paths.clone(),
        }
        .build(cx, |_| {})
        .focusable(false)
    }
}

impl View for SvgZone {
    fn draw(&self, _cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        canvas.set_size(self.width, self.height, self.dpi_factor);
        canvas.clear_rect(0, 0, self.width, self.height, self.bg_color);

        let mut path = self.svg_paths.clone();
        for (path, fill, stroke) in &mut path {
            if let Some(fill) = fill {
                fill.set_anti_alias(true);
                canvas.fill_path(path, *fill);
            }

            if let Some(stroke) = stroke {
                stroke.set_anti_alias(true);
                canvas.stroke_path(path, *stroke);
            }
        }

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

        let mut svg_zone_builder = SvgZoneBuilder::new();
        svg_zone_builder.resize(200, 200);
        svg_zone_builder.set_background_color(Color::rgb(200, 120, 64));

        let tree = usvg::Tree::from_data(
            include_bytes!("Chess_nlt45.svg"),
            &usvg::Options::default().to_ref(),
        )
        .expect("Failed to get data from svg image.");

        let paths = svg::render_svg(tree);
        svg_zone_builder.set_svg_paths(paths);

        SvgZone::new_from_builder(cx, &svg_zone_builder);
    })
    .title("Vizia svg experiment")
    .inner_size((400, 100))
    .run()
}
