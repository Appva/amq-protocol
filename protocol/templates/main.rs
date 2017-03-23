use types::*;

pub const NAME:          &'static str   = "{{protocol.name}}";
pub const MAJOR_VERSION: ShortShortUInt = {{protocol.major_version}};
pub const MINOR_VERSION: ShortShortUInt = {{protocol.minor_version}};
pub const REVISION:      ShortShortUInt = {{protocol.revision}};
pub const PORT:          LongUInt       = {{protocol.port}};
pub const COPYRIGHT:     &'static str   = r#"{{copyright}}"#;

{{#each protocol.constants as |constant| ~}}
pub const {{sanitize_name constant.name}}: {{constant.type}} = {{constant.value}};
{{/each ~}}

pub enum AMQPError {
    Soft(AMQPSoftError),
    Hard(AMQPHardError),
}

impl AMQPError {
    pub fn get_id(&self) -> ShortUInt {
        match *self {
            AMQPError::Soft(ref s) => s.get_id(),
            AMQPError::Hard(ref h) => h.get_id(),
        }
    }

    pub fn from_id(id: ShortUInt) -> Option<AMQPError> {
        AMQPSoftError::from_id(id).map(|e| AMQPError::Soft(e)).or_else(|| AMQPHardError::from_id(id).map(|e| AMQPError::Hard(e)))
    }
}

pub enum AMQPSoftError {
    {{#each protocol.soft_errors as |constant| ~}}
    {{camel constant.name}},
    {{/each ~}}
}

impl AMQPSoftError {
    pub fn get_id(&self) -> ShortUInt {
        match *self {
            {{#each protocol.soft_errors as |constant| ~}}
            AMQPSoftError::{{camel constant.name}} => {{constant.value}},
            {{/each ~}}
        }
    }

    pub fn from_id(id: ShortUInt) -> Option<AMQPSoftError> {
        match id {
            {{#each protocol.soft_errors as |constant| ~}}
            {{constant.value}} => Some(AMQPSoftError::{{camel constant.name}}),
            {{/each ~}}
            _                  => None,
        }
    }
}

pub enum AMQPHardError {
    {{#each protocol.hard_errors as |constant| ~}}
    {{camel constant.name}},
    {{/each ~}}
}

impl AMQPHardError {
    pub fn get_id(&self) -> ShortUInt {
        match *self {
            {{#each protocol.hard_errors as |constant| ~}}
            AMQPHardError::{{camel constant.name}} => {{constant.value}},
            {{/each ~}}
        }
    }

    pub fn from_id(id: ShortUInt) -> Option<AMQPHardError> {
        match id {
            {{#each protocol.hard_errors as |constant| ~}}
            {{constant.value}} => Some(AMQPHardError::{{camel constant.name}}),
            {{/each ~}}
            _                  => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AMQPClass {
    {{#each protocol.classes as |class| ~}}
    {{camel class.name}}({{snake class.name}}::Methods),
    {{/each ~}}
}

{{#each protocol.classes as |class|}}
pub mod {{snake class.name}} {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Methods {
        {{#each class.methods as |method| ~}}
        {{camel method.name}}({{camel method.name}}),
        {{/each ~}}
    }

    {{#each class.methods as |method|}}
    #[derive(Clone, Debug, PartialEq)]
    pub struct {{camel method.name}} {
        {{#each_argument method.arguments as |argument| ~}}
        {{#if argument_is_value ~}}
        pub {{snake argument.name}}: {{argument.type}},
        {{else}}
        {{#each_flag argument as |flag| ~}}
        pub {{snake flag.name}}: Boolean,
        {{/each_flag ~}}
        {{/if ~}}
        {{/each_argument ~}}
    }
    {{/each ~}}
}
{{/each ~}}
