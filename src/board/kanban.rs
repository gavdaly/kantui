use super::card::{Card, CardBuilder};
use super::status::Status;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Kanban {
    columns: Vec<String>,
    cards: Vec<Card>,
}
impl Kanban {
    pub fn new(columns: &[&str]) -> Self {
        let columns = columns.iter().map(|c| c.to_string()).collect();
        Kanban {
            columns,
            cards: Vec::new(),
        }
    }

    pub fn add_column(&mut self, name: String) -> Result<(), String> {
        self.columns.push(name);

        Ok(())
    }

    pub fn add_card(&mut self, card: &Card) -> Result<(), String> {
        self.has_column(card.column())?;
        self.cards.push(card.clone());
        return Ok(());
    }

    fn has_column(&self, column: &String) -> Result<(), String> {
        match self.columns.contains(column) {
            true => Ok(()),
            false => Err("Column does not exist".to_string()),
        }
    }

    pub fn move_card(&mut self, to: &String, card: Card) -> Result<(), String> {
        self.has_column(to)?;

        self.cards = self
            .cards
            .clone()
            .into_iter()
            .map(|mut c| {
                if c == card {
                    c.move_to(to);
                }
                c
            })
            .collect();
        Ok(())
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        let mut column_name = String::new();
        let mut kanban = Kanban::default();
        // ignore frontmatter with `---`
        // Find the first ## and use that as the column name
        // Each `- [ ] Title` `-` is the card the `[ ]` is the status and the `Title` is the title
        // `[ ]` is incomplete and `[x]` is done
        // The date and time are optional and are in the format `YYYY-MM-DD` and `HH:MM` respectively They show as `@{2027-12-31}` and `@@{23:59}`
        // At the end there is another block with %% ignore them
        for line in input.lines() {
            if line.starts_with("## ") {
                match line.split_at(3) {
                    ("## ", column) => {
                        kanban.add_column(column.to_string())?;
                        column_name = column.to_string();
                    }
                    _ => {
                        return Err("Column name is empty".to_string());
                    }
                };
            }
            if line.starts_with("-") {
                if column_name.is_empty() {
                    return Err("No column to add card to".to_string());
                }
                match line.split_at(6) {
                    ("- [ ] ", title) => {
                        kanban.add_card(
                            &CardBuilder::new()
                                .column(&column_name)
                                .title(title)
                                .build()?,
                        )?;
                    }
                    ("- [x] ", title) => {
                        kanban.add_card(
                            &CardBuilder::new()
                                .column(&column_name)
                                .title(title)
                                .status(Status::Done)
                                .build()?,
                        )?;
                    }
                    _ => (),
                }
            }
        }
        Ok(kanban)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = r#"## In Progress

- [ ] I'm doing it!!"#;

    #[test]
    fn test_add_card() {
        let mut kanban = Kanban::default();
        kanban.add_column("Test Column".to_string()).unwrap();

        let card = CardBuilder::new()
            .column("Test Column")
            .title("Test Card")
            .build()
            .unwrap();

        kanban.add_card(&card).unwrap();
        assert_eq!(kanban.columns, vec!["Test Column".to_string()]);
        assert_eq!(kanban.cards.len(), 1);
        assert_eq!(kanban.cards[0].title(), "Test Card");
    }

    #[test]
    fn test_add_card_invalid_column() {
        let mut kanban = Kanban::default();
        let card = CardBuilder::new()
            .column("Invalid Column")
            .title("Test Card")
            .build()
            .unwrap();

        assert!(kanban.add_card(&card).is_err());
    }

    #[test]
    fn test_parse() {
        let kanban = Kanban::parse(TEST_INPUT).unwrap();
        assert_eq!(kanban.columns, vec!["In Progress".to_string()]);
        assert_eq!(kanban.cards.len(), 1);
        assert_eq!(kanban.cards[0].title(), "I'm doing it!!");
        assert_eq!(kanban.cards[0].status(), &Status::Incomplete);
    }
}
