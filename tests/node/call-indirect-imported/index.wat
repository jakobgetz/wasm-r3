(module
    (import "env" "foo" (func $foo))
    (import "env" "bar" (func $bar))
    (import "env" "baz" (func $baz))
    (import "env" "mem" (memory 1))
    (func $main (export "entry")
        i32.const 0
        call_indirect
        i32.const 1
        call_indirect
        i32.const 2
        call_indirect
        i32.const 1
        i32.load
        drop
    )
    (func (export "reentry") 
        i32.const 0
        i32.load
        drop
    )
    (table 3 3 funcref)
    (elem (i32.const 0) $foo $bar $baz)
)