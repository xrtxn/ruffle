use super::{AvmString, AvmStringRepr};
use gc_arena::{Collect, Gc};

#[allow(non_snake_case)]
#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct CommonStrings<'gc> {
    pub str_access: AvmString<'gc>,
    pub str_accessors: AvmString<'gc>,
    pub str_advanced: AvmString<'gc>,
    pub str_asyncError: AvmString<'gc>,
    pub str_bases: AvmString<'gc>,
    pub str_block: AvmString<'gc>,
    pub str_bold: AvmString<'gc>,
    pub str_boldItalic: AvmString<'gc>,
    pub str_boolean: AvmString<'gc>,
    pub str_callee: AvmString<'gc>,
    pub str_center: AvmString<'gc>,
    pub str_click: AvmString<'gc>,
    pub str_constructor: AvmString<'gc>,
    pub str_device: AvmString<'gc>,
    pub str_declaredBy: AvmString<'gc>,
    pub str_doubleClick: AvmString<'gc>,
    pub str_dynamic: AvmString<'gc>,
    pub str_embedded: AvmString<'gc>,
    pub str_embeddedCFF: AvmString<'gc>,
    pub str_false: AvmString<'gc>,
    pub str_function: AvmString<'gc>,
    pub str_height: AvmString<'gc>,
    pub str_httpStatus: AvmString<'gc>,
    pub str_Infinity: AvmString<'gc>,
    pub str_inline: AvmString<'gc>,
    pub str_input: AvmString<'gc>,
    pub str_interfaces: AvmString<'gc>,
    pub str_ioError: AvmString<'gc>,
    pub str_isDynamic: AvmString<'gc>,
    pub str_isFinal: AvmString<'gc>,
    pub str_isStatic: AvmString<'gc>,
    pub str_italic: AvmString<'gc>,
    pub str_justify: AvmString<'gc>,
    pub str_key: AvmString<'gc>,
    pub str_left: AvmString<'gc>,
    pub str_localName: AvmString<'gc>,
    pub str_metadata: AvmString<'gc>,
    pub str_methods: AvmString<'gc>,
    pub str_middleClick: AvmString<'gc>,
    pub str_middleMouseDown: AvmString<'gc>,
    pub str_middleMouseUp: AvmString<'gc>,
    pub str_mouseDown: AvmString<'gc>,
    pub str_mouseMove: AvmString<'gc>,
    pub str_mouseOut: AvmString<'gc>,
    pub str_mouseOver: AvmString<'gc>,
    pub str_mouseUp: AvmString<'gc>,
    pub str_mouseWheel: AvmString<'gc>,
    pub str_name: AvmString<'gc>,
    pub str_NaN: AvmString<'gc>,
    pub str_netStatus: AvmString<'gc>,
    pub str_none: AvmString<'gc>,
    pub str_normal: AvmString<'gc>,
    pub str_null: AvmString<'gc>,
    pub str_number: AvmString<'gc>,
    pub str_object: AvmString<'gc>,
    pub str_optional: AvmString<'gc>,
    pub str_parameters: AvmString<'gc>,
    pub str_pixel: AvmString<'gc>,
    pub str_prefix: AvmString<'gc>,
    pub str_readonly: AvmString<'gc>,
    pub str_readwrite: AvmString<'gc>,
    pub str_regular: AvmString<'gc>,
    pub str_releaseOutside: AvmString<'gc>,
    pub str_returnType: AvmString<'gc>,
    pub str_right: AvmString<'gc>,
    pub str_rightClick: AvmString<'gc>,
    pub str_rightMouseDown: AvmString<'gc>,
    pub str_rightMouseUp: AvmString<'gc>,
    pub str_rollOut: AvmString<'gc>,
    pub str_rollOver: AvmString<'gc>,
    pub str_status: AvmString<'gc>,
    pub str_string: AvmString<'gc>,
    pub str_subpixel: AvmString<'gc>,
    pub str_toJSON: AvmString<'gc>,
    pub str_toString: AvmString<'gc>,
    pub str_traits: AvmString<'gc>,
    pub str_true: AvmString<'gc>,
    pub str_type: AvmString<'gc>,
    pub str_undefined: AvmString<'gc>,
    pub str_uri: AvmString<'gc>,
    pub str_value: AvmString<'gc>,
    pub str_valueOf: AvmString<'gc>,
    pub str_variables: AvmString<'gc>,
    pub str_width: AvmString<'gc>,
    pub str_writeonly: AvmString<'gc>,
    pub str_x: AvmString<'gc>,
    pub str_xml: AvmString<'gc>,
    pub str_y: AvmString<'gc>,
}

