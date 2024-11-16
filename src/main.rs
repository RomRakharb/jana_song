use iced::widget::{button, column, Button, Column};
use std::collections::HashMap;

fn main() -> iced::Result {
    iced::run("test", AddressBook::update, AddressBook::view)
}

#[derive(Default, Debug, Clone)]
struct Address {
    house_no: String,
    soi: Option<String>,
    road: Option<String>,
    village: Option<String>,
    village_no: Option<u32>,
    sub_district: String,
    district: String,
    province: String,
    post_no: u32,
}

#[derive(Default)]
struct AddressBook {
    entries: HashMap<String, Address>,
}

impl AddressBook {
    fn update(&mut self, message: Message) {
        match message {
            Message::Create { name, address } => {
                self.entries.entry(name).or_insert(address);
            }
            Message::Update { name, address } => match self.entries.contains_key(&name) {
                true => {
                    self.entries.insert(name, address);
                }
                false => {}
            },
            Message::Delete(name) => {
                self.entries.remove(&name);
            }
            Message::Clear => todo!(),
        }
    }

    fn view(&self) -> Column<Message> {
        let create: Button<'_, Message, _, _> = button("Add").on_press(Message::Create {
            name: String::from("test"),
            address: Address::default(),
        });

        let interface = column![create];
        interface
    }
}

#[derive(Debug, Clone)]
enum Message {
    Create { name: String, address: Address },
    Update { name: String, address: Address },
    Delete(String),
    Clear,
}
