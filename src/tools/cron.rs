use leptos::*;
use crate::utils::{copy_to_clipboard, I18nState};
use chrono::{DateTime, Local};
use cron::Schedule;
use std::str::FromStr;
use rust_i18n::t;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutputFormat {
    Default,     // 2024-01-01 12:00:00
    ISO8601,     // 2024-01-01T12:00:00+08:00
    Unix,        // 1704085200
}

fn format_time(time: DateTime<Local>, format: OutputFormat) -> String {
    match format {
        OutputFormat::Default => time.format("%Y-%m-%d %H:%M:%S").to_string(),
        OutputFormat::ISO8601 => time.to_rfc3339(),
        OutputFormat::Unix => time.timestamp().to_string(),
    }
}

fn parse_cron(expr: &str, include_seconds: bool) -> Result<Schedule, String> {
    // 如果不包含秒，在表达式前添加 "0 "
    let expr = if !include_seconds && !expr.trim().is_empty() {
        format!("0 {}", expr)
    } else {
        expr.to_string()
    };
    
    Schedule::from_str(&expr).map_err(|e| e.to_string())
}

fn get_next_occurrences(schedule: &Schedule, count: usize, format: OutputFormat) -> Vec<String> {
    schedule
        .upcoming(Local)
        .take(count)
        .map(|time| format_time(time, format))
        .collect()
}

fn describe_field(field: &str) -> String {
    match field {
        "*" => t!("tools.cron.field.every"),
        "?" => t!("tools.cron.field.any"),
        field if field.contains('/') => {
            let parts: Vec<&str> = field.split('/').collect();
            if parts.len() == 2 {
                format!("{} {}", t!("tools.cron.field.every"), parts[1])
            } else {
                field.to_string()
            }
        }
        field if field.contains('-') => {
            let parts: Vec<&str> = field.split('-').collect();
            if parts.len() == 2 {
                format!("{} {} {}", t!("tools.cron.field.from"), parts[0], parts[1])
            } else {
                field.to_string()
            }
        }
        field if field.contains(',') => {
            format!("{} [{}]", t!("tools.cron.field.specific"), field)
        }
        _ => field.to_string()
    }
}

fn describe_cron(expr: &str, include_seconds: bool) -> String {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    let (seconds, rest) = if include_seconds {
        (Some(parts[0]), &parts[1..])
    } else {
        (None, &parts[..])
    };

    if rest.len() != 5 {
        return t!("tools.cron.invalid_expression").to_string();
    }

    let mut desc = Vec::new();
    
    // 添加秒描述
    if let Some(s) = seconds {
        desc.push(format!("{}: {}", t!("tools.cron.field.second"), describe_field(s)));
    }

    // 添加其他字段描述
    let fields = [
        t!("tools.cron.field.minute"),
        t!("tools.cron.field.hour"),
        t!("tools.cron.field.day"),
        t!("tools.cron.field.month"),
        t!("tools.cron.field.week")
    ];
    
    for (i, &field) in rest.iter().enumerate() {
        desc.push(format!("{}: {}", fields[i], describe_field(field)));
    }

    desc.join("\n")
}

