use leptos::*;
use rust_i18n::t;
use urlencoding;
use crate::utils::{copy_to_clipboard, I18nState};
use std::rc::Rc;

#[component]
pub fn UrlTool(i18n: Rc<I18nState>) -> impl IntoView {
    let current_locale = i18n.locale();
    
    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());
    let (mode, set_mode) = create_signal(true); // true for encode, false for decode

    let encode = move || {
        let input_text = input.get();
        let result = urlencoding::encode(&input_text);
        set_output.set(result.into_owned());
        set_error.set(String::new());
    };

    let decode = move || {
        let input_text = input.get();
        match urlencoding::decode(&input_text) {
            Ok(result) => {
                set_output.set(result.into_owned());
                set_error.set(String::new());
            }
            Err(e) => {
                set_error.set(e.to_string());
            }
        }
    };

    view! {
        <div class="space-y-4 p-6">
            <div class="flex justify-between items-center">
                <h2 class="text-lg font-semibold text-gray-900">
                    {move || {
                        let _ = current_locale.get();
                        t!("tools.url.title")
                    }}
                </h2>
                <div class="flex items-center space-x-4">
                    <label class="inline-flex items-center">
                        <input
                            type="radio"
                            class="form-radio"
                            prop:checked=move || mode.get()
                            on:change=move |_| set_mode.set(true)
                        />
                        <span class="ml-2">
                            {move || {
                                let _ = current_locale.get();
                                t!("tools.url.encode")
                            }}
                        </span>
                    </label>
                    <label class="inline-flex items-center">
                        <input
                            type="radio"
                            class="form-radio"
                            prop:checked=move || !mode.get()
                            on:change=move |_| set_mode.set(false)
                        />
                        <span class="ml-2">
                            {move || {
                                let _ = current_locale.get();
                                t!("tools.url.decode")
                            }}
                        </span>
                    </label>
                </div>
            </div>
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
                    on:click=move |_| {
                        if mode.get() {
                            encode();
                        } else {
                            decode();
                        }
                    }
                >
                    {move || {
                        let _ = current_locale.get();
                        if mode.get() {
                            t!("tools.url.encode")
                        } else {
                            t!("tools.url.decode")
                        }
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
                        format!("{}: {}", t!("tools.url.invalid_url"), error.get())
                    }}
                </div>
            </Show>
            <div class="mt-4">
                <label class="block text-sm font-medium text-gray-700">
                    {move || {
                        let _ = current_locale.get();
                        t!("tools.common.output_label")
                    }}
                </label>
                <div class="mt-1 relative">
                    <textarea
                        class="w-full h-48 p-2 border border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                        rows="4"
                        prop:value=output
                        readonly
                    ></textarea>
                    <button
                        class="absolute top-2 right-2 inline-flex items-center px-3 py-1 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
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
            </div>
            <div class="text-red-500 text-sm">
                {move || {
                    let _ = current_locale.get();
                    error.get()
                }}
            </div>
        </div>
    }
}