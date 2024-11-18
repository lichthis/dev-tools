use leptos::*;
use rust_i18n::t;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use crate::utils::{copy_to_clipboard, I18nState};
use std::rc::Rc;

#[component]
pub fn Base64Tool(i18n: Rc<I18nState>) -> impl IntoView {
    let current_locale = i18n.locale();
    
    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());

    let encode = move |_| {
        let input_text = input.get();
        let encoded = STANDARD.encode(input_text.as_bytes());
        set_output.set(encoded);
        set_error.set(String::new());
    };

    let decode = move |_| {
        let input_text = input.get();
        match STANDARD.decode(input_text.as_bytes()) {
            Ok(decoded) => {
                match String::from_utf8(decoded) {
                    Ok(text) => {
                        set_output.set(text);
                        set_error.set(String::new());
                    }
                    Err(e) => set_error.set(e.to_string()),
                }
            }
            Err(e) => set_error.set(e.to_string()),
        }
    };

    view! {
        <div class="p-6">
            <div class="space-y-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                        {move || {
                            let _ = current_locale.get();
                            t!("tools.common.input_placeholder")
                        }}
                    </label>
                    <textarea
                        class="w-full h-48 p-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                        placeholder=move || {
                            let _ = current_locale.get();
                            t!("tools.common.input_placeholder")
                        }
                        on:input=move |ev| set_input.set(event_target_value(&ev))
                        prop:value=input
                    ></textarea>
                </div>

                <div class="flex flex-wrap gap-2">
                    <button
                        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                        on:click=encode
                    >
                        {move || {
                            let _ = current_locale.get();
                            t!("tools.base64.encode")
                        }}
                    </button>
                    <button
                        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                        on:click=decode
                    >
                        {move || {
                            let _ = current_locale.get();
                            t!("tools.base64.decode")
                        }}
                    </button>
                </div>

                <Show
                    when=move || !error.get().is_empty()
                    fallback=|| view! { }
                >
                    <div class="p-4 bg-red-50 border border-red-200 rounded-lg text-red-700 whitespace-pre-wrap">
                        {move || {
                            let _ = current_locale.get();
                            format!("{}: {}", t!("tools.base64.invalid_base64"), error.get())
                        }}
                    </div>
                </Show>

                <Show
                    when=move || !output.get().is_empty()
                    fallback=|| view! { }
                >
                    <div>
                        <div class="flex justify-between items-center mb-2">
                            <label class="block text-sm font-medium text-gray-700">
                                {move || {
                                    let _ = current_locale.get();
                                    t!("tools.common.output_label")
                                }}
                            </label>
                            <button
                                class="inline-flex items-center px-3 py-1 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                                on:click=move |_| {
                                    let _ = copy_to_clipboard(&output.get());
                                }
                            >
                                {move || {
                                    let _ = current_locale.get();
                                    t!("tools.common.copy")
                                }}
                            </button>
                        </div>
                        <pre class="w-full h-48 p-2 bg-gray-50 border border-gray-200 rounded-md overflow-auto whitespace-pre-wrap">
                            {output}
                        </pre>
                    </div>
                </Show>
            </div>
        </div>
    }
}