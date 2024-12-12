use super::card::{Card, CardBuilder};
use super::status::Status;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "kanban.pest"]
pub struct KanbanParser;

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
        Ok(())
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
        let pairs = KanbanParser::parse(Rule::kanban, input).map_err(|e| e.to_string())?;

        let mut kanban = Kanban::default();
        let mut current_column = String::new();

        for pair in pairs.into_iter().next().unwrap().into_inner() {
            match pair.as_rule() {
                Rule::column_heading => {
                    for inner in pair.into_inner() {
                        if inner.as_rule() == Rule::text {
                            current_column = inner.as_str().to_string();
                            kanban.add_column(current_column.clone())?;
                        }
                    }
                }
                Rule::card => {
                    let mut card_text = String::new();
                    let mut status = Status::Incomplete;

                    for part in pair.into_inner() {
                        match part.as_rule() {
                            Rule::status => {
                                let status_inner = part.into_inner().next().unwrap();
                                status = match status_inner.as_rule() {
                                    Rule::complete => Status::Done,
                                    Rule::incomplete => Status::Incomplete,
                                    _ => return Err("Invalid status".to_string()),
                                };
                            }
                            Rule::text => {
                                card_text = part.as_str().to_string();
                            }
                            _ => {}
                        }
                    }

                    kanban.add_card(
                        &CardBuilder::new()
                            .column(&current_column)
                            .title(&card_text)
                            .status(status)
                            .build()?,
                    )?;
                }
                _ => {}
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
