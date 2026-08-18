//! Module de génération de stubs pour le module lua.

#[macro_export]
macro_rules! impl_lua_stub {
    (
        class $class_name:ident {
            $(
                fn $fn_name:ident($($arg:ident: $arg_ty:expr),*) $(-> $ret_ty:expr)?;
            )*
        }
    ) => {
        impl $class_name {
            pub fn generate_stub() -> String {
                let mut stub = String::new();
                stub.push_str(&format!("---@class {}\n", stringify!($class_name)));
                stub.push_str(&format!("local {} = {{}}\n\n", stringify!($class_name)));

                $(
                    // On type explicitement Vec<&str> pour éviter l'erreur E0282 quand il n'y a pas d'arguments
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
                )*

                //stub.push_str(&format!("return {}\n", stringify!($class_name)));
                stub
            }
        }
    };
}
