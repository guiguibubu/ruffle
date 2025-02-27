//! `RegExp` impl

use crate::avm2::class::Class;
use crate::avm2::error::type_error;
use crate::avm2::method::{Method, NativeMethodImpl, ParamConfig};
use crate::avm2::object::{regexp_allocator, ArrayObject, FunctionObject, Object, TObject};
use crate::avm2::regexp::RegExpFlags;
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::avm2::Multiname;
use crate::avm2::QName;
use crate::avm2::{activation::Activation, array::ArrayStorage};
use crate::string::{AvmString, WString};
use gc_arena::GcCell;

// All of these methods will be defined as both
// AS3 instance methods and methods on the `Array` class prototype.
const PUBLIC_INSTANCE_AND_PROTO_METHODS: &[(&str, NativeMethodImpl)] =
    &[("exec", exec), ("test", test)];

/// Implements `RegExp`'s instance initializer.
pub fn instance_init<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        activation.super_init(this, &[])?;

        if let Some(mut regexp) = this.as_regexp_mut(activation.context.gc_context) {
            let source: AvmString<'gc> = match args.get(0) {
                Some(Value::Undefined) => "".into(),
                Some(Value::Object(Object::RegExpObject(o))) => {
                    if !matches!(args.get(1), Some(Value::Undefined)) {
                        return Err(Error::AvmError(type_error(
                            activation,
                            "Error #1100: Cannot supply flags when constructing one RegExp from another.",
                            1100,
                        )?));
                    }
                    let other = o.as_regexp().unwrap();
                    regexp.set_source(other.source());
                    regexp.set_flags(other.flags());
                    return Ok(Value::Undefined);
                }
                arg => arg
                    .unwrap_or(&Value::String("".into()))
                    .coerce_to_string(activation)?,
            };

            regexp.set_source(source);

            let flag_chars = match args.get(1) {
                Some(Value::Undefined) => "".into(),
                arg => arg
                    .unwrap_or(&Value::String("".into()))
                    .coerce_to_string(activation)?,
            };

            let mut flags = RegExpFlags::empty();
            for c in &flag_chars {
                flags |= match u8::try_from(c) {
                    Ok(b's') => RegExpFlags::DOTALL,
                    Ok(b'x') => RegExpFlags::EXTENDED,
                    Ok(b'g') => RegExpFlags::GLOBAL,
                    Ok(b'i') => RegExpFlags::IGNORE_CASE,
                    Ok(b'm') => RegExpFlags::MULTILINE,
                    _ => continue,
                };
            }

            regexp.set_flags(flags);
        }
    }

    Ok(Value::Undefined)
}

fn class_call<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let this_class = activation.subclass_object().unwrap();

    if args.len() == 1 {
        let arg = args.get(0).cloned().unwrap();
        if arg.as_object().and_then(|o| o.as_regexp_object()).is_some() {
            return Ok(arg);
        }
    }
    return this_class.construct(activation, args).map(|o| o.into());
}

/// Implements `RegExp`'s class initializer.
pub fn class_init<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        let scope = activation.create_scopechain();
        let gc_context = activation.context.gc_context;
        let this_class = this.as_class_object().unwrap();
        let regexp_proto = this_class.prototype();

        for (name, method) in PUBLIC_INSTANCE_AND_PROTO_METHODS {
            regexp_proto.set_string_property_local(
                *name,
                FunctionObject::from_method(
                    activation,
                    Method::from_builtin(*method, name, gc_context),
                    scope,
                    None,
                    Some(this_class),
                )
                .into(),
                activation,
            )?;
            regexp_proto.set_local_property_is_enumerable(gc_context, (*name).into(), false);
        }
    }
    Ok(Value::Undefined)
}

