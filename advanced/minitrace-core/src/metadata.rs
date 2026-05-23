pub struct Metadata {
    pub name: Option<&'static str>,
    pub fields: &'static [&'static str],
    pub file: &'static str,
    pub line: u64,
    pub column: u64,
    pub module_path: &'static str,
    pub kind: Kind,
}

pub enum Kind {
    Span,
    Event,
}
