(module
  (func $div (export "div") (param f32 f32) (result f32)
    i32.const 0
    table.get $table
    drop
    local.get 1
    local.get 0
    f32.div
  )
  (table $table (export "table") 1 1 funcref)
  (elem $table (i32.const 0) $div)
)