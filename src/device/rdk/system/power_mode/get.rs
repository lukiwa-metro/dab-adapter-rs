use crate::dab::structs::DabError;
use crate::dab::structs::PowerMode;
use crate::dab::structs::PowerModeGetRequest;
use crate::dab::structs::PowerModeGetResponse;
use crate::device::rdk::interface::rdk_request;
use crate::device::rdk::interface::RdkResponse;
use serde::{Deserialize, Serialize};

pub fn process(_dab_request: PowerModeGetRequest) -> Result<String, DabError> {
    let mut response = PowerModeGetResponse::default();

    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    struct GetPowerState {
        powerState: String,
        success: bool,
    }

    let rdkresponse: RdkResponse<GetPowerState> = rdk_request("org.rdk.System.getPowerState")?;

    //TODO: Move to a separate function
    response.powerMode = match rdkresponse.result.powerState.as_str() {
        "STANDBY" | "DEEP_SLEEP" | "LIGHT_SLEEP" => PowerMode::Standby,
        "ON" => PowerMode::Active,
        _ => {
            return Err(DabError::Err500(format!(
                "Unknown RDK power state {}",
                rdkresponse.result.powerState
            )))
        }
    };

    Ok(serde_json::to_string(&response).unwrap())
}
