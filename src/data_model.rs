use std::{
    fs::{self, File, ReadDir},
    io::Read,
    path::{self, Path},
};

use serde::{Deserialize, Serialize};

use crate::app_constants::AppConstants;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]

pub struct checklist {
    pub name: String,
    pub sections: Vec<checklist_section>,
    pub checklist_description: String,
}

impl checklist {
    pub fn load_checklist(path: &Path) -> Self {
        let serialized_checklist_in_file = File::open(path);

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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct checklist_section {
    #[serde(skip_deserializing)]
    pub order: usize,
    pub name: String,
    pub section_description: String,
    pub checklist_steps: Vec<checklist_step>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct checklist_step {
    #[serde(skip_deserializing)]
    pub order: u32,
    pub text: String,
    pub comment: String,
    #[serde(skip_deserializing)]
    pub test_result: String,
}

pub fn load_checklists() -> Vec<checklist> {
    let path = Path::new(AppConstants::CHECKLIST_ARCHIVE_LOCATION);
    let mut checklist_paths = fs::read_dir(path);

    let mut checklists: Vec<checklist> = Vec::new();

    checklist_paths
        .unwrap()
        .for_each(|x| checklists.push(checklist::load_checklist(x.unwrap().path().as_path())));

    let checklist_count = checklists.clone().into_iter().count() as i32;

    if checklist_count == 0 {
        checklists = vec![get_blank_checklist()];
    }

    checklists
}

pub fn get_blank_checklist() -> checklist {
    checklist {
        name: String::from("N/A"),
        sections: vec![checklist_section {
            name: String::from("No checklists found"),
            section_description: String::from(""),
            order: 1,
            checklist_steps: vec![checklist_step {
                text: String::from(""),
                comment: String::from(""),
                order: 1,
                test_result: String::from("N/A"),
            }],
        }],
        checklist_description: String::from(
            "No checklists found. Import checklists in the .json format to the /checklists folder",
        ),
    }
}
