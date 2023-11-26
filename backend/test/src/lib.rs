use reqwest::blocking::Response;
use serde_json::Value;
use std::collections::HashMap;
const BASE_URL: &str = "http://localhost:8080/api";

fn make_request(
    endpoint: &str,
    method: reqwest::Method,
    body: Option<HashMap<&str, &str>>,
) -> Response {
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/{}", BASE_URL, endpoint);
    let request_builder = match method {
        reqwest::Method::GET => client.get(&url),
        reqwest::Method::POST => client.post(&url).json(&body.unwrap()),
        reqwest::Method::PATCH => client.patch(&url).json(&body.unwrap()),
        reqwest::Method::DELETE => client.delete(&url),
        _ => panic!("Unsupported HTTP method"),
    };

    request_builder.send().expect("Failed to execute request")
}

#[cfg(test)]
pub mod tests {
    use serde_json::json;

    use super::*;
    #[test]
    fn test_residents_index() {
        let response = make_request("residents", reqwest::Method::GET, None);
        assert_eq!(response.status().as_u16(), 200);
    }
    #[test]
    fn test_residents_show() {
        let resident_id = "111111111111111";

        let response = make_request(
            format!("residents/{}", resident_id).as_str(),
            reqwest::Method::GET,
            None,
        );
        assert_eq!(response.status().as_u16(), 200);
    }
    #[test]
    fn test_residents_create() {
        let fake_location = json!({"rfid": "338888222889999", "name": "Fake resident", "doc": "29752", "room": "C-8", "unit": 4, "current_location": 4});
        let resp = reqwest::blocking::Client::new()
            .post(format!("{}/residents", BASE_URL))
            .json(&fake_location)
            .send()
            .expect("Failed to execute request");

        assert_eq!(resp.status().as_u16(), 201);
    }
    #[test]
    fn test_residents_update() {
        let resident_id = "111111111111111";
        let updated_data = [("name", "Updated Name")].iter().cloned().collect();
        let response = make_request(
            &format!("residents/{}", resident_id),
            reqwest::Method::PATCH,
            Some(updated_data),
        );
        assert_eq!(response.status().as_u16(), 200);
    }

    #[test]
    fn test_residents_delete() {
        let resident_id = "338888222889999";
        let response = make_request(
            &format!("residents/{}", resident_id),
            reqwest::Method::DELETE,
            None,
        );
        assert_eq!(response.status().as_u16(), 204);
    }

    #[test]
    fn test_locations_index() {
        let response = make_request("locations", reqwest::Method::GET, None);
        assert_eq!(response.status().as_u16(), 200);
    }
    #[test]
    fn test_locations_show() {
        let response = make_request("locations/4", reqwest::Method::GET, None);
        assert_eq!(response.status().as_u16(), 200);
        assert_eq!(response.json::<Value>().unwrap()["data"][0]["name"], "ASU");
    }
    #[test]
    fn test_locations_create() {
        let fake_location = json!({"id": 69, "name": "Fake Location"});
        let resp = reqwest::blocking::Client::new()
            .post(format!("{}/locations", BASE_URL))
            .json(&fake_location)
            .send()
            .expect("Failed to execute request");
        assert_eq!(resp.status().as_u16(), 201);
    }

    #[test]
    fn test_locations_timestamps() {
        let response = make_request("locations/8/timestamps", reqwest::Method::GET, None);
        assert_eq!(response.status().as_u16(), 200);
        assert!(response.json::<Vec<Value>>().is_ok());
    }

    #[test]
    fn test_locations_timestamps_between() {
        let response = make_request(
            "locations/13/timestamps/2023-11-10/2023-11-30",
            reqwest::Method::GET,
            None,
        );
        assert_eq!(response.status().as_u16(), 200);
    }
    #[test]
    fn test_locations_residents() {
        let response = make_request("locations/8/residents", reqwest::Method::GET, None);
        assert_eq!(response.status().as_u16(), 200);
    }

    #[test]
    fn test_timestamps() {
        // TestTimestampsController
        let response = make_request("timestamps", reqwest::Method::GET, None);
        assert_eq!(response.status().as_u16(), 200);
    }

    #[test]
    fn test_timestamps_post() {
        let data = json!({"rfid": "111111111111111", "location": 9});
        let response = reqwest::blocking::Client::new()
            .post(format!("{}/timestamps", BASE_URL))
            .json(&data)
            .send()
            .expect("Failed to execute request");
        assert_eq!(response.status().as_u16(), 201);
    }
    #[test]
    fn test_timestamps_between() {
        let response = make_request(
            "timestamps/2023-11-18/2023-11-30",
            reqwest::Method::GET,
            None,
        );
        assert_eq!(response.status().as_u16(), 200);
    }
}
