(module
  (import "env" "changeMem" (func $changeMemHost))
  (func $main (export "entry")
    call $changeMemHost
    i32.const 1
    i32.load
    drop
  )
  (memory (export "memory") 1)
  (data (i32.const 0) "\00\00\00\00")
)
