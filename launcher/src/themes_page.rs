//! Settings' accent colour and Home background rows (`horizon_launcher::themes`).
use super::*;
use horizon_launcher::themes;
use std::path::PathBuf;

impl Launcher {
    /// The theme Settings ask for, under the window's own light or dark.
    pub(super) fn settings_theme(&self, window: &Window) -> Theme {
        Theme::for_appearance(
            self.settings.appearance,
            self.settings.accent,
            window.appearance(),
        )
    }

    /// The picture behind Home, if the player keeps one.
    pub(super) fn home_background(&self) -> Option<PathBuf> {
        themes::background(&backend::data_dir(), self.settings.background.as_deref())
    }

    fn choose_background(&mut self, cx: &mut Context<Self>) {
        let paths = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some(tr("Choose").to_string().into()),
        });
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(mut paths))) = paths.await else {
                return;
            };
            let Some(path) = paths.pop() else {
                return;
            };
            let _ = this.update(cx, |this, cx| {
                this.take_background(&path);
                cx.notify();
            });
        })
        .detach();
    }

    fn take_background(&mut self, path: &std::path::Path) {
        match themes::keep_background(&backend::data_dir(), path) {
            Ok(name) => {
                self.settings.background = Some(name);
                self.settings.save();
                self.background_error = None;
            }
            Err(e) => self.background_error = Some(e),
        }
    }

    pub(super) fn home_background_row(&self, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let custom = self.home_background().is_some();
        let mut actions = div().flex().gap(px(4.)).child(
            button(
                "background-choose",
                "Choose a picture…",
                Kind::Secondary,
                &t,
            )
            .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                this.choose_background(cx);
                cx.notify();
            })),
        );
        if custom {
            actions = actions.child(
                button("background-reset", "Landscape", Kind::Ghost, &t).on_click(cx.listener(
                    |this, _: &ClickEvent, _, cx| {
                        this.settings.background = None;
                        this.settings.save();
                        cx.notify();
                    },
                )),
            );
        }
        let said = if custom {
            tr("Your picture, behind Home, cropped to fill it.")
        } else {
            tr("Horizon's landscape. Pick a PNG, JPEG or WebP picture of your own.")
        };
        let detail = div().flex().flex_col().child(said.to_string()).children(
            self.background_error
                .clone()
                .map(|e| div().text_color(t.danger).child(e)),
        );
        index_row(
            0,
            tr("Home background"),
            Some(detail.into_any_element()),
            Some(actions.into_any_element()),
            &t,
        )
    }
}
