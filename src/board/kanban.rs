use super::card::{Card, CardBuilder};
use super::status::Status;
use pest::Parser;
use pest_derive::Parser;

/// A PEG parser for Kanban files using a Pest grammar.
/// Uses the grammar defined in `kanban.pest` to parse Kanban boards and cards.
#[derive(Parser)]
#[grammar = "kanban.pest"]
pub struct KanbanParser;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Kanban {
    columns: Vec<String>,
    cards: Vec<Card>,
}

impl Kanban {
    /// Creates a new Kanban board with the specified columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use kantui::Kanban;
    /// let board = Kanban::new(&["To Do", "In Progress", "Done"]);
    /// ```
    pub fn new(columns: &[&str]) -> Self {
        let columns = columns.iter().map(|c| c.to_string()).collect();
        Kanban {
            columns,
            cards: Vec::new(),
        }
    }

    /// Adds a new column to the Kanban board.
    ///
    /// # Examples
    ///
    /// ```
    /// use kantui::Kanban;
    /// let mut board = Kanban::default();
    /// board.add_column("To Do".to_string()).unwrap();
    /// ```
    pub fn add_column(&mut self, name: String) -> Result<(), String> {
        self.columns.push(name);

        Ok(())
    }

    /// Adds a card to the Kanban board in the specified column.
    /// Returns an error if the column does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use kantui::{Kanban, Card, CardBuilder};
    /// let mut board = Kanban::default();
    /// board.add_column("To Do".to_string()).unwrap();
    ///
    /// let card = CardBuilder::new()
    ///     .column("To Do")
    ///     .title("Implement feature")
    ///     .build()
    ///     .unwrap();
    ///
    /// board.add_card(&card).unwrap();
    /// ```
    pub fn add_card(&mut self, card: &Card) -> Result<(), String> {
        self.has_column(card.column())?;
        self.cards.push(card.clone());
        Ok(())
    }

    /// Internal helper to check if a column exists.
    fn has_column(&self, column: &String) -> Result<(), String> {
        match self.columns.contains(column) {
            true => Ok(()),
            false => Err("Column does not exist".to_string()),
        }
    }

    /// Moves a card to a different column.
    /// Returns an error if the target column does not exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use kantui::{Kanban, Card, CardBuilder};
    /// let mut board = Kanban::default();
    /// board.add_column("To Do".to_string()).unwrap();
    /// board.add_column("Done".to_string()).unwrap();
    ///
    /// let card = CardBuilder::new()
    ///     .column("To Do")
    ///     .title("Task")
    ///     .build()
    ///     .unwrap();
    ///
    /// board.add_card(&card).unwrap();
    /// board.move_card(&"Done".to_string(), card).unwrap();
    /// ```
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

    /// Parses a Kanban board from a string in the markdown-like format.
    /// Returns an error if the input is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use kantui::Kanban;
    ///
    /// let input = r#"## To Do
    /// - [ ] Implement feature
    /// - [x] Write docs"#;
    ///
    /// let board = Kanban::parse(input).unwrap();
    /// ```
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
                    let mut date: Option<String> = None;

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
                            Rule::date => {
                                let date_str = part.as_str();
                                date = Some(date_str.to_string());
                            }
                            _ => {}
                        }
                    }

                    let mut card = CardBuilder::new()
                        .column(&current_column)
                        .title(&card_text)
                        .status(status);
                    if let Some(date) = date {
                        card = card.date(&date);
                    }
                    let card = card.build()?;

                    kanban.add_card(&card)?;
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

    #[test]
    fn test_parse_with_date() {
        let input = r#"## To Do

- [ ] Task with date @{2024-01-15}
- [ ] Second Task
"#;

        let kanban = Kanban::parse(input).unwrap();
        assert_eq!(kanban.columns, vec!["To Do".to_string()]);
        assert_eq!(kanban.cards.len(), 2);
        assert_eq!(kanban.cards[0].title().trim(), "Task with date");
        assert_eq!(kanban.cards[0].date(), Some("2024-01-15".to_string()));
    }

    #[test]
    fn test_date_parse() {
        let input = "2121-12-12";
        let parser = KanbanParser::parse(Rule::date, input).unwrap().as_str();
        assert_eq!(parser, input);
    }

    #[test]
    fn test_parse_card() {
        let input = "- [x] Title @{2025-01-02}";
        let parser = KanbanParser::parse(Rule::card, input)
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(parser.into_inner().len(), 3);
    }
}
