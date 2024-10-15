// Code adapted from https://github.com/orhanbalci/rust-hazelcast-rest
// original MIT license included

//! This library is a wrapper around Hazelcast Rest API.
//! API includes methods to interact with distributed queues and maps only.
//! User can offer elements to named queues and poll elements from them.
//! User can also put elements to a map, remove elements one by one or remove all elements at once
//! from a map.
//!
//! #example
//!
//! ```no_run
//!
//! use hazelcast_rest::HazelcastRestClient;
//! let client = HazelcastRestClient::new("10.0.2.15", "5701");
//! client.queue_offer::<String>("sample_queue", "3".to_owned());
//! client.queue_offer::<String>("sample_queue", "4".to_owned());
//! assert_eq!(3, client.queue_delete("sample_queue", 10).unwrap().parse::<i32>().unwrap());
//!
//! ```
//!
//! #example
//!
//! ```no_run
//!
//! use hazelcast_rest::HazelcastRestClient;
//! let client = HazelcastRestClient::new("10.0.2.15", "5701");
//! client.map_put::<String>("capital_map", "Turkey", "Ankara".to_owned());
//! client.map_put::<String>("capital_map", "France", "Paris".to_owned());
//! client.map_put::<String>("capital_map", "Turkey", "Istanbul".to_owned());
//! client.map_remove_all("capital_map");
//! assert_eq!("Ankara", client.map_get("capital_map", "Turkey").unwrap());
//! assert_eq!("Paris", client.map_get("capital_map", "France").unwrap());
//!
//! ```

use std::io::Read;
use std::result::Result as StdResult;

use reqwest::blocking::Client;
use reqwest::Error;

/// Hazelcast rest api client struct.
pub struct HazelcastRestClient {
    ip_address: &'static str,
    port: &'static str,
    http_client: Client,
}

#[allow(unused_must_use)]
impl HazelcastRestClient {
    /// Creates a new client struct with given address and port
    pub fn new(ip_address: &'static str, port: &'static str) -> HazelcastRestClient {
        HazelcastRestClient {
            ip_address: ip_address,
            port: port,
            http_client: Client::new(),
        }
    }

    /// Inserts an element to the named queue
    pub fn queue_offer<T: ToString>(
        self: &Self,
        queue_name: &str,
        value: T,
    ) -> std::result::Result<String, Error> {
        let url_string = format!(
            "http://{}:{}/hazelcast/rest/queues/{}",
            self.ip_address, self.port, queue_name
        );
        self.http_client
            .post(&url_string)
            .body(value.to_string())
            .send()
            .and_then(|mut x| {
                let mut content = String::new();
                x.read_to_string(&mut content);
                StdResult::Ok(content)
            })
    }

    /// Polls an element from the named queue
    pub fn queue_delete(
        self: &Self,
        queue_name: &str,
        timeout: i32,
    ) -> std::result::Result<String, Error> {
        let url_string = format!(
            "http://{}:{}/hazelcast/rest/queues/{}/{}",
            self.ip_address,
            self.port,
            queue_name,
            timeout.to_string()
        );
        self.http_client
            .delete(&url_string)
            .send()
            .and_then(|mut x| {
                let mut content = String::new();
                x.read_to_string(&mut content);
                StdResult::Ok(content)
            })
    }

    /// Gets the size of the named queue. User should unwrap and parse the resultant string to get
    /// the number.
    pub fn queue_size(self: &Self, queue_name: &str) -> std::result::Result<String, Error> {
        let url_string = format!(
            "http://{}:{}/hazelcast/rest/queues/{}/size",
            self.ip_address, self.port, queue_name
        );
        self.http_client.get(&url_string).send().and_then(|mut x| {
            let mut content = String::new();
            x.read_to_string(&mut content);
            StdResult::Ok(content)
        })
    }

    /// Puts key-value to the named map. Overwrites if given key is already in map.
    pub fn map_put<T: ToString>(
        self: &Self,
        map_name: &str,
        key_name: &str,
        value: T,
    ) -> std::result::Result<String, Error> {
        let url_string = format!(
            "http://{}:{}/hazelcast/rest/maps/{}/{}",
            self.ip_address, self.port, map_name, key_name
        );
        self.http_client
            .post(&url_string)
            .body(value.to_string())
            .send()
            .and_then(|mut x| {
                let mut content = String::new();
                x.read_to_string(&mut content);
                StdResult::Ok(content)
            })
    }

    /// Gets element with given key from given map.
    pub fn map_get(
        self: &Self,
        map_name: &str,
        key_name: &str,
    ) -> std::result::Result<String, Error> {
        let url_string = format!(
            "http://{}:{}/hazelcast/rest/maps/{}/{}",
            self.ip_address, self.port, map_name, key_name
        );
        self.http_client.get(&url_string).send().and_then(|mut x| {
            let mut content = String::new();
            x.read_to_string(&mut content);
            StdResult::Ok(content)
        })
    }

    /// Removes element from given map with given key
    pub fn map_remove(
        self: &Self,
        map_name: &str,
        key_name: &str,
    ) -> std::result::Result<String, Error> {
        let url_string = format!(
            "http://{}:{}/hazelcast/rest/maps/{}/{}",
            self.ip_address, self.port, map_name, key_name
        );
        self.http_client
            .delete(&url_string)
            .send()
            .and_then(|mut x| {
                let mut content = String::new();
                x.read_to_string(&mut content);
                StdResult::Ok(content)
            })
    }

    /// Removes all elements of the named map.
    pub fn map_remove_all(self: &Self, map_name: &str) -> std::result::Result<String, Error> {
        let url_string = format!(
            "http://{}:{}/hazelcast/rest/maps/{}",
            self.ip_address, self.port, map_name
        );
        self.http_client
            .delete(&url_string)
            .send()
            .and_then(|mut x| {
                let mut content = String::new();
                x.read_to_string(&mut content);
                StdResult::Ok(content)
            })
    }
}
