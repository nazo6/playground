#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::*;
use tokio::sync::mpsc::UnboundedReceiver;
use ui::init;

struct Root {
    message: SharedString,
    _recv_task: Task<()>,
}

impl Root {
    fn new(cx: &mut Context<'_, Self>, mut mes_receiver: UnboundedReceiver<String>) -> Self {
        let _recv_task = cx.spawn(|this, mut cx| async move {
            loop {
                if let Some(message) = mes_receiver.recv().await {
                    println!("Received in UI: {}", message);
                    this.update(&mut cx, |this, cx| {
                        this.message = SharedString::new(message);
                        cx.notify();
                    })
                    .unwrap();
                };
            }
        });

        Self {
            message: SharedString::new(""),
            _recv_task,
        }
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        println!("Render");

        div()
            .bg(white())
            .child(format!("message: {}", self.message))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let chan = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        let mut i = 0;
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            i += 1;
            chan.0.send(format!("from tokio task: {}", i)).unwrap();
            println!("Sending from tokio task: {}", i);
        }
    });

    let app = Application::new();

    app.run(move |app| {
        init(app);

        app.open_window(WindowOptions::default(), |_, app| {
            app.new(|cx| Root::new(cx, chan.1))
        })
        .unwrap();
    });

    Ok(())
}
