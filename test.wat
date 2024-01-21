(module
    (import "env" "foo" (func (type 0)))
    (type (func))
    (global $global (mut i32) (i32.const 0))
    (func (param i32 i32 i64 i32 f32 f32 i64)
    (local $i32 i32)(local $i64 i64)(local $f32 f32)(local $f64 f64)
    (local f64 f64 funcref)
        i32.const 1
        i32.const 1
        i32.eq
        (if (then i32.const 1 drop))
        call $lol
        local.get $i32
        i32.const 1
        i32.store align=1
        i32.const 0
        (call_indirect $table (type 0))
        global.get $global
        drop
    )
    (func $lol)
    (memory 1 1)
    (table 1 1 funcref)
    (table $table 1 1 funcref)
    (elem (table $table) (i32.const 0) $lol)
)