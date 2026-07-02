pub fn make_table<T: std::fmt::Debug, R>(data: &[R]) -> tabled::Table
where
    R: std::borrow::Borrow<[T]>,
{
    let mut builder = tabled::builder::Builder::new();
    builder.push_record((0..data[0].borrow().len()).map(|v| v.to_string()));
    for row in data {
        builder.push_record(row.borrow().iter().map(|v| format!("{:?}", v)));
    }
    let mut table = builder.index().build();
    table.with(tabled::settings::Style::rounded());
    table
}

pub fn make_horizontal<T: std::fmt::Debug>(data: &[T]) -> tabled::Table {
    let mut builder = tabled::builder::Builder::new();
    builder.push_record((0..data.len()).map(|v| v.to_string()));
    builder.push_record(data.iter().map(|v| format!("{:?}", v)));
    let mut table = builder.index().build();
    table.with(tabled::settings::Style::rounded());
    table
}

pub struct TableBuilder {
    builder: tabled::builder::Builder,
}
impl TableBuilder {
    pub fn new(width: usize) -> Self {
        let mut builder = tabled::builder::Builder::new();
        builder.push_record((0..width).map(|i| i.to_string()));
        Self { builder }
    }

    pub fn push_record<T: std::fmt::Debug>(&mut self, row: &[T]) -> &mut Self {
        self.builder
            .push_record(row.iter().map(|v| format!("{:?}", v)));
        self
    }

    pub fn finish(self) -> tabled::Table {
        let mut table = self.builder.index().build();
        table.with(tabled::settings::Style::rounded());
        table
    }
}
