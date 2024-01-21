(module
(type (;0;) (func))
(import "r3" "check_mem" (func $check_mem))
(func (;0;) (type 0)
(local $addr_local i32)(local $i32 i32)(local $i64 i64)(local $f32 f32)(local $f64 f64)(local $funcref funcref)
global.get $mem_pointer
i32.const 2
i32.store8 $trace_mem offset=0
global.get $mem_pointer
i32.const 0
i32.store $trace_mem offset=1
global.get $mem_pointer
i32.const 0
i32.store $trace_mem offset=5
global.get $mem_pointer
i32.const 9
i32.add
global.set $mem_pointer
global.get $mem_pointer
i32.const 15
i32.store8 $trace_mem offset=0
global.get $mem_pointer
i32.const 1
i32.add
global.set $mem_pointer
i32.const 384000000
global.get $mem_pointer
i32.eq
(if (then call $check_mem))
)
(export "entry" (func 0))

(memory $trace_mem (export "trace") 10000)
(global $mem_pointer (export "trace_byte_length") (mut i32) (i32.const 0))
(table $table (export "lookup_table") 100000 funcref)
(global $table_pointer (export "lookup_table_pointer") (mut i32) (i32.const 0))
)