impl<'gc> CommonStrings<'gc> {
    pub fn new<F>(mut intern_atom_from_static: F) -> Self
    where
        F: for<'a> FnMut(&'static [u8]) -> Gc<'gc, AvmStringRepr<'gc>>,
    {
        let mut intern_from_static = |s: &'static [u8]| intern_atom_from_static(s).into();

        Self {
            str_access: intern_from_static(b"access"),
            str_accessors: intern_from_static(b"accessors"),
            str_advanced: intern_from_static(b"advanced"),
            str_asyncError: intern_from_static(b"asyncError"),
            str_bases: intern_from_static(b"bases"),
            str_block: intern_from_static(b"block"),
            str_bold: intern_from_static(b"bold"),
            str_boldItalic: intern_from_static(b"boldItalic"),
            str_boolean: intern_from_static(b"boolean"),
            str_callee: intern_from_static(b"callee"),
            str_center: intern_from_static(b"center"),
            str_click: intern_from_static(b"click"),
            str_constructor: intern_from_static(b"constructor"),
            str_declaredBy: intern_from_static(b"declaredBy"),
            str_device: intern_from_static(b"device"),
            str_doubleClick: intern_from_static(b"doubleClick"),
            str_dynamic: intern_from_static(b"dynamic"),
            str_embedded: intern_from_static(b"embedded"),
            str_embeddedCFF: intern_from_static(b"embeddedCFF"),
            str_false: intern_from_static(b"false"),
            str_function: intern_from_static(b"function"),
            str_height: intern_from_static(b"height"),
            str_httpStatus: intern_from_static(b"httpStatus"),
            str_Infinity: intern_from_static(b"Infinity"),
            str_inline: intern_from_static(b"inline"),
            str_input: intern_from_static(b"input"),
            str_interfaces: intern_from_static(b"interfaces"),
            str_ioError: intern_from_static(b"ioError"),
            str_isDynamic: intern_from_static(b"isDynamic"),
            str_isFinal: intern_from_static(b"isFinal"),
            str_isStatic: intern_from_static(b"isStatic"),
            str_italic: intern_from_static(b"italic"),
            str_justify: intern_from_static(b"justify"),
            str_key: intern_from_static(b"key"),
            str_left: intern_from_static(b"left"),
            str_localName: intern_from_static(b"localName"),
            str_metadata: intern_from_static(b"metadata"),
            str_methods: intern_from_static(b"methods"),
            str_middleClick: intern_from_static(b"middleClick"),
            str_middleMouseDown: intern_from_static(b"middleMouseDown"),
            str_middleMouseUp: intern_from_static(b"middleMouseUp"),
            str_mouseDown: intern_from_static(b"mouseDown"),
            str_mouseMove: intern_from_static(b"mouseMove"),
            str_mouseOut: intern_from_static(b"mouseOut"),
            str_mouseOver: intern_from_static(b"mouseOver"),
            str_mouseUp: intern_from_static(b"mouseUp"),
            str_mouseWheel: intern_from_static(b"mouseWheel"),
            str_name: intern_from_static(b"name"),
            str_NaN: intern_from_static(b"NaN"),
            str_netStatus: intern_from_static(b"netStatus"),
            str_none: intern_from_static(b"none"),
            str_normal: intern_from_static(b"normal"),
            str_null: intern_from_static(b"null"),
            str_number: intern_from_static(b"number"),
            str_object: intern_from_static(b"object"),
            str_optional: intern_from_static(b"optional"),
            str_parameters: intern_from_static(b"parameters"),
            str_pixel: intern_from_static(b"pixel"),
            str_prefix: intern_from_static(b"prefix"),
            str_readonly: intern_from_static(b"readonly"),
            str_readwrite: intern_from_static(b"readwrite"),
            str_regular: intern_from_static(b"regular"),
            str_releaseOutside: intern_from_static(b"releaseOutside"),
            str_returnType: intern_from_static(b"returnType"),
            str_right: intern_from_static(b"right"),
            str_rightClick: intern_from_static(b"rightClick"),
            str_rightMouseDown: intern_from_static(b"rightMouseDown"),
            str_rightMouseUp: intern_from_static(b"rightMouseUp"),
            str_rollOut: intern_from_static(b"rollOut"),
            str_rollOver: intern_from_static(b"rollOver"),
            str_status: intern_from_static(b"status"),
            str_string: intern_from_static(b"string"),
            str_subpixel: intern_from_static(b"subpixel"),
            str_toJSON: intern_from_static(b"toJSON"),
            str_toString: intern_from_static(b"toString"),
            str_traits: intern_from_static(b"traits"),
            str_true: intern_from_static(b"true"),
            str_type: intern_from_static(b"type"),
            str_undefined: intern_from_static(b"undefined"),
            str_uri: intern_from_static(b"uri"),
            str_value: intern_from_static(b"value"),
            str_valueOf: intern_from_static(b"valueOf"),
            str_variables: intern_from_static(b"variables"),
            str_width: intern_from_static(b"width"),
            str_writeonly: intern_from_static(b"writeonly"),
            str_x: intern_from_static(b"x"),
            str_xml: intern_from_static(b"xml"),
            str_y: intern_from_static(b"y"),
        }
    }
}
