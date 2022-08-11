mod widgets;
use vizia::prelude::*;
use widgets::SvgZone;

pub enum AppEvent {
    Hello,
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
    .resizable(false)
    .run()
}