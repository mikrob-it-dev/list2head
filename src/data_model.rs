use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::app_constants::AppConstants;

#[derive(Clone, Serialize, Deserialize, Debug)]

pub struct checklist {
    pub name: String,
    pub checklist_steps: Vec<checklist_step>,
}

impl checklist {
    pub fn load_checklist() -> Self {
        let serialized_checklist_in_file = File::open(AppConstants::CONFIG_FILE_LOCATION);

        match &serialized_checklist_in_file {
            Ok(_) => {}
            Err(_) => {
                log::error!("Checklist file load failed, exiting");
            }
        }

        let mut buffer = String::new();
        let config_file_read_result = serialized_checklist_in_file
            .unwrap()
            .read_to_string(&mut buffer);

        match config_file_read_result {
            Ok(_) => {
                log::info!("Checklist file read successfully: {}", &buffer)
            }
            Err(_) => log::error!("Checklist file read failed"),
        }

        let mut checklist_deserialized: checklist = serde_json::from_str(&buffer).unwrap();
        for mut step in &mut checklist_deserialized.checklist_steps {
            step.test_result = String::from("N/A");
        }

        checklist_deserialized
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct checklist_step {
    pub order: u32,
    pub text: String,
    pub comment: String,
    pub section: String,
    #[serde(skip_deserializing)]
    pub test_result: String,
}

impl checklist_step {
    pub fn test_step() -> Self {
        checklist_step {
            order: 1,
            text: String::from("test text"),
            comment: String::from("test comment"),
            section: String::from("test section"),
            test_result: String::from("N/A"),
        }
    }
}
