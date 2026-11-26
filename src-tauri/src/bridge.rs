// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Grimoire Authors

//! The bridge between the webview and `grim-sdk`.

use grim_sdk::GrimDriver;
use serde::Serialize;

/// Failure of a bridge call, serialized to the front end as `{kind, code, message}`.
#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum BridgeError {
    NotFound { code: Option<i32>, message: String },
    Failed { code: Option<i32>, message: String },
}

/// The version of the grim `driver` points at, as the SDK reports it.
pub fn version_with(driver: &GrimDriver) -> Result<String, BridgeError> {
    driver.version().map_err(|e| match e.code {
        None => BridgeError::NotFound {
            code: None,
            message: e.message,
        },
        code => BridgeError::Failed {
            code,
            message: e.message,
        },
    })
}

/// The one stderr line a call writes; the acceptance suite reads it.
pub fn log_line(result: &Result<String, BridgeError>) -> String {
    match result {
        Ok(v) => format!("grim-gui: grim_version ok version={v}"),
        Err(BridgeError::NotFound { .. }) => "grim-gui: grim_version not-found".to_owned(),
        Err(BridgeError::Failed { code: Some(c), .. }) => format!("grim-gui: grim_version failed code={c}"),
        Err(BridgeError::Failed { code: None, .. }) => "grim-gui: grim_version failed".to_owned(),
    }
}

/// Async, with the blocking SDK call on the blocking pool: a hung grim must not freeze the window.
#[tauri::command]
pub async fn grim_version() -> Result<String, BridgeError> {
    let result = tauri::async_runtime::spawn_blocking(|| {
        version_with(&GrimDriver {
            bin: grim_sdk::bin_from_env(),
        })
    })
    .await
    .unwrap_or_else(|e| {
        Err(BridgeError::Failed {
            code: None,
            message: format!("grim call did not complete: {e}"),
        })
    });
    eprintln!("{}", log_line(&result));
    result
}

#[cfg(all(test, unix))]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};

    use super::*;

    fn fake(dir: &Path, body: &str) -> GrimDriver {
        let bin: PathBuf = dir.join("grim");
        fs::write(&bin, format!("#!/bin/sh\n{body}\n")).unwrap();
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o755)).unwrap();
        GrimDriver { bin }
    }

    #[test]
    fn ok_returns_the_version_text() {
        let dir = tempfile::tempdir().unwrap();
        let d = fake(dir.path(), "echo 'grim 0.17.0'");
        let r = version_with(&d);
        assert_eq!(r.as_ref().unwrap(), "0.17.0");
        assert_eq!(log_line(&r), "grim-gui: grim_version ok version=0.17.0");
    }

    #[test]
    fn missing_grim_is_not_found_with_no_code() {
        let d = GrimDriver {
            bin: PathBuf::from("/nonexistent/dir/grim"),
        };
        let r = version_with(&d);
        match &r {
            Err(BridgeError::NotFound { code: None, message }) => assert!(!message.is_empty()),
            other => panic!("{other:?}"),
        }
        assert_eq!(log_line(&r), "grim-gui: grim_version not-found");
    }

    #[test]
    fn failing_grim_is_failed_with_the_sdk_code() {
        let dir = tempfile::tempdir().unwrap();
        let d = fake(dir.path(), "echo oops >&2; exit 78");
        let r = version_with(&d);
        // The SDK folds every undocumented exit code into 69 "unexpected output".
        match &r {
            Err(BridgeError::Failed {
                code: Some(69),
                message,
            }) => assert!(message.contains("78"), "{message}"),
            other => panic!("{other:?}"),
        }
        assert_eq!(log_line(&r), "grim-gui: grim_version failed code=69");
    }

    #[test]
    fn a_join_failure_logs_failed_without_a_code() {
        let r: Result<String, BridgeError> = Err(BridgeError::Failed {
            code: None,
            message: "m".into(),
        });
        assert_eq!(log_line(&r), "grim-gui: grim_version failed");
    }

    #[test]
    fn empty_version_output_is_passed_on_as_empty() {
        let dir = tempfile::tempdir().unwrap();
        let d = fake(dir.path(), "true");
        assert_eq!(version_with(&d).unwrap(), "");
    }

    #[test]
    fn a_non_executable_grim_is_not_a_success() {
        let dir = tempfile::tempdir().unwrap();
        let bin = dir.path().join("grim");
        fs::write(&bin, "#!/bin/sh\necho 'grim 1'\n").unwrap();
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o644)).unwrap();
        let r = version_with(&GrimDriver { bin });
        assert!(r.is_err(), "{r:?}");
        assert!(log_line(&r).starts_with("grim-gui: grim_version "));
    }

    #[test]
    fn unrecognised_output_is_passed_on_unchanged() {
        let dir = tempfile::tempdir().unwrap();
        let d = fake(dir.path(), "echo garbage");
        assert_eq!(version_with(&d).unwrap(), "garbage");
    }

    #[test]
    fn errors_serialize_as_kind_code_message() {
        let e = BridgeError::NotFound {
            code: None,
            message: "m".into(),
        };
        assert_eq!(
            serde_json::to_value(&e).unwrap(),
            serde_json::json!({"kind": "not-found", "code": null, "message": "m"})
        );
        let e = BridgeError::Failed {
            code: Some(7),
            message: "m".into(),
        };
        assert_eq!(
            serde_json::to_value(&e).unwrap(),
            serde_json::json!({"kind": "failed", "code": 7, "message": "m"})
        );
    }
}
