/// Receive alerts from alert manager.

// {
//   "version": "3",
//   "groupKey": <number>     // key identifying the group of alerts (e.g. to deduplicate)
//   "status": "<resolved|firing>",
//   "receiver": <string>,
//   "groupLabels": <object>,
//   "commonLabels": <object>,
//   "commonAnnotations": <object>,
//   "externalURL": <string>,  // backlink to the Alertmanager.
//   "alerts": [
//     {
//       "labels": <object>,
//       "annotations": <object>,
//       "startsAt": "<rfc3339>",
//       "endsAt": "<rfc3339>"
//     },
//     …
//   ]
// }

extern crate serde_json;

use std::collections::HashMap;

use chrono::{DateTime, UTC};

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    #[serde(rename = "firing")]
    Firing,
    #[serde(rename = "resolved")]
    Resolved,
}

type Labels = HashMap<String, String>;

type Annotations = HashMap<String, String>;

// https://github.com/serde-rs/serde/pull/788 will allow us to just specify
// the naming convention.

#[derive(Serialize, Deserialize, Debug)]
pub struct AlertMessage {
    version: String,
    #[serde(rename = "groupKey")]
    group_key: i32,
    status: Status,
    receiver: String,
    #[serde(rename = "groupLabels")]
    group_labels: Labels,
    #[serde(rename = "commonLabels")]
    common_labels: Labels,
    #[serde(rename = "commonAnnotations")]
    common_annotations: Annotations,
    #[serde(rename = "externalURL")]
    external_url: String, // TODO: URL
    alerts: Vec<Alert>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Alert {
    labels: Labels,
    annotations: Annotations,
    #[serde(rename = "startsAt")]
    starts_at: DateTime<UTC>,
    #[serde(rename = "endsAt")]
    ends_at: DateTime<UTC>,
}
