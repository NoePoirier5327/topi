//! Module de génération de stubs pour le module lua.

#[macro_export]
macro_rules! impl_lua_class_stub {
    (
        class $class_name:ident {
            $(fields {
                $($field_name:ident: $field_ty:expr),* $(,)?
            })?
            $(methods {
                $(
                    fn $fn_name:ident($($arg:ident: $arg_ty:expr),*) $(-> $ret_ty:expr)?;
                )*
            })?
        }
    ) => {
        impl $class_name {
            pub fn generate_stub() -> String {
                let mut stub = String::new();
                stub.push_str(&format!("---@class {}\n", stringify!($class_name)));

                // 1. Génération des champs (---@field)
                $($(
                    stub.push_str(&format!("---@field {} {}\n", stringify!($field_name), $field_ty));
                )*)?

                stub.push_str(&format!("local {} = {{}}\n\n", stringify!($class_name)));

                // 2. Génération des méthodes (functions)
                $($(
                    let mut args_vec: Vec<&str> = Vec::new();
                    $(
                        stub.push_str(&format!("---@param {} {}\n", stringify!($arg), $arg_ty));
                        args_vec.push(stringify!($arg));
                    )*
                    $(
                        stub.push_str(&format!("---@return {}\n", $ret_ty));
                    )?

                    let args_str = args_vec.join(", ");
                    stub.push_str(&format!("function {}:{}({}) end\n\n", stringify!($class_name), stringify!($fn_name), args_str));
                )*)?

                stub
            }
        }
    };
}

#[macro_export]
macro_rules! impl_lua_enum_stub {
    (
        enum $enum_name:ident {
            $($variant:ident),* $(,)?
        }
    ) => {
        impl $enum_name {
            pub fn generate_stub() -> String {
                let mut stub = String::new();
                stub.push_str(&format!("---@enum {}\n", stringify!($enum_name)));
                stub.push_str(&format!("local {} = {{\n", stringify!($enum_name)));
                $(
                    stub.push_str(&format!("    {} = \"{}\",\n", stringify!($variant), stringify!($variant)));
                )*
                stub.push_str("}\n\n");
                stub
            }
        }
    };
}
