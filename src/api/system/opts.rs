use std::collections::HashMap;

/// Opts for filtering streams of Docker events
#[derive(Default, Debug)]
pub struct EventsOpts {
    params: HashMap<&'static str, String>,
}

impl EventsOpts {
    pub fn builder() -> EventsOptsBuilder {
        EventsOptsBuilder::default()
    }

    /// serialize Opts as a string. returns None if no Opts are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(crate::util::url::encoded_pairs(&self.params))
        }
    }
}

#[derive(Copy, Clone)]
pub enum EventFilterType {
    Container,
    Image,
    Volume,
    Network,
    Daemon,
}

fn event_filter_type_to_string(filter: EventFilterType) -> &'static str {
    match filter {
        EventFilterType::Container => "container",
        EventFilterType::Image => "image",
        EventFilterType::Volume => "volume",
        EventFilterType::Network => "network",
        EventFilterType::Daemon => "daemon",
    }
}

/// Filter Opts for image listings
pub enum EventFilter {
    Container(String),
    Event(String),
    Image(String),
    Label(String),
    Type(EventFilterType),
    Volume(String),
    Network(String),
    Daemon(String),
}

/// Builder interface for `EventOpts`
#[derive(Default)]
pub struct EventsOptsBuilder {
    params: HashMap<&'static str, String>,
    events: Vec<String>,
    containers: Vec<String>,
    images: Vec<String>,
    labels: Vec<String>,
    volumes: Vec<String>,
    networks: Vec<String>,
    daemons: Vec<String>,
    types: Vec<String>,
}

impl EventsOptsBuilder {
    #[cfg(feature = "chrono")]
    /// Only return events since this time.
    pub fn since<Tz>(&mut self, timestamp: &chrono::DateTime<Tz>) -> &mut Self
    where
        Tz: chrono::TimeZone,
    {
        self.params
            .insert("since", timestamp.timestamp().to_string());
        self
    }

    #[cfg(not(feature = "chrono"))]
    /// Only return events since this time, as a UNIX timestamp.
    pub fn since(&mut self, timestamp: i64) -> &mut Self {
        self.params.insert("since", timestamp.to_string());
        self
    }

    #[cfg(feature = "chrono")]
    /// Only return events before this time.
    pub fn until<Tz>(&mut self, timestamp: &chrono::DateTime<Tz>) -> &mut Self
    where
        Tz: chrono::TimeZone,
    {
        self.params
            .insert("until", timestamp.timestamp().to_string());
        self
    }

    #[cfg(not(feature = "chrono"))]
    /// Only return events before this time, as a UNIX timestamp.
    pub fn until(&mut self, timestamp: i64) -> &mut Self {
        self.params.insert("until", timestamp.to_string());
        self
    }

    pub fn filter(&mut self, filters: Vec<EventFilter>) -> &mut Self {
        let mut params = HashMap::new();
        for f in filters {
            match f {
                EventFilter::Container(n) => {
                    self.containers.push(n);
                    params.insert("container", self.containers.clone())
                }
                EventFilter::Event(n) => {
                    self.events.push(n);
                    params.insert("event", self.events.clone())
                }
                EventFilter::Image(n) => {
                    self.images.push(n);
                    params.insert("image", self.images.clone())
                }
                EventFilter::Label(n) => {
                    self.labels.push(n);
                    params.insert("label", self.labels.clone())
                }
                EventFilter::Volume(n) => {
                    self.volumes.push(n);
                    params.insert("volume", self.volumes.clone())
                }
                EventFilter::Network(n) => {
                    self.networks.push(n);
                    params.insert("network", self.networks.clone())
                }
                EventFilter::Daemon(n) => {
                    self.daemons.push(n);
                    params.insert("daemon", self.daemons.clone())
                }
                EventFilter::Type(n) => {
                    let event_type = event_filter_type_to_string(n).to_string();
                    self.types.push(event_type);
                    params.insert("type", self.types.clone())
                }
            };
        }
        self.params.insert(
            "filters",
            serde_json::to_string(&params).unwrap_or_default(),
        );
        self
    }

    pub fn build(&self) -> EventsOpts {
        EventsOpts {
            params: self.params.clone(),
        }
    }
}