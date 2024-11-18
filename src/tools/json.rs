use leptos::*;
use rust_i18n::t;
use serde_json::{self, Value};
use serde_yaml;
use crate::utils::{copy_to_clipboard, I18nState};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FormatType {
    Standard,
    SingleQuote,
    NoQuote,
    Yaml,
}

#[component]
pub fn JsonTool(i18n: Rc<I18nState>) -> impl IntoView {
    let current_locale = i18n.locale();
    
    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(String::new());
    let (error, set_error) = create_signal(String::new());
    let (format_type, set_format_type) = create_signal(FormatType::Standard);

    let format = move |_| {
        if input.get().is_empty() {
            set_output.set(String::new());
            set_error.set(String::new());
            return;
        }

        match serde_json::from_str::<Value>(&input.get()) {
            Ok(json) => {
                match format_type.get() {
                    FormatType::Standard => {
                        match serde_json::to_string_pretty(&json) {
                            Ok(formatted) => {
                                set_output.set(formatted);
                                set_error.set(String::new());
                            }
                            Err(e) => set_error.set(e.to_string()),
                        }
                    }
                    FormatType::SingleQuote => {
                        match serde_json::to_string_pretty(&json) {
                            Ok(formatted) => {
                                let single_quoted = formatted.replace('\"', "'");
                                set_output.set(single_quoted);
                                set_error.set(String::new());
                            }
                            Err(e) => set_error.set(e.to_string()),
                        }
                    }
                    FormatType::NoQuote => {
                        match serde_json::to_string_pretty(&json) {
                            Ok(formatted) => {
                                let no_quote = formatted
                                    .lines()
                                    .map(|line| {
                                        if line.contains(':') {
                                            let parts: Vec<&str> = line.splitn(2, ':').collect();
                                            let key = parts[0].trim().trim_matches('"');
                                            let value = parts[1].trim();
                                            format!("{}: {}", key, value)
                                        } else {
                                            line.to_string()
                                        }
                                    })
                                    .collect::<Vec<String>>()
                                    .join("\n");
                                set_output.set(no_quote);
                                set_error.set(String::new());
                            }
                            Err(e) => set_error.set(e.to_string()),
                        }
                    }
                    FormatType::Yaml => {
                        match serde_yaml::to_string(&json) {
                            Ok(yaml) => {
                                set_output.set(yaml);
                                set_error.set(String::new());
                            }
                            Err(e) => set_error.set(e.to_string()),
                        }
                    }
                }
            }
            Err(e) => set_error.set(e.to_string()),
        }
    };

    let minify = move |_| {
        if input.get().is_empty() {
            set_output.set(String::new());
            set_error.set(String::new());
            return;
        }

        match serde_json::from_str::<Value>(&input.get()) {
            Ok(json) => {
                match serde_json::to_string(&json) {
                    Ok(minified) => {
                        set_output.set(minified);
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
                        on:click=format
                    >
                        {move || {
                            let _ = current_locale.get();
                            t!("tools.common.format")
                        }}
                    </button>
                    <button
                        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                        on:click=minify
                    >
                        {move || {
                            let _ = current_locale.get();
                            t!("tools.common.minify")
                        }}
                    </button>
                </div>

                <div class="flex flex-wrap gap-2">
                    <button
                        class=move || format!(
                            "px-3 py-2 text-sm font-medium rounded-md {} {}",
                            if matches!(format_type.get(), FormatType::Standard) {
                                "bg-blue-100 text-blue-700"
                            } else {
                                "text-gray-700 hover:bg-gray-100"
                            },
                            "focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                        )
                        on:click=move |_| set_format_type.set(FormatType::Standard)
                    >
                        {move || {
                            let _ = current_locale.get();
                            t!("tools.json.format_standard")
                        }}
                    </button>
                    <button
                        class=move || format!(
                            "px-3 py-2 text-sm font-medium rounded-md {} {}",
                            if matches!(format_type.get(), FormatType::SingleQuote) {
                                "bg-blue-100 text-blue-700"
                            } else {
                                "text-gray-700 hover:bg-gray-100"
                            },
                            "focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                        )
                        on:click=move |_| set_format_type.set(FormatType::SingleQuote)
                    >
                        {move || {
                            let _ = current_locale.get();
                            t!("tools.json.format_single_quote")
                        }}
                    </button>
                    <button
                        class=move || format!(
                            "px-3 py-2 text-sm font-medium rounded-md {} {}",
                            if matches!(format_type.get(), FormatType::NoQuote) {
                                "bg-blue-100 text-blue-700"
                            } else {
                                "text-gray-700 hover:bg-gray-100"
                            },
                            "focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                        )
                        on:click=move |_| set_format_type.set(FormatType::NoQuote)
                    >
                        {move || {
                            let _ = current_locale.get();
                            t!("tools.json.format_no_quote")
                        }}
                    </button>
                    <button
                        class=move || format!(
                            "px-3 py-2 text-sm font-medium rounded-md {} {}",
                            if matches!(format_type.get(), FormatType::Yaml) {
                                "bg-blue-100 text-blue-700"
                            } else {
                                "text-gray-700 hover:bg-gray-100"
                            },
                            "focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                        )
                        on:click=move |_| set_format_type.set(FormatType::Yaml)
                    >
                        {move || {
                            let _ = current_locale.get();
                            t!("tools.json.format_yaml")
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
                            format!("{}: {}", t!("tools.json.invalid_json"), error.get())
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