use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: Cow<'static, str>,
    pub title: Cow<'static, str>,
    pub text: Cow<'static, str>,
    pub icon: Cow<'static, str>,
    pub bg: Cow<'static, str>,
    pub border: Cow<'static, str>,
    pub ring: Cow<'static, str>,
}

pub const DEFAULT: Theme = Theme {
    name: Cow::Borrowed("default"),
    title: Cow::Borrowed("#2f80ed"),
    icon: Cow::Borrowed("#4c71f2"),
    text: Cow::Borrowed("#434d58"),
    bg: Cow::Borrowed("#fffefe"),
    border: Cow::Borrowed("#e4e2e2"),
    ring: Cow::Borrowed("#2f80ed"),
};
