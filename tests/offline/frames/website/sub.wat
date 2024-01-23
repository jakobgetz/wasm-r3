(module
  (func $sub (export "sub") (param i32 i32) (result i32)
    i32.const 0
    table.get $table
    drop
    local.get 1
    local.get 0
    i32.sub
  )
  (table $table (export "table") 1 1 funcref)
  (elem $table (i32.const 0) $sub)
)