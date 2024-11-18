use leptos::*;
use rust_i18n::t;
use web_sys::window;
use std::rc::Rc;

rust_i18n::i18n!("locales");

fn get_stored_locale() -> String {
    window()
        .and_then(|window| window.local_storage().ok()?)
        .and_then(|storage| storage.get_item("locale").ok()?)
        .unwrap_or_else(|| String::from("en"))
}

fn save_locale(locale: &str) {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("locale", locale);
        }
    }
}

fn main() {
    // Native entry point
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    
    mount_to_body(App);
}

mod tools {
    pub mod base64;
    pub mod cron;
    pub mod json;
    pub mod url;
}

use tools::{
    base64::Base64Tool,
    cron::CronTool,
    json::JsonTool,
    url::UrlTool,
};

mod utils;

use crate::utils::I18nState;

#[derive(Clone, PartialEq, Debug)]
enum Tool {
    UrlEncoder,
    Base64,
    JsonFormatter,
    Cron,
}

#[component]
pub fn App() -> impl IntoView {
    let (current_tool, set_current_tool) = create_signal(Tool::UrlEncoder);
    let i18n = store_value(Rc::new(I18nState::new()));
    
    // 初始化时设置语言
    let initial_locale = get_stored_locale();
    rust_i18n::set_locale(&initial_locale);
    
    // 创建响应式信号
    let (locale_trigger, set_locale_trigger) = create_signal(String::new());
    
    // 创建响应式的语言 memo
    let current_locale = create_memo(move |_| {
        locale_trigger.get();  // 添加依赖
        i18n.get_value().locale().get()
    });

    let switch_locale = move |locale: &str| {
        save_locale(locale);
        i18n.get_value().locale().set(locale.to_string());
        rust_i18n::set_locale(locale);
        set_locale_trigger.set(locale.to_string());  // 触发更新
    };

    view! {
        <div class="min-h-screen bg-gray-50">
            <nav class="bg-white shadow-sm">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between h-16">
                        <div class="flex-shrink-0 flex items-center">
                            <span class="text-xl font-bold text-gray-900">
                                {move || {
                                    let _ = current_locale.get();  // 使用 memo
                                    t!("app.title")
                                }}
                            </span>
                        </div>
                        <div class="flex space-x-4">
                            <button 
                                class=move || {
                                    if current_tool.get() == Tool::UrlEncoder {
                                        "px-3 py-2 text-sm font-medium text-blue-600 border-b-2 border-blue-600"
                                    } else {
                                        "px-3 py-2 text-sm font-medium text-gray-500 hover:text-blue-600"
                                    }
                                }
                                on:click=move |_| set_current_tool.set(Tool::UrlEncoder)
                            >
                                {move || {
                                    let _ = current_locale.get();  // 使用 memo
                                    t!("nav.url_encoder")
                                }}
                            </button>
                            <button 
                                class=move || {
                                    if current_tool.get() == Tool::Base64 {
                                        "px-3 py-2 text-sm font-medium text-blue-600 border-b-2 border-blue-600"
                                    } else {
                                        "px-3 py-2 text-sm font-medium text-gray-500 hover:text-blue-600"
                                    }
                                }
                                on:click=move |_| set_current_tool.set(Tool::Base64)
                            >
                                {move || {
                                    let _ = current_locale.get();  // 使用 memo
                                    t!("nav.base64")
                                }}
                            </button>
                            <button 
                                class=move || {
                                    if current_tool.get() == Tool::JsonFormatter {
                                        "px-3 py-2 text-sm font-medium text-blue-600 border-b-2 border-blue-600"
                                    } else {
                                        "px-3 py-2 text-sm font-medium text-gray-500 hover:text-blue-600"
                                    }
                                }
                                on:click=move |_| set_current_tool.set(Tool::JsonFormatter)
                            >
                                {move || {
                                    let _ = current_locale.get();  // 使用 memo
                                    t!("nav.json_formatter")
                                }}
                            </button>
                            <button 
                                class=move || {
                                    if current_tool.get() == Tool::Cron {
                                        "px-3 py-2 text-sm font-medium text-blue-600 border-b-2 border-blue-600"
                                    } else {
                                        "px-3 py-2 text-sm font-medium text-gray-500 hover:text-blue-600"
                                    }
                                }
                                on:click=move |_| set_current_tool.set(Tool::Cron)
                            >
                                {move || {
                                    let _ = current_locale.get();  // 使用 memo
                                    t!("nav.cron")
                                }}
                            </button>
                        </div>
                        <div class="ml-4 flex items-center">
                            <button
                                class=move || {
                                    if current_locale.get() == "en" {  // 使用 memo
                                        "px-3 py-2 text-sm font-medium text-blue-600 border-b-2 border-blue-600"
                                    } else {
                                        "px-3 py-2 text-sm font-medium text-gray-500 hover:text-blue-600"
                                    }
                                }
                                on:click=move |_| switch_locale("en")
                            >
                                "EN"
                            </button>
                            <button
                                class=move || {
                                    if current_locale.get() == "zh" {  // 使用 memo
                                        "px-3 py-2 text-sm font-medium text-blue-600 border-b-2 border-blue-600"
                                    } else {
                                        "px-3 py-2 text-sm font-medium text-gray-500 hover:text-blue-600"
                                    }
                                }
                                on:click=move |_| switch_locale("zh")
                            >
                                "中"
                            </button>
                        </div>
                    </div>
                </div>
            </nav>
            <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                <div class="px-4 py-6 sm:px-0">
                    <div class="bg-white rounded-lg shadow">
                        {move || {
                            let _ = current_locale.get();  // 使用 memo
                            let _ = current_tool.get();   
                            let i18n_value = i18n.get_value();  // 获取实际的 Rc<I18nState>
                            match current_tool.get() {
                                Tool::UrlEncoder => view! { <UrlTool i18n=i18n_value.clone()/> },
                                Tool::Base64 => view! { <Base64Tool i18n=i18n_value.clone()/> },
                                Tool::JsonFormatter => view! { <JsonTool i18n=i18n_value.clone()/> },
                                Tool::Cron => view! { <CronTool i18n=i18n_value.clone()/> },
                            }
                        }}
                    </div>
                </div>
            </main>
        </div>
    }
}