#[component]
pub fn CronTool(i18n: Rc<I18nState>) -> impl IntoView {
    let current_locale = i18n.locale();

    let (input, set_input) = create_signal("* * * * *".to_string());
    let (description, set_description) = create_signal(String::new());
    let (next_times, set_next_times) = create_signal(Vec::new());
    let (error, set_error) = create_signal(String::new());
    let (include_seconds, set_include_seconds) = create_signal(false);
    let (output_format, set_output_format) = create_signal(OutputFormat::Default);

    let parse_cron_expr = move || {
        let input_text = input.get();
        
        // 清空之前的结果
        set_description.set(String::new());
        set_next_times.set(Vec::new());
        set_error.set(String::new());

        if input_text.trim().is_empty() {
            set_error.set(format!("{}: {}", t!("tools.cron.invalid_input"), t!("tools.cron.input_placeholder")));
            return;
        }

        // 解析并验证 cron 表达式
        match parse_cron(&input_text, include_seconds.get()) {
            Ok(schedule) => {
                // 设置描述
                set_description.set(describe_cron(&input_text, include_seconds.get()));
                
                // 获取接下来的执行时间
                let times = get_next_occurrences(&schedule, 5, output_format.get());
                set_next_times.set(times);
            }
            Err(e) => {
                set_error.set(format!("{}: {}", t!("tools.cron.invalid_cron"), e));
            }
        }
    };

    let handle_click = move |_: web_sys::MouseEvent| {
        parse_cron_expr();
    };

    // 组件加载时自动解析默认表达式
    create_effect(move |_| {
        parse_cron_expr();
    });

    view! {
        <div class="flex flex-col gap-4 p-4">
            <h2 class="text-2xl font-bold text-gray-800">{move || {
                let _ = current_locale.get();
                t!("tools.cron.title")
            }}</h2>
            <div class="flex flex-col gap-4">
                <div class="flex items-center gap-4">
                    <input
                        type="text"
                        class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        placeholder=move || {
                            let _ = current_locale.get();
                            t!("tools.cron.input_placeholder")
                        }
                        prop:value=move || input.get()
                        on:input=move |ev| {
                            set_input.set(event_target_value(&ev));
                        }
                    />
                    <div class="flex items-center">
                        <input
                            type="checkbox"
                            class="h-4 w-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                            prop:checked=move || include_seconds.get()
                            on:change=move |ev| {
                                set_include_seconds.set(event_target_checked(&ev));
                            }
                        />
                        <label class="ml-2 text-sm text-gray-700">
                            {move || {
                                let _ = current_locale.get();
                                t!("tools.cron.include_seconds")
                            }}
                        </label>
                    </div>
                    <select
                        class="px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            match value.as_str() {
                                "default" => set_output_format.set(OutputFormat::Default),
                                "iso8601" => set_output_format.set(OutputFormat::ISO8601),
                                "unix" => set_output_format.set(OutputFormat::Unix),
                                _ => (),
                            }
                        }
                    >
                        <option value="default">{move || {
                            let _ = current_locale.get();
                            t!("tools.cron.output_format_default")
                        }}</option>
                        <option value="iso8601">{move || {
                            let _ = current_locale.get();
                            t!("tools.cron.output_format_iso8601")
                        }}</option>
                        <option value="unix">{move || {
                            let _ = current_locale.get();
                            t!("tools.cron.output_format_unix")
                        }}</option>
                    </select>
                    <button 
                        class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        on:click=handle_click
                    >
                        {move || {
                            let _ = current_locale.get();
                            t!("tools.cron.parse")
                        }}
                    </button>
                </div>
            </div>
            
            // 错误信息
            <Show
                when=move || !error.get().is_empty()
                fallback=|| view! { }
            >
                <div class="p-4 bg-red-50 border border-red-200 rounded-lg text-red-700">
                    {move || error.get()}
                </div>
            </Show>

            // 描述
            <Show
                when=move || !description.get().is_empty()
                fallback=|| view! { }
            >
                <div class="flex flex-col gap-2">
                    <h3 class="text-lg font-semibold text-gray-800">{move || {
                        let _ = current_locale.get();
                        t!("tools.cron.description")
                    }}</h3>
                    <pre class="p-4 bg-gray-50 border border-gray-200 rounded-lg font-mono text-sm whitespace-pre-wrap">
                        {move || description.get()}
                    </pre>
                </div>
            </Show>

            // 下次执行时间
            <Show
                when=move || !next_times.get().is_empty()
                fallback=|| view! { }
            >
                <div class="flex flex-col gap-2">
                    <h3 class="text-lg font-semibold text-gray-800">{move || {
                        let _ = current_locale.get();
                        t!("tools.cron.next_runs")
                    }}</h3>
                    <div class="relative">
                        <pre class="p-4 bg-gray-50 border border-gray-200 rounded-lg font-mono text-sm whitespace-pre-wrap">
                            {move || next_times.get().join("\n")}
                        </pre>
                        <button 
                            class="absolute top-2 right-2 p-2 text-gray-500 hover:text-gray-700 focus:outline-none"
                            on:click=move |_| copy_to_clipboard(&next_times.get().join("\n"))
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                <path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z" />
                                <path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z" />
                            </svg>
                        </button>
                    </div>
                </div>
            </Show>
        </div>
    }
}