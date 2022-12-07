#[derive(Clone)]
pub struct AppConstants {}

impl AppConstants {
    // app info
    pub const APP_NAME: &str = "list2head";
    pub const APP_VERSION: &str = "v0.1 (experimental)";
    pub const APP_DEVELOPER: &str = "mikrob";
    pub const APP_DEVELOPER_WEBSITE: &str = "http://mikrob.it";

    // file locations
    pub const CHECKLIST_ARCHIVE_LOCATION: &str = "checklists/";
    pub const LOG_FILE_LOCATION: &str = "log/";

    pub const FONT_SIZE: f32 = 20.0;

    // TODO: format better
    pub const LICENSE_TEXT: &str =

"Copyright © 2022 mikrob\n\n
Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:\n\n
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.\n\n
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.";
}
