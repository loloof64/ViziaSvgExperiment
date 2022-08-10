use vizia::prelude::*;

#[derive(Lens)]
struct AppData {}

pub enum AppEvent {
    Hello,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Hello => {
                println!("Hello !");
            }
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData{}.build(cx);

        Button::new(
            cx,
            |cx| cx.emit(AppEvent::Hello),
            |cx| Label::new(cx, "Say hello"),
        );
    })
    .title("Vizia svg experiment")
    .inner_size((400, 100))
    .run()
}
