pub fn make_table<T: std::fmt::Display>(data: &[Vec<T>]) -> tabled::Table {
    let mut builder = tabled::builder::Builder::new();
    builder.push_record((0..data[0].len()).map(|v| v.to_string()));
    for row in data {
        builder.push_record(row.iter().map(|v| v.to_string()));
    }
    let mut table = builder.index().build();
    table.with(tabled::settings::Style::rounded());
    table
}

pub fn make_horizontal<T: std::fmt::Display>(data: &[T]) -> tabled::Table {
    let mut builder = tabled::builder::Builder::new();
    builder.push_record((0..data.len()).map(|v| v.to_string()));
    builder.push_record(data.iter().map(|v| v.to_string()));
    let mut table = builder.index().build();
    table.with(tabled::settings::Style::rounded());
    table
}

pub fn make_table_builder<T: std::fmt::Display>(width: usize) -> TableBuilder {
    TableBuilder::new(width)
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

    pub fn push_record<T: std::fmt::Display>(&mut self, row: &[T]) -> &mut Self {
        self.builder.push_record(row.iter().map(|v| v.to_string()));
        self
    }

    pub fn finish(self) -> tabled::Table {
        let mut table = self.builder.index().build();
        table.with(tabled::settings::Style::rounded());
        table
    }
}
