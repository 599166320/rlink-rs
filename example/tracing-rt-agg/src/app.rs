use std::time::Duration;
use rlink::core::backend::KeyedStateBackend;
use rlink::core::data_stream::TDataStream;
use rlink::core::data_types::Schema;
use rlink::core::env::{StreamApp, StreamExecutionEnvironment};
use rlink::core::properties::{FunctionProperties, Properties, SystemProperties};
use rlink::core::runtime::ClusterDescriptor;
use rlink::functions::source::vec_source;
use rlink::utils::process::parse_arg;
use rlink_connector_kafka::{BOOTSTRAP_SERVERS, KAFKA, TOPICS};
use rlink_connector_kafka::sink::builder::KafkaOutputFormatBuilder;
use rlink_example_utils::gen_record::gen_records;

#[derive(Clone, Debug)]
pub struct TracingRtStreamApp {}

const KAFKA_FN: &'static str = "kafka_fn";
impl StreamApp for TracingRtStreamApp {
    async fn prepare_properties(&self, properties: &mut Properties) {
        properties.set_application_name("tracing-rt-agg");

        properties.set_pub_sub_channel_size(1024 * 1000);
        properties.set_checkpoint_interval(Duration::from_secs(2 * 60));
        properties.set_keyed_state_backend(KeyedStateBackend::Memory);

        let brokers = parse_arg("brokers").unwrap_or("localhost:9092".to_string());
        let topic = parse_arg("topic").unwrap_or("rlink-test".to_string());

        let kafka_sink_properties = {
            let mut sink_properties = Properties::new();
            sink_properties.set_str(TOPICS, topic.as_str());

            let mut kafka_properties = Properties::new();
            kafka_properties.set_str(BOOTSTRAP_SERVERS, brokers.as_str());

            sink_properties.extend_sub_properties(KAFKA, kafka_properties);
            sink_properties
        };

        properties.extend_sink(KAFKA_FN, kafka_sink_properties);

        println!("{}", properties.to_lines_string());
    }

    fn build_stream(&self, properties: &Properties, env: &mut StreamExecutionEnvironment) {
        let sink = KafkaOutputFormatBuilder::try_from(properties.to_sink(KAFKA_FN))
            .unwrap()
            .build();

        env.register_source(vec_source(
            gen_records(),
            Schema::from(&model::FIELD_METADATA),
            3,
        ))
            .flat_map(OutputMapperFunction::new())
            .add_sink(sink);
    }

    async fn pre_worker_startup(&self, _cluster_descriptor: &ClusterDescriptor) {
        todo!()
    }
}