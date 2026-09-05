use std::path::PathBuf;

use godot::{
    classes::{Engine, INode, Node},
    prelude::*,
};
use reservist_content::frozen::{self, FrozenScenario};
use reservist_core::{
    api::{Command, Rejected, Session, View},
    save::SaveFile,
};
use serde_json::{Map, Value, json};

use crate::serialization::{dictionary_to_value, value_to_dictionary};

const DEFAULT_PACKAGE_ID: &str = "MEASURED_FIRMING";

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ReservistSession {
    base: Base<Node>,
    session: Option<Session>,
    scenario: Option<FrozenScenario>,
    scenario_path: Option<String>,
}

#[godot_api]
impl INode for ReservistSession {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            session: None,
            scenario: None,
            scenario_path: None,
        }
    }
}

#[godot_api]
impl ReservistSession {
    #[func]
    fn load_scenario(&mut self, path: GString) -> VarDictionary {
        let path = self.globalize_path(path);
        let candidate = (|| {
            let scenario = frozen::validate_scenario(&path)
                .map_err(|error| (error.category, error.message))?;
            let session = Session::new(&scenario, DEFAULT_PACKAGE_ID)
                .map_err(|error| (error.category, error.reason))?;
            Ok::<_, (String, String)>((scenario, session))
        })();

        match candidate {
            Ok((scenario, session)) => {
                self.scenario_path = Some(path.display().to_string());
                self.scenario = Some(scenario);
                self.session = Some(session);
                self.accepted(json!({"message": "Scenario loaded."}))
            }
            Err((category, reason)) => self.rejection(&category, reason),
        }
    }

    #[func]
    fn next_command_id(&self) -> GString {
        GString::from(
            self.session
                .as_ref()
                .map(Session::next_command_id)
                .unwrap_or_default()
                .as_str(),
        )
    }

    #[func]
    fn submit(&mut self, command: VarDictionary) -> VarDictionary {
        let command = match dictionary_to_value(&command).and_then(|value| {
            serde_json::from_value::<Command>(value).map_err(|error| error.to_string())
        }) {
            Ok(command) => command,
            Err(reason) => return self.rejection("invalid_command", reason),
        };
        let receipt = match self.session.as_mut() {
            Some(session) => session.submit(command),
            None => {
                return self.rejection("no_session", "Load a scenario before submitting commands.");
            }
        };
        self.receipt(receipt)
    }

    #[func]
    fn view(&self, name: GString) -> VarDictionary {
        let view = match serde_json::from_value::<View>(Value::String(name.to_string())) {
            Ok(view) => view,
            Err(error) => return self.rejection("invalid_view", error.to_string()),
        };
        let projection = match self.session.as_ref() {
            Some(session) => session.view(view),
            None => {
                return self.rejection("no_session", "Load a scenario before requesting views.");
            }
        };
        match projection {
            Ok(projection) => {
                self.accepted(serde_json::to_value(projection).expect("API projections serialize"))
            }
            Err(error) => self.rejected(error),
        }
    }

    #[func]
    fn preview_option(&self, option_id: GString) -> VarDictionary {
        let result = match self.session.as_ref() {
            Some(session) => session.preview_option(&option_id.to_string()),
            None => {
                return self.rejection("no_session", "Load a scenario before reviewing an option.");
            }
        };
        match result {
            Ok(card) => self.accepted(json!({"card": card})),
            Err(error) => self.rejected(error),
        }
    }

    #[func]
    fn save(&self, path: GString) -> VarDictionary {
        let path = self.globalize_path(path);
        let session = match self.session.as_ref() {
            Some(session) => session,
            None => return self.rejection("no_session", "Load a scenario before saving."),
        };
        let result = session
            .checkpoint(self.scenario_path.clone())
            .and_then(|save| save.write_to(&path));
        match result {
            Ok(()) => self.accepted(json!({"message": "Session saved."})),
            Err(error) => self.rejection(&error.category().to_string(), error.to_string()),
        }
    }

    #[func]
    fn resume(&mut self, path: GString) -> VarDictionary {
        let path = self.globalize_path(path);
        let scenario = match self.scenario.as_ref() {
            Some(scenario) => scenario,
            None => {
                return self.rejection("no_scenario", "Load a scenario before resuming a save.");
            }
        };
        let candidate =
            SaveFile::read_from(&path).and_then(|save| Session::resume(&save, scenario));
        match candidate {
            Ok(session) => {
                self.session = Some(session);
                self.accepted(json!({"message": "Session resumed."}))
            }
            Err(error) => self.rejection(&error.category().to_string(), error.to_string()),
        }
    }
}

impl ReservistSession {
    fn globalize_path(&self, path: GString) -> PathBuf {
        let mut settings = Engine::singleton()
            .get_singleton("ProjectSettings")
            .expect("Godot registers ProjectSettings before extension nodes");
        let global = settings
            .call("globalize_path", &[path.to_variant()])
            .to::<GString>();
        PathBuf::from(global.to_string())
    }

    fn accepted(&self, payload: Value) -> VarDictionary {
        self.envelope(payload, Some(true))
    }

    fn receipt(&self, receipt: reservist_core::api::Receipt) -> VarDictionary {
        self.envelope(
            serde_json::to_value(receipt).expect("API receipts serialize"),
            None,
        )
    }

    fn envelope(&self, payload: Value, accepted: Option<bool>) -> VarDictionary {
        let Some(session) = self.session.as_ref() else {
            return self.rejection("no_session", "No session is available.");
        };
        let mut response = match payload {
            Value::Object(response) => response,
            _ => return self.rejection("bridge", "API response was not an object."),
        };
        let available_verbs = match session.available_verbs() {
            Ok(verbs) => verbs,
            Err(error) => return self.rejected(error),
        };
        if let Some(accepted) = accepted {
            response.insert("accepted".into(), Value::Bool(accepted));
        }
        if !response.contains_key("state_hash") {
            response.insert("state_hash".into(), Value::String(session.state_hash()));
        }
        if !response.contains_key("current_time") {
            response.insert("current_time".into(), Value::String(session.current_time()));
        }
        if !response.contains_key("scenario_hash") {
            response.insert(
                "scenario_hash".into(),
                Value::String(session.scenario_hash().into()),
            );
        }
        response.insert(
            "available_verbs".into(),
            Value::Array(available_verbs.into_iter().map(Value::String).collect()),
        );
        value_to_dictionary(Value::Object(response))
            .expect("API response converts to a Godot Dictionary")
    }

    fn rejected(&self, error: Rejected) -> VarDictionary {
        self.rejection(&error.category, error.reason)
    }

    fn rejection(&self, category: &str, reason: impl Into<String>) -> VarDictionary {
        let state_hash = self
            .session
            .as_ref()
            .map(Session::state_hash)
            .unwrap_or_default();
        let response = Map::from_iter([
            ("accepted".into(), Value::Bool(false)),
            ("category".into(), Value::String(category.into())),
            ("reason".into(), Value::String(reason.into())),
            ("state_hash".into(), Value::String(state_hash.clone())),
            ("previous_state_hash".into(), Value::String(state_hash)),
        ]);
        value_to_dictionary(Value::Object(response))
            .expect("rejections convert to a Godot Dictionary")
    }
}
