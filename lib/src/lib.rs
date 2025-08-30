use wasm_bindgen::prelude::*;
use std::collections::HashSet;
use tf_demo_parser::demo::data::attributes::Attribute;
use tf_demo_parser::demo::data::attributes::has_attribute;
use tf_demo_parser::demo::data::attributes::get_attribute_value;
use tf_demo_parser::demo::data::attributes::AttributeValueType;
use tf_demo_parser::demo::data::DemoTick;
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::parser::MessageHandler;
use tf_demo_parser::demo::sendprop::SendPropValue;
use tf_demo_parser::MessageType;
pub use tf_demo_parser::{Demo, DemoParser, Parse, ParseError, ParserState, Stream};

struct OtherI64 {
    value: i64
}

impl AttributeValueType for OtherI64 {
    fn from_prop_value(value: &SendPropValue) -> Self {
        OtherI64 {
            value: i64::try_from(value).unwrap_or_default()
        }
    }
}

#[derive(Default, Debug)]
struct TextureAnalyser {
    pub state: HashSet<i64>
}

impl MessageHandler for TextureAnalyser {
    type Output = HashSet<i64>;

    fn does_handle(message_type: MessageType) -> bool {
        matches!(
            message_type,
            MessageType::PacketEntities
        )
    }

    fn handle_message(&mut self, message: &Message, _tick: DemoTick, _parser_state: &ParserState) {
        match message {
            Message::PacketEntities(message) => {
                for entity in &message.entities {
                    let has_hi = has_attribute(&entity.props, Attribute::CustomTextureHi);
                    let has_lo = has_attribute(&entity.props, Attribute::CustomTextureLo);

                    if has_hi && has_lo {
                        let hi: Option<OtherI64> = get_attribute_value(&entity.props, Attribute::CustomTextureHi);
                        let lo: Option<OtherI64> = get_attribute_value(&entity.props, Attribute::CustomTextureLo);
                        if let (Some(hi), Some(lo)) = (hi, lo) {
                            let gid: i64 = (hi.value << 32) | (lo.value);
                            self.state.insert(gid);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn into_output(self, _state: &ParserState) -> Self::Output {
        self.state
    }
}

impl TextureAnalyser {
    pub fn new() -> Self {
        Self::default()
    }
}

#[wasm_bindgen]
pub fn parse_demo(bytes: &[u8]) -> Option<Vec<String>> {
    let demo = Demo::new(&bytes);
    let parser = DemoParser::new_with_analyser(demo.get_stream(), TextureAnalyser::new());
    let result = parser.parse();
    if let Ok(ids) = result {
        Some(ids.1.into_iter().map(|id| id.to_string()).collect())
    } else {
        None
    }
}