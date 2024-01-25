(module
;; page size of memory is 65536
  (func $main (export "entry") (result i32)
    i32.const 65520
    i32.const 1
    i32.add
    i64.const 1
    ;; i64.store align=8
    i64.store align=8
    i32.const 0
  )
  (memory (export "memory") 1 1)
)