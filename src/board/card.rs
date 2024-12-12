use super::status::Status;

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    column: String,
    status: Status,
    title: String,
    date: Option<String>,
    time: Option<String>,
}

impl Card {
    pub fn to_string(&self) -> String {
        let status = self.status.to_string();
        let title = &self.title;
        let date = self
            .date
            .as_ref()
            .map(|d| format!(" @{{{}}}", d))
            .unwrap_or_default();
        let time = self
            .time
            .as_ref()
            .map(|t| format!(" @@{{{}}}", t))
            .unwrap_or_default();
        format!("- [{status}] {title}{date}{time}")
    }
    pub fn move_to(&mut self, to: &String) {
        self.column = to.clone();
    }
    pub fn column(&self) -> &String {
        &self.column
    }
    pub fn status(&self) -> &Status {
        &self.status
    }
    pub fn title(&self) -> &String {
        &self.title
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CardBuilder {
    column: Option<String>,
    status: Option<Status>,
    title: Option<String>,
    date: Option<String>,
    time: Option<String>,
}

impl CardBuilder {
    pub fn new() -> Self {
        CardBuilder::default()
    }

    pub fn column(mut self, column: &str) -> Self {
        self.column = Some(column.to_string());
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn _date(mut self, date: &str) -> Self {
        self.date = Some(date.to_string());
        self
    }

    pub fn _time(mut self, time: &str) -> Self {
        self.time = Some(time.to_string());
        self
    }

    pub fn build(self) -> Result<Card, String> {
        let column = self.column.ok_or("Column is required")?;
        let status = self.status.unwrap_or_default();
        let title = self.title.ok_or("Title is required")?;
        Ok(Card {
            column,
            status,
            title,
            date: self.date,
            time: self.time,
        })
    }
}
