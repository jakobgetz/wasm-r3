(module
  (import "env" "changeTable" (func $changeTable))
  (import "env" "a" (func $a))
  (import "env" "b" (func $b))
  (func $main (export "main")
    i32.const 0
    call_indirect 0
    call $changeTable
    i32.const 0
    call_indirect 0
  )
  (table 2 funcref (export "table"))
  (elem (i32.const 0) 0 $a $b)
)
