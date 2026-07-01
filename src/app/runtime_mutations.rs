use crate::api::schema::{Method, PaneTarget, TabTarget, WorkspaceTarget};

use super::App;

impl App {
    pub(crate) fn dispatch_runtime_mutation(&mut self, id: &'static str, method: Method) -> String {
        self.dispatch_api_request(id, method)
    }

    pub(crate) fn dispatch_deferred_runtime_mutation(
        &mut self,
        id: &'static str,
        method: Method,
    ) -> Option<String> {
        self.dispatch_deferred_api_request(id, method)
    }

    pub(crate) fn runtime_workspace_focus(
        &mut self,
        id: &'static str,
        workspace_id: String,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::WorkspaceFocus(WorkspaceTarget { workspace_id }))
    }

    pub(crate) fn runtime_workspace_close(
        &mut self,
        id: &'static str,
        workspace_id: String,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::WorkspaceClose(WorkspaceTarget { workspace_id }))
    }

    pub(crate) fn runtime_tab_focus(&mut self, id: &'static str, tab_id: String) -> String {
        self.dispatch_runtime_mutation(id, Method::TabFocus(TabTarget { tab_id }))
    }

    pub(crate) fn runtime_tab_close(&mut self, id: &'static str, tab_id: String) -> String {
        self.dispatch_runtime_mutation(id, Method::TabClose(TabTarget { tab_id }))
    }

    pub(crate) fn runtime_pane_focus(&mut self, id: &'static str, pane_id: String) -> String {
        self.dispatch_runtime_mutation(id, Method::PaneFocus(PaneTarget { pane_id }))
    }

    pub(crate) fn runtime_pane_close(&mut self, id: &'static str, pane_id: String) -> String {
        self.dispatch_runtime_mutation(id, Method::PaneClose(PaneTarget { pane_id }))
    }
}
