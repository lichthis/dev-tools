pub fn copy_to_clipboard(text: &str) {
    if let Some(window) = web_sys::window() {
        let navigator = window.navigator();
        let clipboard = navigator.clipboard();
        let _ = clipboard.write_text(text);
    }
}

use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Storage;

#[derive(Clone, Debug)]
pub struct I18nState {
    locale: RwSignal<String>,
}

impl I18nState {
    pub fn new() -> Self {
        let locale = create_rw_signal(get_saved_locale().unwrap_or_else(|| "en".to_string()));
        rust_i18n::set_locale(&locale.get_untracked());
        Self { locale }
    }

    pub fn locale(&self) -> RwSignal<String> {
        self.locale
    }
}

thread_local! {
    static I18N_STATE: RefCell<Option<Rc<I18nState>>> = RefCell::new(None);
}

pub fn get_saved_locale() -> Option<String> {
    window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage: Storage| storage.get_item("locale").ok())
        .flatten()
}