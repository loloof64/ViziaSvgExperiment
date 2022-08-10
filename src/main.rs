use usvg::{Size};
use vizia::vg::{Paint, Path};
use vizia::{prelude::*, vg};

pub enum AppEvent {
    Hello,
}

struct SvgZone {
    svg_paths: Vec<(Path, Option<Paint>, Option<Paint>)>,
    svg_size: Size,
}

impl SvgZone {
    pub fn new<'a>(cx: &'a mut Context, data: &[u8]) -> Handle<'a, Self> {
        let tree = usvg::Tree::from_data(
            data,
            &usvg::Options {
                dpi: 1.0,
                default_size: Size::new(900.0, 900.0).unwrap(),
                ..Default::default()
            }
            .to_ref(),
        )
        .expect("Failed to get data from svg image.");

        let svg_size = tree.svg_node().size;

        Self { svg_paths: render_svg(tree), svg_size }
            .build(cx, |_| {})
            .focusable(false)
    }
}

impl View for SvgZone {
    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        canvas.save();
        canvas.reset();

        let scalex = bounds.width() / self.svg_size.width() as f32;
        let scaley = bounds.height() / self.svg_size.height() as f32;

        canvas.scale(scalex, scaley);

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

        canvas.restore();

        // TEMP: Draw an outline of the view
        let mut border_path = Path::new();
        border_path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
        canvas.stroke_path(&mut border_path, Paint::color(vg::Color::black()));
    }
}

fn main() {
    Application::new(|cx| {
        SvgZone::new(cx, include_bytes!("resources/Chess_nlt45.svg")).size(Pixels(200.0));
    })
    .title("SVG")
    .inner_size((450, 450))
    .run()
}

pub fn render_svg(svg: usvg::Tree) -> Vec<(Path, Option<Paint>, Option<Paint>)> {
    use usvg::NodeKind;
    use usvg::PathSegment;

    let mut paths = Vec::new();

    for node in svg.root().descendants() {
        match &*node.borrow() {
            NodeKind::Path(svg_path) => {
                let mut path = Path::new();

                for command in svg_path.data.iter() {
                    match command {
                        PathSegment::MoveTo { x, y } => path.move_to(*x as f32, *y as f32),
                        PathSegment::LineTo { x, y } => path.line_to(*x as f32, *y as f32),
                        PathSegment::CurveTo { x1, y1, x2, y2, x, y } => path.bezier_to(
                            *x1 as f32, *y1 as f32, *x2 as f32, *y2 as f32, *x as f32, *y as f32,
                        ),
                        PathSegment::ClosePath => path.close(),
                    }
                }

                let to_femto_color = |usvg_paint: &usvg::Paint| match usvg_paint {
                    usvg::Paint::Color(usvg::Color { red, green, blue }) => {
                        Some(vg::Color::rgb(*red, *green, *blue))
                    }
                    _ => None,
                };

                let fill = svg_path
                    .fill
                    .as_ref()
                    .and_then(|fill| to_femto_color(&fill.paint))
                    .map(Paint::color);

                let stroke = svg_path.stroke.as_ref().and_then(|stroke| {
                    to_femto_color(&stroke.paint).map(|paint| {
                        let mut stroke_paint = Paint::color(paint);
                        stroke_paint.set_line_width(stroke.width.value() as f32);
                        stroke_paint
                    })
                });

                paths.push((path, fill, stroke))
            }
            _ => (),
        }
    }

    paths
}
