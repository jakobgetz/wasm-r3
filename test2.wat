(module
(import "r3" "check_mem" (func $check_mem))
(import "r3" "check_table" (func $check_table))
(type (;0;) (func))
(func (;0;) (type 0)
(local $addr_local i32)(local $i32 i32)(local $i64 i64)(local $f32 f32)(local $f64 f64)(local $funcref funcref)(local)
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
i32.gt_u
(if (then call $check_mem))
i32.const 60000.004
global.get $table_pointer
i32.gt_u
(if (then call $check_table))
)
(func (;1;) (type 0)
(local $addr_local i32)(local $i32 i32)(local $i64 i64)(local $f32 f32)(local $f64 f64)(local $funcref funcref)(local)
global.get $mem_pointer
i32.const 2
i32.store8 $trace_mem offset=0
global.get $mem_pointer
i32.const 1
i32.store $trace_mem offset=1
global.get $mem_pointer
i32.const 0
i32.store $trace_mem offset=5
global.get $mem_pointer
i32.const 9
i32.add
global.set $mem_pointer
i32.const 0
global.get $mem_pointer
i32.const 17
i32.store8 $trace_mem offset=0
global.get $mem_pointer
i32.const 0
i32.store $trace_mem offset=1
local.tee $i32
global.get $mem_pointer
local.get $i32
i32.store $trace_mem offset=5
local.get $i32
table.get 0
global.get $table_pointer
local.set $addr_local
local.tee $funcref
global.get $table_pointer
local.get $funcref
table.set $table
global.get $mem_pointer
global.get $table_pointer
i32.store $trace_mem offset=9
global.get $table_pointer
i32.const 1
i32.add
global.set $table_pointer
drop
global.get $mem_pointer
i32.const 13
i32.add
global.set $mem_pointer
call_indirect (type 0)
global.get $mem_pointer
i32.const 254
i32.store8 $trace_mem offset=0
global.get $mem_pointer
local.get $addr_local
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
i32.gt_u
(if (then call $check_mem))
i32.const 60000.004
global.get $table_pointer
i32.gt_u
(if (then call $check_table))
)
(table (;0;) 1 1 funcref)
(export "entry" (func 3))
(elem (;0;) (i32.const 0) func 2)

(memory $trace_mem (export "trace") 10000)
(global $mem_pointer (export "trace_byte_length") (mut i32) (i32.const 0))
(table $table (export "lookup") 100000 funcref)
(global $table_pointer (export "lookup_table_pointer") (mut i32) (i32.const 0))
)