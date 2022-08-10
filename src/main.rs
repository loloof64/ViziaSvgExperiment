use usvg::{LineCap, LineJoin, Size, Transform};
use vizia::vg::{Paint, Path};
use vizia::{prelude::*, vg};

pub enum AppEvent {
    Hello,
}

struct SvgZone {
    svg_paths: Vec<(Path, Option<Paint>, Option<Paint>, Transform)>,
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

        Self {
            svg_paths: render_svg(tree),
            svg_size,
        }
        .build(cx, |_| {})
        .focusable(false)
    }
}

impl View for SvgZone {
    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        canvas.save();
        canvas.reset();

        canvas.translate(bounds.x, bounds.y);

        // Scale with DPI
        let scale = cx.style.dpi_factor as f32;
        canvas.scale(scale, scale);

        let scalex = bounds.width() / self.svg_size.width() as f32;
        let scaley = bounds.height() / self.svg_size.height() as f32;

        canvas.scale(scalex, scaley);

        let mut path = self.svg_paths.clone();
        for (path, fill, stroke, transform) in &mut path {
            canvas.save();
            canvas.set_transform(
                transform.a as f32,
                transform.b as f32,
                transform.c as f32,
                transform.d as f32,
                transform.e as f32,
                transform.f as f32,
            );
            if let Some(fill) = fill {
                fill.set_anti_alias(true);
                canvas.fill_path(path, *fill);
            }

            if let Some(stroke) = stroke {
                stroke.set_anti_alias(true);
                canvas.stroke_path(path, *stroke);
            }
            canvas.restore();
        }

        canvas.flush();

        canvas.restore();
    }
}

fn main() {
    Application::new(|cx| {
        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                SvgZone::new(cx, include_bytes!("resources/Chess_plt45.svg")).size(Pixels(100.0));
                SvgZone::new(cx, include_bytes!("resources/Chess_nlt45.svg")).size(Pixels(100.0));
                SvgZone::new(cx, include_bytes!("resources/Chess_blt45.svg")).size(Pixels(100.0));
                SvgZone::new(cx, include_bytes!("resources/Chess_rlt45.svg")).size(Pixels(100.0));
                SvgZone::new(cx, include_bytes!("resources/Chess_qlt45.svg")).size(Pixels(100.0));
                SvgZone::new(cx, include_bytes!("resources/Chess_klt45.svg")).size(Pixels(100.0));
            });

            HStack::new(cx, |cx| {
                SvgZone::new(cx, include_bytes!("resources/Chess_pdt45.svg")).size(Pixels(100.0));
                SvgZone::new(cx, include_bytes!("resources/Chess_ndt45.svg")).size(Pixels(100.0));
                SvgZone::new(cx, include_bytes!("resources/Chess_bdt45.svg")).size(Pixels(100.0));
                SvgZone::new(cx, include_bytes!("resources/Chess_rdt45.svg")).size(Pixels(100.0));
                SvgZone::new(cx, include_bytes!("resources/Chess_qdt45.svg")).size(Pixels(100.0));
                SvgZone::new(cx, include_bytes!("resources/Chess_kdt45.svg")).size(Pixels(100.0));
            });
            SvgZone::new(cx, include_bytes!("resources/Ghostscript_Tiger.svg")).size(Pixels(300.0));
        });
    })
    .title("SVG")
    .inner_size((800, 800))
    .run()
}

pub fn render_svg(svg: usvg::Tree) -> Vec<(Path, Option<Paint>, Option<Paint>, Transform)> {
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
                        PathSegment::CurveTo {
                            x1,
                            y1,
                            x2,
                            y2,
                            x,
                            y,
                        } => path.bezier_to(
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

                        let line_cap: vg::LineCap = match stroke.linecap {
                            LineCap::Butt => vg::LineCap::Butt,
                            LineCap::Square => vg::LineCap::Square,
                            LineCap::Round => vg::LineCap::Round,
                        };

                        let line_join: vg::LineJoin = match stroke.linejoin {
                            LineJoin::Bevel => vg::LineJoin::Bevel,
                            LineJoin::Miter => vg::LineJoin::Miter,
                            LineJoin::Round => vg::LineJoin::Round,
                        };

                        stroke_paint.set_line_cap(line_cap);
                        stroke_paint.set_line_join(line_join);

                        stroke_paint.set_miter_limit(stroke.miterlimit.value() as f32);

                        stroke_paint
                    })
                });

                let transform = svg_path.transform;

                paths.push((path, fill, stroke, transform))
            }
            _ => (),
        }
    }

    paths
}
