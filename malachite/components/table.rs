pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: Vec<&str>, rows: Vec<Vec<&str>>) -> Self {
        Self {
            headers: headers.into_iter().map(|s| s.to_string()).collect(),
            rows: rows.into_iter().map(|row| row.into_iter().map(|s| s.to_string()).collect()).collect(),
        }
    }

    pub fn render(&self) -> String {
        let headers_html: String = self.headers.iter()
            .map(|header| format!("<th>{}</th>", header))
            .collect();

        let rows_html: String = self.rows.iter()
            .map(|row| {
                let row_html: String = row.iter()
                    .map(|cell| format!("<td>{}</td>", cell))
                    .collect();
                format!("<tr>{}</tr>", row_html)
            })
            .collect();

        format!(
            "<table class='table'><thead><tr>{}</tr></thead><tbody>{}</tbody></table>",
            headers_html, rows_html
        )
    }
}
