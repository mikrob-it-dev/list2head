use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::app_constants::AppConstants;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]

pub struct Checklist {
    pub name: String,
    pub sections: Vec<ChecklistSection>,
    #[serde(default = "default_string")]
    pub checklist_description: String,
    #[serde(skip_deserializing)]
    pub checklist_path: PathBuf,
}

impl Checklist {
    pub fn load_checklist(path: PathBuf) -> Self {
        let path_clone = path.clone();
        let serialized_checklist_in_file = File::open(path_clone);

        match &serialized_checklist_in_file {
            Ok(_) => {}
            Err(_) => {
                log::error!("Checklist file load failed, exiting");
            }
        }

        let mut buffer = String::new();
        let checklist_file_read_result = serialized_checklist_in_file
            .unwrap()
            .read_to_string(&mut buffer);

        match checklist_file_read_result {
            Ok(_) => {
                log::info!("Checklist file read successfully from: {}", path.display())
            }
            Err(_) => log::error!("Checklist file read failed"),
        }

        let mut section_order = 1;

        let mut checklist_deserialized: Checklist = serde_json::from_str(&buffer).unwrap();
        for section in &mut checklist_deserialized.sections {
            let mut step_order = 1;

            for mut step in &mut section.checklist_steps {
                step.result = StepResult::Unattempted;
                step.order = step_order;

                step_order += 1;
            }

            section.order = section_order;
            section_order += 1;
        }

        checklist_deserialized.checklist_path = path;

        checklist_deserialized
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ChecklistSection {
    #[serde(skip_deserializing)]
    pub order: usize,
    pub name: String,
    #[serde(default = "default_string")]
    pub section_description: String,
    pub checklist_steps: Vec<ChecklistStep>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ChecklistStep {
    #[serde(skip_deserializing)]
    pub order: u32,
    pub text: String,
    #[serde(default = "default_string")]
    pub comment: String,
    #[serde(skip_deserializing)]
    pub result: StepResult,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub enum StepResult {
    Correct,
    Incorrect,
    Skipped,
    #[default]
    Unattempted,
}

#[derive(PartialEq, Default)]
pub enum KeyboardInstruction {
    RegisterOkResult,
    RegisterNokResult,
    StepAhead,
    SkipStep,
    StartSection,
    SkipSection,
    #[default]
    None,
}

fn default_string() -> String {
    String::from("")
}

pub fn load_checklists() -> Vec<Checklist> {
    let path = Path::new(AppConstants::CHECKLIST_ARCHIVE_LOCATION);
    println!("{}", fs::read_dir(path).unwrap_err());
    let checklist_paths = fs::read_dir(path).unwrap();

    let mut checklists: Vec<Checklist> = Vec::new();

    checklist_paths
        .for_each(|x| checklists.push(Checklist::load_checklist(x.unwrap().path())));

    let checklist_count = checklists.clone().into_iter().count() as i32;

    if checklist_count == 0 {
        checklists = vec![get_blank_checklist()];
    }

    checklists
}

pub fn get_blank_checklist() -> Checklist {
    Checklist {
        name: String::from("N/A"),
        sections: vec![ChecklistSection {
            name: String::from("No checklists found"),
            section_description: String::from(""),
            order: 1,
            checklist_steps: vec![ChecklistStep {
                text: String::from(""),
                comment: String::from(""),
                order: 1,
                result: StepResult::Unattempted,
            }],
        }],
        checklist_description: String::from(
            "No checklists found. Import checklists in the .json format to the /checklists folder",
        ),
        checklist_path: PathBuf::from(""),
    }
}
