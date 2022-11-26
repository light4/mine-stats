use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: Cow<'static, str>,
    pub title: Cow<'static, str>,
    pub text: Cow<'static, str>,
    pub icon: Cow<'static, str>,
    pub bg: Cow<'static, str>,
    pub border: Option<Cow<'static, str>>,
    pub ring: Option<Cow<'static, str>>,
}

pub const DEFAULT: Theme = Theme {
    name: Cow::Borrowed("default"),
    title: Cow::Borrowed("#2f80ed"),
    icon: Cow::Borrowed("#4c71f2"),
    text: Cow::Borrowed("#434d58"),
    bg: Cow::Borrowed("#fffefe"),
    border: Some(Cow::Borrowed("#e4e2e2")),
    ring: Some(Cow::Borrowed("#2f80ed")),
};

pub const ONEDARK: Theme = Theme {
    name: Cow::Borrowed("onedark"),
    title: Cow::Borrowed("#e4bf7a"),
    icon: Cow::Borrowed("#8eb573"),
    text: Cow::Borrowed("#df6d74"),
    bg: Cow::Borrowed("#282c34"),
    border: None,
    ring: None,
};

impl Default for Theme {
    fn default() -> Self {
        DEFAULT
    }
}
