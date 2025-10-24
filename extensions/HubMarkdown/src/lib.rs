mod enter;

use enter::on_enter_action;
use std::sync::Arc;
use zed_extension_api as zed;

struct HubMarkdown {}

impl zed::Extension for HubMarkdown {
    fn new() -> Self {
        Self {}
    }
}

zed::register_extension!(HubMarkdown);