/// Implements `RegExp.dotall`
pub fn dotall<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        if let Some(regexp) = this.as_regexp() {
            return Ok(regexp.flags().contains(RegExpFlags::DOTALL).into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.extended`
pub fn extended<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        if let Some(regexp) = this.as_regexp() {
            return Ok(regexp.flags().contains(RegExpFlags::EXTENDED).into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.global`
pub fn global<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        if let Some(regexp) = this.as_regexp() {
            return Ok(regexp.flags().contains(RegExpFlags::GLOBAL).into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.ignoreCase`
pub fn ignore_case<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        if let Some(regexp) = this.as_regexp() {
            return Ok(regexp.flags().contains(RegExpFlags::IGNORE_CASE).into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.multiline`
pub fn multiline<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        if let Some(regexp) = this.as_regexp() {
            return Ok(regexp.flags().contains(RegExpFlags::MULTILINE).into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.lastIndex`'s getter
pub fn last_index<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        if let Some(re) = this.as_regexp() {
            return Ok(re.last_index().into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.lastIndex`'s setter
pub fn set_last_index<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        if let Some(mut re) = this.as_regexp_mut(activation.context.gc_context) {
            let i = args
                .get(0)
                .unwrap_or(&Value::Undefined)
                .coerce_to_u32(activation)?;
            re.set_last_index(i as usize);
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.source`
pub fn source<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        if let Some(re) = this.as_regexp() {
            return Ok(re.source().into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.exec`
pub fn exec<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        if let Some(mut re) = this.as_regexp_mut(activation.context.gc_context) {
            let text = args
                .get(0)
                .unwrap_or(&Value::Undefined)
                .coerce_to_string(activation)?;

            let (storage, index) = match re.exec(text) {
                Some(matched) => {
                    let substrings = matched
                        .groups()
                        .map(|range| range.map(|r| WString::from(&text[r])));

                    let storage = ArrayStorage::from_iter(substrings.map(|s| match s {
                        None => Value::Undefined,
                        Some(s) => AvmString::new(activation.context.gc_context, s).into(),
                    }));

                    (storage, matched.start())
                }
                None => return Ok(Value::Null),
            };

            let object = ArrayObject::from_storage(activation, storage)?;

            object.set_string_property_local("index", Value::Number(index as f64), activation)?;

            object.set_string_property_local("input", text.into(), activation)?;

            return Ok(object.into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `RegExp.test`
pub fn test<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        if let Some(mut re) = this.as_regexp_mut(activation.context.gc_context) {
            let text = args
                .get(0)
                .unwrap_or(&Value::Undefined)
                .coerce_to_string(activation)?;
            return Ok(re.test(text).into());
        }
    }

    Ok(Value::Undefined)
}

/// Construct `RegExp`'s class.
pub fn create_class<'gc>(activation: &mut Activation<'_, 'gc>) -> GcCell<'gc, Class<'gc>> {
    let mc = activation.context.gc_context;
    let class = Class::new(
        QName::new(activation.avm2().public_namespace, "RegExp"),
        Some(Multiname::new(activation.avm2().public_namespace, "Object")),
        Method::from_builtin_and_params(
            instance_init,
            "<RegExp instance initializer>",
            vec![
                ParamConfig::optional("re", Multiname::any(mc), Value::Undefined),
                ParamConfig::optional("flags", Multiname::any(mc), Value::Undefined),
            ],
            false,
            mc,
        ),
        Method::from_builtin(class_init, "<RegExp class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);
    write.set_instance_allocator(regexp_allocator);
    write.set_call_handler(Method::from_builtin(
        class_call,
        "<RegExp call handler>",
        mc,
    ));

    const PUBLIC_INSTANCE_PROPERTIES: &[(
        &str,
        Option<NativeMethodImpl>,
        Option<NativeMethodImpl>,
    )] = &[
        ("dotall", Some(dotall), None),
        ("extended", Some(extended), None),
        ("global", Some(global), None),
        ("ignoreCase", Some(ignore_case), None),
        ("multiline", Some(multiline), None),
        ("lastIndex", Some(last_index), Some(set_last_index)),
        ("source", Some(source), None),
    ];
    write.define_builtin_instance_properties(
        mc,
        activation.avm2().public_namespace,
        PUBLIC_INSTANCE_PROPERTIES,
    );

    write.define_builtin_instance_methods(
        mc,
        activation.avm2().as3_namespace,
        PUBLIC_INSTANCE_AND_PROTO_METHODS,
    );

    class
}
