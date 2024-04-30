use leptos::*;
use std::time::Duration;

#[derive(Clone)]
pub enum ToastType {
    Success,
    Error,
}

#[derive(Clone)]
pub struct ToastMessage {
    pub message: String,
    pub toast_type: ToastType,
    pub visible: bool,
}

#[component]
pub fn Toast() -> impl IntoView {
    let (toast, set_toast) = create_signal(ToastMessage {
        message: String::new(),
        toast_type: ToastType::Success,
        visible: false,
    });

    provide_context(set_toast);

    let base_class="fixed bottom-10 left-1/2 transform -translate-x-1/2 text-white px-4 py-2 rounded shadow-lg transition-all duration-300";

    let toast_class = move || {
        let t = toast();
        let background = match t.toast_type {
            ToastType::Success => "bg-green-600",
            ToastType::Error => "bg-red-600",
        };

        let opacity = if t.visible == true {
            "opacity-1".to_string()
        } else {
            "opacity-0".to_string()
        };

        format!("{} {} {}", base_class, background, opacity)
    };

    create_effect(move |_| {
        let t = toast();
        if t.visible {
            set_timeout(
                move || {
                    set_toast.update(|msg| {
                        msg.visible = false;
                    })
                },
                Duration::new(4, 0),
            );
        }
    });

    view! {
      <div id="toast" class=toast_class>
        {move || toast().message}
      </div>
    }
}
