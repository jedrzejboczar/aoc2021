use std::fmt::Display;

pub trait ColumnDisplay {
    type Item;

    fn columns<'a>(&'a self) -> &'a [Self::Item];

    fn column_display<'a>(&'a self, prefix: String, separator: String) -> Columns<'a, Self::Item> {
        Columns {
            columns: self.columns(),
            prefix,
            separator,
        }
    }
}

pub struct Columns<'a, T> {
    columns: &'a [T],
    prefix: String,
    separator: String,
}

impl<'a, T> Display for Columns<'a, T>
    where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_groups: Vec<Vec<_>> = self.columns.iter()
            .map(|b| b.to_string()
                .split('\n')
                .map(str::to_string)
                .collect())
            .collect();
        for i in 0.. {
            let lines: Vec<_> = line_groups.iter()
                .filter_map(|lines| lines.get(i).map(|s| s.as_str()))
                .collect();
            if lines.is_empty() {
                break;
            }
            if i != 0 {
                writeln!(f, "")?;
            }
            write!(f, "{}{}", self.prefix, lines.join(&self.separator))?;
        }
        Ok(())
    }
}

impl<T: Display> ColumnDisplay for &[T] {
    type Item = T;

    fn columns<'a>(&'a self) -> &'a [Self::Item] {
        self
    }
}

impl<T: Display> ColumnDisplay for &mut [T] {
    type Item = T;

    fn columns<'a>(&'a self) -> &'a [Self::Item] {
        self
    }
}
