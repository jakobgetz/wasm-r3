(module
    (import "env" "foo" (func $foo ))
    (memory 1 10)
    (func $main (export "main") (param i32)
        local.get 0
        memory.grow
        drop
        call $foo
    )
)
