use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct List {
    /// The name of the list
    pub list_name: String,

    /// Vector containing the `Conversation`s in this list
    pub convs: Vec<Uuid>,

    /// Is the list open or not
    pub is_open: bool,

    /// In which place is this list in the list of lists
    pub order_index: usize,

    /// Mark the list for deletion
    pub delete: bool,
}

impl List {
    pub fn new(name: String) -> List {
        List {
            list_name: name,
            convs: Vec::new(),
            is_open: false,
            order_index: 0,
            delete: false,
        }
    }

    pub fn add(mut self, conv_uuid: Uuid) -> Self {
        self.convs.push(conv_uuid);
        self
    }

    pub fn remove(mut self, target_uuid: Uuid) -> Self {
        for (i, conv_uuid) in self.convs.iter().enumerate() {
            if *conv_uuid == target_uuid {
                self.convs.remove(i);
                break;
            }
        }
        self
    }
}