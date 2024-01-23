(module
  (func $add (export "add") (param i32 i32) (result i32)
    i32.const 0
    table.get $table
    drop
    local.get 1
    local.get 0
    i32.add
  )
  (table $table (export "table") 1 1 funcref)
  (elem $table (i32.const 0) $add)
)