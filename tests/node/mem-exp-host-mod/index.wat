(module
  (import "env" "changeMem" (func $changeMem))
  (func $main (export "entry")
    i32.const 1
    i32.load
    drop
    call $changeMem
    i32.const 1
    i32.load
    drop
  )
  (memory (export "memory") 1)
)
