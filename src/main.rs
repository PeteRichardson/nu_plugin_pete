/// nushell plugin example from
/// https://www.nushell.sh/contributor-book/plugins.html#creating-a-plugin-in-rust
/// 
/// Here's how to iterate on this code
///
/// 1. cargo install --path .
/// 2. plugin add ~/.cargo/bin/nu_plugin_pete
/// 3. plugin use pete
///
///  "hello" | len
/// 18
///


use nu_plugin::{EvaluatedCall, JsonSerializer, serve_plugin};
use nu_plugin::{EngineInterface, Plugin, PluginCommand, SimplePluginCommand};
use nu_protocol::{LabeledError, Signature, Type, Value};

struct LenPlugin;

impl Plugin for LenPlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(Len),
        ]
    }
}

struct Len;

impl SimplePluginCommand for Len {
    type Plugin = LenPlugin;

    fn name(&self) -> &str {
        "len"
    }

    fn usage(&self) -> &str {
        "calculates the length of its input"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .input_output_type(Type::String, Type::Int)
    }

    fn run(
        &self,
        _plugin: &LenPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let span = input.span();
        match input {
            Value::String { val, .. } => Ok(
                // Added 17 to prove code changes get installed correctly
                Value::int((val.len()+17) as i64, span)
            ),
            _ => Err(
                LabeledError::new("Expected String input from pipeline")
                    .with_label(
                        format!("requires string input; got {}", input.get_type()),
                        call.head,
                    )
            ),
        }
    }
}

fn main() {
    serve_plugin(&LenPlugin, JsonSerializer)
}
