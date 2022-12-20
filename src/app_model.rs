use crate::data_model::{self, Checklist, StepResult, ChecklistStep};

#[derive(Clone)]
pub struct ChecklistPosition {
    pub section: usize,
    pub step: usize,
    pub start_section: u32,
}

#[derive(Clone)]
pub struct EguiApp {
    pub is_license_info_shown: bool,
    pub checklists: Vec<Checklist>,
    pub selected_checklist: Checklist,
    pub checklist_position: ChecklistPosition,
    pub checklist_start_section_string: String,
}

impl EguiApp {
    pub fn default() -> Self {
        Self {
            is_license_info_shown: false,
            checklists: data_model::load_checklists(),
            checklist_position: ChecklistPosition {
                section: 1,
                step: 0,
                start_section: 1,
            },
            selected_checklist: data_model::load_checklists().get(0).unwrap().to_owned(),
            checklist_start_section_string: String::from("1"),
        }
    }

    // TODO: Consider using first / last instead of actual indices
    pub fn is_end_of_checklist_reached(&self) -> bool {
        let number_of_sections = self.selected_checklist.sections.iter().count();
        let last_checklist_step = self
            .selected_checklist
            .sections
            .last()
            .unwrap()
            .checklist_steps
            .iter()
            .count();
        if (self.checklist_position.section == number_of_sections)
            && (self.checklist_position.step == last_checklist_step)
        {
            return true;
        } else {
            return false;
        };
    }

    pub fn set_current_checklist_step_result(&mut self, result: StepResult) {
        self.selected_checklist.sections[&self.checklist_position.section - 1]
                    .checklist_steps[&self.checklist_position.step - 1].result = result;
    }

    pub fn advance_checklist_step(&mut self, skip_step: bool) {
        if !self.is_end_of_checklist_reached() {
            if skip_step && self.checklist_position.step != 0 {
                self.set_current_checklist_step_result(StepResult::Skipped);
            }

            self.checklist_position.step += 1;

            if self.checklist_position.step
                == self.selected_checklist.sections[self.checklist_position.section - 1]
                    .checklist_steps
                    .iter()
                    .count()
                    + 1
            {
                // TODO: improve... starting at 0 to let section jump in first
                self.checklist_position.step = 0;
                self.checklist_position.section += 1;
            };
        }
    }

    pub fn advance_checklist_section(&mut self, skip_section: bool) {
        if !self.is_end_of_checklist_reached() && skip_section {
            for step in &mut self.selected_checklist.sections[self.checklist_position.section - 1]
                .checklist_steps
            {
                step.result = StepResult::Skipped;
            }

            self.checklist_position.section += 1;
        } else {
            self.checklist_position.step = 1;
        }
    }

    pub fn reset_checklist(&mut self, starting_section: usize) {
        self.checklist_position.step = 0;
        self.checklist_position.section = starting_section;

        for section in &mut self.selected_checklist.sections {
            for step in &mut section.checklist_steps {
                step.result = StepResult::Unattempted;
            }
        }
    }
}
