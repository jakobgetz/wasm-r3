(module
(import "r3" "check_mem" (func $check_mem))
(import "r3" "check_table" (func $check_table))
(type (;0;) (func (param i32) (result i32)))
(type (;1;) (func))
(type (;2;) (func (param i32)))
(import "env" "log" (func (;0;) (type 0)))
(import "env" "memory" (memory (;0;) 1 1))
(func (;1;) (type 1)
(local $addr_local i32)(local $i32 i32)(local $i64 i64)(local $f32 f32)(local $f64 f64)(local $funcref funcref)(local)
global.get $internal_call
i32.eqz
(if (then
global.get $mem_pointer
i32.const 2
i32.store8 $trace_mem offset=0
global.get $mem_pointer
i32.const 1
i32.store $trace_mem offset=1
global.get $mem_pointer
i32.const 1
i32.store $trace_mem offset=5
global.get $mem_pointer
i32.const 9
i32.add
global.set $mem_pointer
) (else))
i32.const 42
i32.const 0
global.get $mem_pointer
i32.const 40
i32.store8 $trace_mem offset=0
i32.const 4
i32.add
local.tee $i32
global.get $mem_pointer
local.get $i32
i32.store $trace_mem offset=1
i32.load
local.tee $i32
global.get $mem_pointer
local.get $i32
i32.store $trace_mem offset=5
global.get $mem_pointer
i32.const 9
i32.add
global.set $mem_pointer
i32.const 0
global.set $internal_call
global.get $mem_pointer
i32.const 16
i32.store8 $trace_mem offset=0
global.get $mem_pointer
i32.const 0
i32.store $trace_mem offset=1
global.get $mem_pointer
i32.const 5
i32.add
global.set $mem_pointer
call 2
global.get $mem_pointer
i32.const 255
i32.store8 $trace_mem offset=0
global.get $mem_pointer
i32.const 0
i32.store $trace_mem offset=1
global.get $mem_pointer
i32.const 0
i32.store $trace_mem offset=5
local.tee $i32
global.get $mem_pointer
local.get $i32
i32.store $trace_mem offset=9
global.get $mem_pointer
i32.const 13
i32.add
global.set $mem_pointer
drop
i32.const 1
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
call_indirect (type 2)
global.get $mem_pointer
i32.const 254
i32.store8 $trace_mem offset=0
global.get $mem_pointer
local.get $addr_local
i32.store $trace_mem offset=1
global.get $mem_pointer
i32.const 2
i32.store $trace_mem offset=5
global.get $mem_pointer
i32.const 9
i32.add
global.set $mem_pointer

global.get $internal_call
i32.eqz
(if (then
global.get $mem_pointer
i32.const 15
i32.store8 $trace_mem offset=0
global.get $mem_pointer
i32.const 1
i32.add
global.set $mem_pointer
) (else
global.get $internal_call
i32.const 1
i32.sub
global.set $internal_call
))
i32.const 448000000
global.get $mem_pointer
i32.le_u
(if (then call $check_mem))
i32.const 80000
global.get $table_pointer
i32.le_u
(if (then call $check_table))
)
(func (;2;) (type 1)
(local $addr_local i32)(local $i32 i32)(local $i64 i64)(local $f32 f32)(local $f64 f64)(local $funcref funcref)(local)
)
(func (;3;) (type 2) (param i32)
(local $addr_local i32)(local $i32 i32)(local $i64 i64)(local $f32 f32)(local $f64 f64)(local $funcref funcref)(local)
)
(table (;0;) 3 3 funcref)
(export "entry" (func 3))
(elem(;0;)(i32.const0)func 5 5 5)

(memory $trace_mem (export "trace") 10000)
(global $mem_pointer (export "trace_byte_length") (mut i32) (i32.const 0))
(table $table (export "lookup") 100000 funcref)
(global $table_pointer (export "lookup_table_pointer") (mut i32) (i32.const 0))
(global $internal_call (mut i32) (i32.const 0))
)