#[macro_export]
macro_rules! vtable {
    ($(($name:expr, $items:expr)),+ $(,)?) => {
        {
            let mut builder = tabled_utils::VerticalTableBuilder::new();
            builder$(.column($name, $items))+;
            let table = builder.finish();
            eprintln!("{table}");
        }
    };
}

#[derive(Default)]
pub struct VerticalTableBuilder {
    columns: Vec<Column>,
}

struct Column {
    label: String,
    items: Vec<String>,
}

impl VerticalTableBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn column<S, T, I>(&mut self, label: S, items: I) -> &mut Self
    where
        S: std::borrow::Borrow<str>,
        T: std::fmt::Debug,
        I: std::iter::IntoIterator<Item = T>,
    {
        self.columns.push(Column {
            label: label.borrow().to_owned(),
            items: items
                .into_iter()
                .map(|item| format!("{:?}", item))
                .collect(),
        });
        self
    }
    pub fn finish(self) -> tabled::Table {
        let mut builder = tabled::builder::Builder::new();
        builder.push_record(self.columns.iter().map(|column| column.label.clone()));
        for i in 0..self
            .columns
            .iter()
            .map(|column| column.items.len())
            .max()
            .unwrap()
        {
            builder.push_record(
                self.columns
                    .iter()
                    .map(|column| column.items.get(i).cloned().unwrap_or_default()),
            );
        }
        let mut table = builder.index().build();
        table.with(tabled::settings::Style::rounded());
        table
    }
}
