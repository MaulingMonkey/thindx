macro_rules! scope {
    //( $literal:expr ) => { let _flame_scope = flame::start_guard($literal); };
    ( $($tt:tt)*    ) => { let _flame_scope = flame::start_guard(format!($($tt)*).replace("\\", "\\\\").replace("\"", "\\\"")); };
}

macro_rules! error   { ( $($tt:tt)* ) => { mmrbi::error!($($tt)*) }}
macro_rules! status  { ( $($tt:tt)* ) => { mmrbi::status!($($tt)*) }}
macro_rules! warning { ( $($tt:tt)* ) => { mmrbi::warning!($($tt)*) }}

macro_rules! fatal {
    ( $($tt:tt)* ) => {{
        $crate::before_exit();
        mmrbi::fatal!($($tt)*);
    }};
}
