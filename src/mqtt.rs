use core::cell::RefCell;
use core::pin::Pin;
use core::{num::ParseIntError, str::Utf8Error};

use core::{error, fmt};
use embassy_net::tcp::TcpSocket;
use log::{debug, error};
use rand_core::RngCore;
use rust_mqtt::client::client_config;
use rust_mqtt::packet::v5::publish_packet::QualityOfService::QoS2;
use rust_mqtt::packet::v5::reason_codes::ReasonCode;
use rust_mqtt::utils::rng_generator::CountingRng;
use rust_mqtt::{client::client::MqttClient, client::client_config::ClientConfig};
use static_cell::StaticCell;

pub type MqttClientAlias<'a, T> = MqttClient<'a, &'a mut TcpSocket<'static>, 5, T>;

pub type ClientConfigAlias<'a, T: RngCore> = ClientConfig<'a, 5, T>;

pub struct MQTT<'a> {
    pub client: &'a mut MqttClientAlias<'a, CountingRng>,
}

pub enum BrokerMessage<'b> {
    WaitFor(u64),
    Reset(bool),
    TopicPayload(&'b str, &'b [u8]),
}

type Result<T> = core::result::Result<T, BrokerMessageError>;

pub const PICO_TIME_TOPIC: &'static str = "pico-time";
pub const PICO_STATUS_TOPIC: &'static str = "pico-status";
pub const PICO_TOPIC: &'static str = "pico";

#[derive(Debug)]
pub enum BrokerMessageError {
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    Parse(ParseIntError),
    Utf8Error,
    InvalidTopic,
    Mqtt(ReasonCode),
}

#[derive(Debug)]
struct InvalidTopic;

impl fmt::Display for BrokerMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BrokerMessageError::Parse(..) => {
                write!(f, "the provided string could not be parsed as int")
            }
            BrokerMessageError::Utf8Error => {
                write!(f, "the provided string is not a valid utf-8 string")
            }
            BrokerMessageError::InvalidTopic => write!(f, "invalid topic"),
            BrokerMessageError::Mqtt(ref code) => {
                write!(f, "mqtt request failed with code: {:?}", code)
            }
        }
    }
}

impl error::Error for BrokerMessageError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            BrokerMessageError::Utf8Error => None,
            BrokerMessageError::InvalidTopic => Some(&InvalidTopic),
            BrokerMessageError::Parse(ref e) => Some(e),
            BrokerMessageError::Mqtt(..) => None,
        }
    }
}

impl fmt::Display for InvalidTopic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid topic")
    }
}

impl error::Error for InvalidTopic {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<ParseIntError> for BrokerMessageError {
    fn from(err: ParseIntError) -> BrokerMessageError {
        BrokerMessageError::Parse(err)
    }
}

impl From<Utf8Error> for BrokerMessageError {
    fn from(_: Utf8Error) -> BrokerMessageError {
        BrokerMessageError::Utf8Error
    }
}

impl From<ReasonCode> for BrokerMessageError {
    fn from(code: ReasonCode) -> BrokerMessageError {
        BrokerMessageError::Mqtt(code)
    }
}

impl<'a> MQTT<'a> {
    pub fn new(
        socket: &'a RefCell<TcpSocket<'static>>,
        client_id: &'a str,
        username: &'a str,
        password: &'a str,
    ) -> Self {
        let mut config: ClientConfigAlias<'a, CountingRng> = client_config::ClientConfig::new(
            client_config::MqttVersion::MQTTv5,
            CountingRng(20000),
        );
        config.add_client_id(client_id);
        config.add_max_subscribe_qos(QoS2);
        config.add_username(username);
        config.add_password(password);
        config.add_will(PICO_STATUS_TOPIC, "offline".as_bytes(), true);
        config.keep_alive = 43200;
        config.max_packet_size = 100;

        static RECV_BUFFER: StaticCell<[u8; 80]> = StaticCell::new();
        static WRITE_BUFFER: StaticCell<[u8; 80]> = StaticCell::new();

        let mut client: MqttClientAlias<'a, CountingRng> = MqttClient::new(
            &mut socket.borrow_mut(),
            WRITE_BUFFER.init([0; 80]),
            80,
            RECV_BUFFER.init([0; 80]),
            80,
            config,
        );

        Self {
            client: &mut client,
        }
    }
    pub async fn connect(&mut self) -> Result<()> {
        self.client.connect_to_broker().await.unwrap();
        self.client
            .subscribe_to_topic(PICO_TIME_TOPIC)
            .await
            .unwrap();
        self.client.connect_to_broker().await?;
        Ok(())
    }
    pub async fn receive_broker_message(&mut self) -> Result<BrokerMessage> {
        let (topic, payload) = match self.client.receive_message().await {
            Ok((topic, payload)) => (topic, payload),
            Err(err) => {
                error!("failed to receive message from broker. {:?}", err);
                return Err(err.into());
            }
        };
        if topic.eq(PICO_TIME_TOPIC) {
            if payload.len() == 0 {
                debug!("got no payload. assuming no wait time, continuing...");
                return Ok(BrokerMessage::WaitFor(0));
            }
            let wait_for_str: &str = match core::str::from_utf8(payload) {
                Ok(s) => s,
                Err(err) => {
                    error!(
                        "payload is of an invalid type. expected utf-8 string. {:?}",
                        err
                    );
                    return Err(err.into());
                }
            };
            let wait_for = match wait_for_str.parse::<u64>() {
                Ok(i) => i,
                Err(err) => {
                    error!("utf-8 string is not a valid number. {:?}", err);
                    return Err(err.into());
                }
            };

            debug!("wait_for: {}s", wait_for);
            return Ok(BrokerMessage::WaitFor(wait_for));
        }
        Err(BrokerMessageError::InvalidTopic)
    }
}
