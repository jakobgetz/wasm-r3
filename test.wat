(module
;; page size of memory is 65536
  (func $main (export "entry") (result i32)
    i32.const 1
    memory.grow
    i32.const -1
    i32.eq
    (if (then unreachable) (else))
    i32.const 85533
    i32.const 3
    i32.store
    i32.const 85533
    i32.load
  )
  (memory (export "memory") 1 10)
)