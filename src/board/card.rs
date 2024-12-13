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
    pub fn move_to(&mut self, to: &str) {
        self.column = to.to_string();
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

    pub fn date(&self) -> Option<String> {
        self.date.clone()
    }

    pub fn time(&self) -> Option<String> {
        self.time.clone()
    }

    pub fn mut_rename(mut self, new_name: &str) -> Self {
        self.title = new_name.to_string();
        self
    }

    pub fn rename(&self, new_name: &str) -> Self {
        Card {
            title: new_name.to_string(),
            ..self.clone()
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

        write!(f, "- [{status}] {title}{date}{time}")
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

    pub fn date(mut self, date: &str) -> Self {
        self.date = Some(date.to_string());
        self
    }

    pub fn time(mut self, time: &str) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_builder() {
        let card = CardBuilder::new()
            .column("Column")
            .status(Status::Done)
            .title("Title")
            .build()
            .unwrap();
        assert_eq!(card.column(), "Column");
        assert_eq!(card.status(), &Status::Done);
        assert_eq!(card.title(), "Title");
    }

    #[test]
    fn test_card_to_string() {
        let card = CardBuilder::new()
            .column("Column")
            .status(Status::Done)
            .title("Title")
            .build()
            .unwrap();
        assert_eq!(card.to_string(), "- [x] Title");
    }

    #[test]
    fn test_card_move_to() {
        let mut card = CardBuilder::new()
            .column("Column")
            .status(Status::Done)
            .title("Title")
            .build()
            .unwrap();
        card.move_to(&"New Column".to_string());
        assert_eq!(card.column(), "New Column");
    }

    #[test]
    fn test_card_rename() {
        let card = CardBuilder::new()
            .column("Column")
            .status(Status::Done)
            .title("Title")
            .build()
            .unwrap();
        let renamed = card.clone().rename("New Title");
        assert_eq!(renamed.title(), "New Title");
        assert_eq!(card.title(), "Title");
    }

    #[test]
    fn test_card_mut_rename() {
        let card = CardBuilder::new()
            .column("Column")
            .status(Status::Done)
            .title("Title")
            .build()
            .unwrap();
        let renamed = card.mut_rename("New Title");
        assert_eq!(renamed.title(), "New Title");
    }
}
