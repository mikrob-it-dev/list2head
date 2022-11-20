use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::app_constants::AppConstants;

#[derive(Clone, Serialize, Deserialize, Debug)]

pub struct checklist {
    pub name: String,
    pub sections: Vec<checklist_section>,
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

        let mut section_order = 1;

        let mut checklist_deserialized: checklist = serde_json::from_str(&buffer).unwrap();
        for section in &mut checklist_deserialized.sections {

            let mut step_order = 1;

            for mut step in &mut section.checklist_steps {
                step.test_result = String::from("N/A");
                step.order = step_order;

                step_order += 1;
            }

            section.order = section_order;
            section_order += 1;
        }

        checklist_deserialized
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct checklist_section {
    #[serde(skip_deserializing)]
    pub order: usize,
    pub name: String,
    pub section_description: String,
    pub checklist_steps: Vec<checklist_step>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct checklist_step {
    #[serde(skip_deserializing)]
    pub order: u32,
    pub text: String,
    pub comment: String,
    #[serde(skip_deserializing)]
    pub test_result: String,
}
