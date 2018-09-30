macro_rules! next {
    (empty) => {};

    ((peel, $callback:tt, ($value:tt))) => {
        $callback!( empty => );
    };

    ((peel, $callback:tt, ($value:tt, $($other:tt),+))) => {
        $callback!( (peel, $callback, ($($other),+)) => $($other),+ );
    };
}

macro_rules! foreach {
    ($callback:tt => $($values:tt),*) => {
        $callback!( (peel, $callback, ($($values),*)) => $($values),* );
    };
}

macro_rules! loop_through_identifiers {
    ($callback:tt) => {
        foreach!( $callback => A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12 );
    };
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __js_raw_asm {
    ($code:expr, $($arg:expr),*) => {
        {
            #[allow(unused_unsafe)]
            unsafe {
                const CODE: &'static str = concat!( $code, "\0" );
                $crate::private::emscripten_asm_const_int(
                    CODE as *const _ as *const u8,
                    $($arg),*
                )
            }
        }
    };

    ($code:expr) => { __js_raw_asm!( $code, ) };
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __js_raw_asm {
    ($code:expr, ) => {{
        #[allow(unused_unsafe)]
        unsafe {
            $crate::private::__js_0( concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    // Autogenerated with the following Ruby script:
    // args, params = [], []
    // 1.upto( 17 ) do |nth|
    //     args << "$a#{nth}:expr"
    //     params << "$a#{nth} as i32"
    //     puts "($code:expr, #{args.join ', '}) => {{"
    //     puts "    #[allow(unused_unsafe)]"
    //     puts "    unsafe {"
    //     puts "        #[allow(trivial_numeric_casts)]"
    //     puts "        $crate::private::__js_#{nth}( #{params.join ', '}, concat!( $code, \"\\0\" ) as *const _ as *const u8 )"
    //     puts "    }"
    //     puts "}};"
    // end
    ($code:expr, $a1:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_1( $a1 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_2( $a1 as i32, $a2 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_3( $a1 as i32, $a2 as i32, $a3 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_4( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_5( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, $a5 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_6( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, $a5 as i32, $a6 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_7( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, $a5 as i32, $a6 as i32, $a7 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_8( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, $a5 as i32, $a6 as i32, $a7 as i32, $a8 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_9( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, $a5 as i32, $a6 as i32, $a7 as i32, $a8 as i32, $a9 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_10( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, $a5 as i32, $a6 as i32, $a7 as i32, $a8 as i32, $a9 as i32, $a10 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_11( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, $a5 as i32, $a6 as i32, $a7 as i32, $a8 as i32, $a9 as i32, $a10 as i32, $a11 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_12( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, $a5 as i32, $a6 as i32, $a7 as i32, $a8 as i32, $a9 as i32, $a10 as i32, $a11 as i32, $a12 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr, $a13:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_13( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, $a5 as i32, $a6 as i32, $a7 as i32, $a8 as i32, $a9 as i32, $a10 as i32, $a11 as i32, $a12 as i32, $a13 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_14( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, $a5 as i32, $a6 as i32, $a7 as i32, $a8 as i32, $a9 as i32, $a10 as i32, $a11 as i32, $a12 as i32, $a13 as i32, $a14 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr, $a15:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_15( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, $a5 as i32, $a6 as i32, $a7 as i32, $a8 as i32, $a9 as i32, $a10 as i32, $a11 as i32, $a12 as i32, $a13 as i32, $a14 as i32, $a15 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr, $a15:expr, $a16:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_16( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, $a5 as i32, $a6 as i32, $a7 as i32, $a8 as i32, $a9 as i32, $a10 as i32, $a11 as i32, $a12 as i32, $a13 as i32, $a14 as i32, $a15 as i32, $a16 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};
    ($code:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr, $a15:expr, $a16:expr, $a17:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            #[allow(trivial_numeric_casts)]
            $crate::private::__js_17( $a1 as i32, $a2 as i32, $a3 as i32, $a4 as i32, $a5 as i32, $a6 as i32, $a7 as i32, $a8 as i32, $a9 as i32, $a10 as i32, $a11 as i32, $a12 as i32, $a13 as i32, $a14 as i32, $a15 as i32, $a16 as i32, $a17 as i32, concat!( $code, "\0" ) as *const _ as *const u8 )
        }
    }};

    ($code:expr) => { __js_raw_asm!( $code, ) };
}

// Abandon all hope, ye who enter here!
//
// If there was a contest for the ugliest and most hacky macro ever written,
// I would enter this one.
//
// There is probably a more clever way to write this macro, but oh well.
#[doc(hidden)]
#[macro_export]
macro_rules! _js_impl {
    (@_inc @$cmd:tt "0" $($rest:tt)*) => { _js_impl!( @$cmd "1" $($rest)* ) };
    (@_inc @$cmd:tt "1" $($rest:tt)*) => { _js_impl!( @$cmd "2" $($rest)* ) };
    (@_inc @$cmd:tt "2" $($rest:tt)*) => { _js_impl!( @$cmd "3" $($rest)* ) };
    (@_inc @$cmd:tt "3" $($rest:tt)*) => { _js_impl!( @$cmd "4" $($rest)* ) };
    (@_inc @$cmd:tt "4" $($rest:tt)*) => { _js_impl!( @$cmd "5" $($rest)* ) };
    (@_inc @$cmd:tt "5" $($rest:tt)*) => { _js_impl!( @$cmd "6" $($rest)* ) };
    (@_inc @$cmd:tt "6" $($rest:tt)*) => { _js_impl!( @$cmd "7" $($rest)* ) };
    (@_inc @$cmd:tt "7" $($rest:tt)*) => { _js_impl!( @$cmd "8" $($rest)* ) };
    (@_inc @$cmd:tt "8" $($rest:tt)*) => { _js_impl!( @$cmd "9" $($rest)* ) };
    (@_inc @$cmd:tt "9" $($rest:tt)*) => { _js_impl!( @$cmd "10" $($rest)* ) };
    (@_inc @$cmd:tt "10" $($rest:tt)*) => { _js_impl!( @$cmd "11" $($rest)* ) };
    (@_inc @$cmd:tt "11" $($rest:tt)*) => { _js_impl!( @$cmd "12" $($rest)* ) };
    (@_inc @$cmd:tt "12" $($rest:tt)*) => { _js_impl!( @$cmd "13" $($rest)* ) };
    (@_inc @$cmd:tt "13" $($rest:tt)*) => { _js_impl!( @$cmd "14" $($rest)* ) };
    (@_inc @$cmd:tt "14" $($rest:tt)*) => { _js_impl!( @$cmd "15" $($rest)* ) };
    (@_inc @$cmd:tt "15" $($rest:tt)*) => { _js_impl!( @$cmd "16" $($rest)* ) };
    (@_inc @$cmd:tt "16" $($rest:tt)*) => { _js_impl!( @$cmd "17" $($rest)* ) };

    (@_stringify $arg_counter:tt [$terminator:tt $($terminator_rest:tt)*] [$($out:tt)*] -> [] $next:tt $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator_rest)*] [$($out)* ($terminator)] -> $next $($rest)* )
    };

    (@_stringify $arg_counter:tt [] [$($out:tt)*] -> []) => {
        concat!( $(concat! $out),* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [($($inner:tt)*) $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [")" $($terminator)*] [$($out)* ("(")] -> [$($inner)*] [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [[$($inner:tt)*] $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter ["]" $($terminator)*] [$($out)* ("[")] -> [$($inner)*] [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [{$($inner:tt)*} $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter ["}" $($terminator)*] [$($out)* ("{")] -> [$($inner)*] [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [@{$arg:expr} $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_inc @_stringify $arg_counter [$($terminator)*] [$($out)* ("($") ($arg_counter) (")")] -> [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [++ $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* ("++")] -> [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [-- $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* ("--")] -> [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [=== $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* ("===")] -> [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [!== $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* ("!==")] -> [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [$token:tt . $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* (stringify!( $token )) (".")] -> [$($remaining)*] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [$token:tt] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* (stringify!( $token ))] -> [] $($rest)* )
    };

    (@_stringify $arg_counter:tt [$($terminator:tt)*] [$($out:tt)*] -> [$token:tt $($remaining:tt)*] $($rest:tt)*) => {
        _js_impl!( @_stringify $arg_counter [$($terminator)*] [$($out)* (stringify!( $token )) (" ")] -> [$($remaining)*] $($rest)* )
    };

    (@stringify [$($flags:tt)*] -> $($rest:tt)*) => {
        _js_impl!( @if no_return in [$($flags)*] {
            _js_impl!( @_stringify "0" [] [] -> [$($rest)*] )
        } else {
            _js_impl!( @_stringify "1" [] [] -> [$($rest)*] )
        })
    };

    (@prelude $arg_counter:tt [$($out:tt)*] -> $arg:tt $($rest:tt)*) => {
        _js_impl!( @_inc @prelude $arg_counter [$($out)* ("$") ($arg_counter) (" = Module.STDWEB_PRIVATE.to_js($") ($arg_counter) (");")] -> $($rest)* )
    };

    (@prelude $arg_counter:tt [$($out:tt)*] ->) => {
        concat!( $(concat! $out),* )
    };

    (@if no_return in [no_return $($rest:tt)*] {$($true_case:tt)*} else {$($false_case:tt)*}) => {
        $($true_case)*
    };

    (@if $condition:tt in [] {$($true_case:tt)*} else {$($false_case:tt)*}) => {
        $($false_case)*
    };

    (@if $condition:tt in [$token:tt $($rest:tt)*] {$($true_case:tt)*} else {$($false_case:tt)*}) => {
        _js_impl!( @if $condition in [$($rest)*] {$($true_case)*} else {$($false_case)*} );
    };

    (@prepare $memory_required:ident [] [$($names:tt)*]) => {};
    (@prepare $memory_required:ident [$arg:tt $($rest_args:tt)*] [$name:tt $($rest_names:tt)*]) => {
        let $name = $arg;
        let $name = $crate::private::IntoNewtype::into_newtype( $name );
        $memory_required += $crate::private::JsSerializeOwned::memory_required_owned( &$name );
        _js_impl!( @prepare $memory_required [$($rest_args)*] [$($rest_names)*] );
    };

    (@serialize $arena:ident [] [$($names:tt)*]) => {};
    (@serialize $arena:ident [$arg:tt $($rest_args:tt)*] [$name:tt $($rest_names:tt)*]) => {
        let mut $name = Some( $name );
        let $name = $crate::private::JsSerializeOwned::into_js_owned( &mut $name, &$arena );
        let $name = &$name as *const _;
        _js_impl!( @serialize $arena [$($rest_args)*] [$($rest_names)*] );
    };

    (@call_emscripten [$code:expr] [] [$($arg_names:tt)*]) => {
        __js_raw_asm!( $code );
    };
    // Autogenerated with the following Ruby script:
    // args, names, params = [], [], []
    // 1.upto( 16 ) do |nth|
    //     args << "$a#{nth}:tt"
    //     names << "$a#{nth}_name:tt"
    //     params << "$a#{nth}_name"
    //     puts "(@call_emscripten [$code:expr] [#{args.join ' '}] [#{names.join ' '} $($arg_names:tt)*]) => {"
    //     puts "    __js_raw_asm!( $code, #{params.join ', '} );"
    //     puts "};"
    // end
    (@call_emscripten [$code:expr] [$a1:tt] [$a1_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt] [$a1_name:tt $a2_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt $a5:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt $a5:tt $a6:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $a6_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name, $a6_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt $a5:tt $a6:tt $a7:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $a6_name:tt $a7_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name, $a6_name, $a7_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt $a5:tt $a6:tt $a7:tt $a8:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $a6_name:tt $a7_name:tt $a8_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name, $a6_name, $a7_name, $a8_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt $a5:tt $a6:tt $a7:tt $a8:tt $a9:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $a6_name:tt $a7_name:tt $a8_name:tt $a9_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name, $a6_name, $a7_name, $a8_name, $a9_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt $a5:tt $a6:tt $a7:tt $a8:tt $a9:tt $a10:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $a6_name:tt $a7_name:tt $a8_name:tt $a9_name:tt $a10_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name, $a6_name, $a7_name, $a8_name, $a9_name, $a10_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt $a5:tt $a6:tt $a7:tt $a8:tt $a9:tt $a10:tt $a11:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $a6_name:tt $a7_name:tt $a8_name:tt $a9_name:tt $a10_name:tt $a11_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name, $a6_name, $a7_name, $a8_name, $a9_name, $a10_name, $a11_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt $a5:tt $a6:tt $a7:tt $a8:tt $a9:tt $a10:tt $a11:tt $a12:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $a6_name:tt $a7_name:tt $a8_name:tt $a9_name:tt $a10_name:tt $a11_name:tt $a12_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name, $a6_name, $a7_name, $a8_name, $a9_name, $a10_name, $a11_name, $a12_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt $a5:tt $a6:tt $a7:tt $a8:tt $a9:tt $a10:tt $a11:tt $a12:tt $a13:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $a6_name:tt $a7_name:tt $a8_name:tt $a9_name:tt $a10_name:tt $a11_name:tt $a12_name:tt $a13_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name, $a6_name, $a7_name, $a8_name, $a9_name, $a10_name, $a11_name, $a12_name, $a13_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt $a5:tt $a6:tt $a7:tt $a8:tt $a9:tt $a10:tt $a11:tt $a12:tt $a13:tt $a14:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $a6_name:tt $a7_name:tt $a8_name:tt $a9_name:tt $a10_name:tt $a11_name:tt $a12_name:tt $a13_name:tt $a14_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name, $a6_name, $a7_name, $a8_name, $a9_name, $a10_name, $a11_name, $a12_name, $a13_name, $a14_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt $a5:tt $a6:tt $a7:tt $a8:tt $a9:tt $a10:tt $a11:tt $a12:tt $a13:tt $a14:tt $a15:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $a6_name:tt $a7_name:tt $a8_name:tt $a9_name:tt $a10_name:tt $a11_name:tt $a12_name:tt $a13_name:tt $a14_name:tt $a15_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name, $a6_name, $a7_name, $a8_name, $a9_name, $a10_name, $a11_name, $a12_name, $a13_name, $a14_name, $a15_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt $a5:tt $a6:tt $a7:tt $a8:tt $a9:tt $a10:tt $a11:tt $a12:tt $a13:tt $a14:tt $a15:tt $a16:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $a6_name:tt $a7_name:tt $a8_name:tt $a9_name:tt $a10_name:tt $a11_name:tt $a12_name:tt $a13_name:tt $a14_name:tt $a15_name:tt $a16_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name, $a6_name, $a7_name, $a8_name, $a9_name, $a10_name, $a11_name, $a12_name, $a13_name, $a14_name, $a15_name, $a16_name );
    };
    (@call_emscripten [$code:expr] [$a1:tt $a2:tt $a3:tt $a4:tt $a5:tt $a6:tt $a7:tt $a8:tt $a9:tt $a10:tt $a11:tt $a12:tt $a13:tt $a14:tt $a15:tt $a16:tt $a17:tt] [$a1_name:tt $a2_name:tt $a3_name:tt $a4_name:tt $a5_name:tt $a6_name:tt $a7_name:tt $a8_name:tt $a9_name:tt $a10_name:tt $a11_name:tt $a12_name:tt $a13_name:tt $a14_name:tt $a15_name:tt $a16_name:tt $a17_name:tt $($arg_names:tt)*]) => {
        __js_raw_asm!( $code, $a1_name, $a2_name, $a3_name, $a4_name, $a5_name, $a6_name, $a7_name, $a8_name, $a9_name, $a10_name, $a11_name, $a12_name, $a13_name, $a14_name, $a15_name, $a16_name, $a17_name );
    };
    //

    (@call [$code:expr, [$($flags:tt)*]] [$($args:tt)*] ->) => {
        // It'd be nice to put at least some of this inside a function inside the crate,
        // but then it wouldn't work (I tried!) as the string with the code wouldn't be
        // passed as a direct reference to a constant, and Emscripten needs that to actually
        // use the JavaScript code we're passing to it.
        {
            if cfg!( test ) {
                $crate::initialize();
            }

            let mut memory_required = 0;
            _js_impl!( @prepare memory_required [$($args)*] [a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15] );

            #[allow(unused_variables)]
            let arena = $crate::private::PreallocatedArena::new( memory_required );

            _js_impl!( @serialize arena [$($args)*] [a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15] );
            arena.assert_no_free_space_left();

            $crate::private::noop( &mut memory_required );

            #[allow(unused_unsafe, unused_parens)]
            unsafe {
                _js_impl!(
                    @if no_return in [$($flags)*] {
                        _js_impl!(
                            @call_emscripten
                            [concat!( _js_impl!( @prelude "0" [] -> $($args)* ), $code )]
                            [$($args)*]
                            [a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15]
                        );
                    } else {{
                        let mut result: $crate::private::SerializedValue = ::std::default::Default::default();
                        _js_impl!(
                            @call_emscripten
                            [concat!(
                                _js_impl!( @prelude "1" [] -> $($args)* ),
                                "Module.STDWEB_PRIVATE.from_js($0, (function(){", $code, "})());"
                            )]
                            [RESULT $($args)*]
                            [(&mut result as *mut _) a0 a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15]
                        );
                        result.deserialize()
                    }}
                )
            }
        }
    };

    (@call [$code:expr, [$($flags:tt)*]] [$($args:tt)*] -> { $($inner:tt)* } $($rest:tt)*) => {
        _js_impl!( @call [$code, [$($flags)*]] [$($args)*] -> $($inner)* $($rest)* );
    };

    (@call [$code:expr, [$($flags:tt)*]] [$($args:tt)*] -> ( $($inner:tt)* ) $($rest:tt)*) => {
        _js_impl!( @call [$code, [$($flags)*]] [$($args)*] -> $($inner)* $($rest)* );
    };

    (@call [$code:expr, [$($flags:tt)*]] [$($args:tt)*] -> [ $($inner:tt)* ] $($rest:tt)*) => {
        _js_impl!( @call [$code, [$($flags)*]] [$($args)*] -> $($inner)* $($rest)* );
    };

    (@call [$code:expr, [$($flags:tt)*]] [$($args:tt)*] -> @{$arg:expr} $($rest:tt)*) => {
        _js_impl!( @call [$code, [$($flags)*]] [$($args)* $arg] -> $($rest)* );
    };

    (@call [$code:expr, [$($flags:tt)*]] [$($args:tt)*] -> $token:tt $($rest:tt)*) => {
        _js_impl!( @call [$code, [$($flags)*]] [$($args)*] -> $($rest)* );
    };
}

/// Embeds JavaScript code into your Rust program.
///
/// This macro supports normal JavaScript syntax, albeit with a few limitations:
///
///   * String literals delimited with `'` are not supported.
///   * Semicolons are always required.
///   * The macro will hit the default recursion limit pretty fast, so you'll
///     probably want to increase it with `#![recursion_limit="500"]`.
///     (This is planned to be fixed once procedural macros land in stable Rust.)
///   * Any callbacks passed into JavaScript will **leak memory** by default!
///     You need to call `.drop()` on the callback from the JavaScript side to free it.
///
/// You can pass Rust expressions into the JavaScript code with `@{...expr...}`.
/// The value returned by this macro is an instance of [Value](enum.Value.html).
///
/// # Examples
///
/// ## Regular Usage
///
/// ```
/// let name = "Bob";
/// let result = js! {
///     console.log( "Hello " + @{name} + "!" );
///     return 2 + 2;
/// };
///
/// println!( "2 + 2 = {:?}", result );
/// ```
///
/// Note: you **must** include the `return ...;` statement to get a value.
///
/// ## No Return
///
/// If you don't need to return a value from your snippet you can add a @(no_return) attribute to
/// slightly improve performance.
///
/// ```
/// let name = "Bob";
/// js! { @(no_return)
///     console.log( "Hello " + @{name} + "!" );
/// };
/// ```
#[macro_export]
macro_rules! js {
    (@($($flags:tt),*) $($token:tt)*) => {
        _js_impl!( @call [_js_impl!( @stringify [$($flags)*] -> $($token)* ), [$($flags)*]] [] -> $($token)* )
    };

    ($($token:tt)*) => {
        _js_impl!( @call [_js_impl!( @stringify [] -> $($token)* ), []] [] -> $($token)* )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __js_serializable_boilerplate {
    ($kind:tt) => {
        __js_serializable_boilerplate!( () ($kind) () );
    };

    (impl< $($impl_arg:tt),* > for $kind:ty where $($bounds:tt)*) => {
        __js_serializable_boilerplate!( ($($impl_arg),*) ($kind) ($($bounds)*) );
    };

    (impl< $($impl_arg:tt),* > for $kind:ty) => {
        __js_serializable_boilerplate!( ($($impl_arg),*) ($kind) () );
    };

    (($($impl_arg:tt)*) ($($kind_arg:tt)*) ($($bounds:tt)*)) => {
        impl< $($impl_arg)* > $crate::private::JsSerializeOwned for $($kind_arg)* where $($bounds)* {
            #[inline]
            fn into_js_owned< '_a >( value: &'_a mut Option< Self >, arena: &'_a $crate::private::PreallocatedArena ) -> $crate::private::SerializedValue< '_a > {
                $crate::private::JsSerialize::_into_js( value.as_ref().unwrap(), arena )
            }

            #[inline]
            fn memory_required_owned( &self ) -> usize {
                $crate::private::JsSerialize::_memory_required( self )
            }
        }

        impl< '_r, $($impl_arg)* > $crate::private::JsSerializeOwned for &'_r $($kind_arg)* where $($bounds)* {
            #[inline]
            fn into_js_owned< '_a >( value: &'_a mut Option< Self >, arena: &'_a $crate::private::PreallocatedArena ) -> $crate::private::SerializedValue< '_a > {
                $crate::private::JsSerialize::_into_js( value.unwrap(), arena )
            }

            #[inline]
            fn memory_required_owned( &self ) -> usize {
                $crate::private::JsSerialize::_memory_required( *self )
            }
        }
    };
}

macro_rules! error_boilerplate {
    ($type_name:ident) => {
        impl ::std::fmt::Display for $type_name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "{}: {}", stringify!($type_name), self.message())
            }
        }

        impl ::std::error::Error for $type_name {
            fn description(&self) -> &str {
                stringify!($type_name)
            }
        }
    };

    ($type_name:ident, name = $error_name:expr) => {
        impl ::InstanceOf for $type_name {
            #[inline]
            fn instance_of( reference: &Reference ) -> bool {
                __js_raw_asm!(
                    concat!(
                        "var r = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );",
                        "return (r instanceof DOMException) && (r.name === \"", $error_name, "\");"
                    ),
                    reference.as_raw()
                ) == 1
            }
        }

        error_boilerplate!( $type_name );
    };
}

macro_rules! instanceof {
    ($value:expr, $kind:ident) => {{
        use $crate::unstable::TryInto;
        let reference: Option< &$crate::Reference > = (&$value).try_into().ok();
        reference.map( |reference| {
            __js_raw_asm!(
                concat!( "return (Module.STDWEB_PRIVATE.acquire_js_reference( $0 ) instanceof ", stringify!( $kind ), ") | 0;" ),
                reference.as_raw()
            ) == 1
        }).unwrap_or( false )
    }};
}

macro_rules! newtype_enum {
    ($name:ident {
        $(
            $(#[$attr:meta])*
            $variant:ident = $value:expr
        ),* $(,)*
    }) => {
        impl $name {
            $(
                $(#[$attr])*
                pub const $variant: $name = $name($value);
            )*
        }
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self.0 {
                    $($value => write!(formatter, "{}::{}", stringify!($name), stringify!($variant)),)*
                    other => write!(formatter, "{}({})", stringify!($name), other)
                }
            }
        }
    }
}

/// Embeds JavaScript code into your Rust program similar to the `js!` macro, but
/// catches errors that may be thrown.
///
/// This macro will attempt to coerce the value into the inferred `Result` type.
/// The success and error types should implement `TryFrom<Value>`.
///
/// # Examples
///
/// ```
/// let result: Result<i32, String> = js_try! {
///     throw "error";
/// }.unwrap();
/// assert_eq!(result, Err("error".to_string()));
/// ```
macro_rules! js_try {
    (@(no_return) $($token:tt)*) => {{
        let result = js! {
            try {
                $($token)*
                return {
                    success: true
                };
            } catch( error ) {
                return {
                    error: error,
                    success: false
                };
            }
        };

        use ::webcore::try_from::TryInto;
        if js!( return @{result.as_ref()}.success; ) == true {
            Ok(Ok(()))
        } else {
            match js!( return @{result}.error; ).try_into() {
                Ok(e) => Ok(Err(e)),
                Err(e) => Err(e),
            }
        }
    }};

    ($($token:tt)*) => {{
        let result = js! {
            try {
                return {
                    value: function() { $($token)* }(),
                    success: true
                };
            } catch( error ) {
                return {
                    error: error,
                    success: false
                };
            }
        };

        use webcore::try_from::TryInto;
        if js!( return @{result.as_ref()}.success; ) == true {
            match js!( return @{result}.value; ).try_into() {
                Ok(t) => Ok(Ok(t)),
                Err(e) => Err(e),
            }
        } else {
            match js!( return @{result}.error; ).try_into() {
                Ok(e) => Ok(Err(e)),
                Err(e) => Err(e),
            }
        }
    }};
}

macro_rules! error_enum_boilerplate {
    ($( #[ $error_meta:meta ] )* $error_name:ident, $( $( #[ $variant_meta:meta ] )* $variant:ident),*) => {
        $( #[ $error_meta ] )*
        #[derive(Debug, Clone)]
        pub enum $error_name {
            $(
                $( #[ $variant_meta ] )*
                $variant($variant)
            ),*
        }

        impl TryFrom<::webcore::value::Value> for $error_name {
            type Error = ::webcore::value::ConversionError;

            fn try_from(value: ::webcore::value::Value) -> Result<Self, Self::Error> {
                $(
                    if let Ok(v) = $variant::try_from(value.clone()) {
                        return Ok($error_name::$variant(v));
                    }
                )*

                Err(::webcore::value::ConversionError::type_mismatch( &value ))
            }
        }

        impl ::std::fmt::Display for $error_name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $($error_name::$variant( ref r ) => r.fmt(formatter),)*
                }
            }
        }

        impl ::std::error::Error for $error_name {
            fn description(&self) -> &str {
                stringify!($error_name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    macro_rules! stringify_js {
        ($($token:tt)*) => {
            _js_impl!( @stringify [] -> $($token)* )
        };
    }

    #[test]
    fn stringify() {
        assert_eq!( stringify_js! { console.log }, "console.log" );
        assert_eq!( stringify_js! { 1.0 }, "1.0" );
        assert_eq!( stringify_js! { [ 1.0 ] }, "[1.0]" );
        assert_eq!( stringify_js! { { 1.0 } }, "{1.0}" );
        assert_eq!( stringify_js! { ( 1.0 ) }, "(1.0)" );
        assert_eq!( stringify_js! { a b }, "a b" );
        assert_eq!( stringify_js! { === }, "===" );
        assert_eq!( stringify_js! { ++i }, "++i" );
        assert_eq!( stringify_js! { i++ }, "i ++" );
        assert_eq!( stringify_js! { --i }, "--i" );
        assert_eq!( stringify_js! { i-- }, "i --" );
        assert_eq!( stringify_js! { ( @{1} ); }.replace( " ", "" ), "(($1));" );
        assert_eq!(
            stringify_js! {
                console.log( "Hello!", @{1234i32} );
            }.replace( " ", "" ),
            "console.log(\"Hello!\",($1));"
        );
        assert_eq!(
            stringify_js! {
                @{a}.fn( @{b} );
            }.replace( " ", "" ),
            "($1).fn(($2));"
        );
    }

    #[test]
    fn js_try() {
        use ::webcore::value::{ConversionError, Value};

        let v: Result<Value, Value> = js_try!( return "test"; ).unwrap();
        assert_eq!( v, Ok(Value::String("test".to_string())) );

        let v: Result<bool, bool> = js_try!( return true; ).unwrap();
        assert_eq!( v, Ok(true) );

        let v: Result<bool, bool> = js_try!( throw true; ).unwrap();
        assert_eq!( v, Err(true) );

        let v: Result<i32, String> = js_try!( throw "error"; ).unwrap();
        assert_eq!( v, Err("error".to_string()) );

        let v: Result<(), String> = js_try!( @(no_return) 2+2; ).unwrap();
        assert_eq!( v, Ok(()) );

        let v: Result<(), f64> = js_try!( @(no_return) throw 3.3; ).unwrap();
        assert_eq!( v, Err(3.3) );

        let v: Result< Result<i32, i32>, _ > = js_try!( return "f"; );
        match v {
            Err(ConversionError::TypeMismatch { actual_type: _ }) => (),
            _ => panic!("Expected ConversionError::TypeMistmatch, got {:?}", v),
        }

        let v: Result< Result<i32, i32>, _ > = js_try!( throw "Broken"; );
        match v {
            Err(ConversionError::TypeMismatch { actual_type: _ }) => (),
            _ => panic!("Expected ConversionError::TypeMistmatch, got {:?}", v),
        }
    }
}