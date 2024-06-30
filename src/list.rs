use uuid::Uuid;
pub struct List {
    pub list_name: String,
    pub convs: Vec<Uuid>,
}

impl List {
    pub fn new(name: String) -> List {
        List {
            list_name: name,
            convs: Vec::new(),
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