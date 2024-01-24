(module
  (func $main (export "entry") (result i32)
    i32.const 0
    i32.load offset=4
  )
  (memory (export "memory") 1)
)
