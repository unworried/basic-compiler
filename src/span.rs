#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn combine(mut spans: Vec<Span>) -> Span {
        spans.sort_by(|a, b| a.start.cmp(&b.start));

        let start = spans.first().unwrap().start; // TODO: Deal with errors
        let end = spans.last().unwrap().end;

        // Do i need string lit?

        Span::new(start, end)
    }

    pub fn length(&self) -> usize {
        (self.end - self.start) + 1
    }
}