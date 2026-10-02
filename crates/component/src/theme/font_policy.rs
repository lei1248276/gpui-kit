//! Optional application-owned fonts, installed before theme initialization.
use gpui::{App, Global, SharedString};
use std::{rc::Rc, time::Duration};

/// Already verified families owned by this App. Theme reloads preserve them.
/// Without this policy the existing default font checks remain unchanged.
#[derive(Clone)]
#[non_exhaustive]
pub struct ThemeFonts {
    pub font_family: SharedString,
    pub mono_font_family: SharedString,
}

impl ThemeFonts {
    /// Choose families already verified by the application.
    pub fn new(
        font_family: impl Into<SharedString>,
        mono_font_family: impl Into<SharedString>,
    ) -> Self {
        Self {
            font_family: font_family.into(),
            mono_font_family: mono_font_family.into(),
        }
    }
}

impl Global for ThemeFonts {}

pub(super) fn apply(cx: &mut App) -> bool {
    let Some(fonts) = cx.try_global::<ThemeFonts>().cloned() else {
        return false;
    };
    let theme = super::Theme::global_mut(cx);
    theme.font_family = fonts.font_family;
    theme.mono_font_family = fonts.mono_font_family;
    true
}

/// Explicit diagnostics only; the callback is owned by the observing App.
pub struct FontEnumerationObserver(Rc<dyn Fn(Duration)>);

impl FontEnumerationObserver {
    /// Observe actual enumeration without changing font selection.
    pub fn new(observer: Rc<dyn Fn(Duration)>) -> Self {
        Self(observer)
    }
}
impl Global for FontEnumerationObserver {}

pub(super) fn record_enumeration(cx: &App, elapsed: Duration) {
    if let Some(observer) = cx.try_global::<FontEnumerationObserver>() {
        (observer.0)(elapsed);
    }
}
