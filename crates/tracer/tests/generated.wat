(module
(import "r3" "check_mem" (func $check_mem))
(type (;0;) (func))
(import "env" "memory" (memory (;0;) 1 1))
(func (;0;) (type 0)
(local $addr_local i32)(local $i32 i32)(local $i64 i64)(local $f32 f32)(local $f64 f64)(local $funcref funcref)(local)
global.get $call_stack_addr
i32.load8_u $call_stack
global.get $call_stack_addr
i32.const 1
i32.add
global.set $call_stack_addr
global.get $call_stack_addr
i32.const 1
i32.store8 $call_stack
i32.eqz
(if (then
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
) (else))
i32.const 0
local.tee $addr_local
i32.load align=1
global.get $mem_pointer
i32.const 40
i32.store8 $trace_mem offset=0
global.get $mem_pointer
local.get $addr_local
local.get $i32
i32.store $shadow_mem
local.get $addr_local
i32.store $trace_mem offset=1
global.get $mem_pointer
local.get $i32
i32.store $trace_mem offset=5
global.get $mem_pointer
i32.const 9
i32.add
global.set $mem_pointer
local.get $i32
drop

global.get $call_stack_addr
i32.const 1
i32.sub
global.set $call_stack_addr
global.get $call_stack_addr
i32.const 1
i32.eq
(if (then
global.get $mem_pointer
i32.const 15
i32.store8 $trace_mem offset=0
global.get $mem_pointer
i32.const 1
i32.add
global.set $mem_pointer
) (else))
i32.const 44800000
global.get $mem_pointer
i32.le_u
(if (then call $check_mem))
)
(export "entry" (func 1))

(memory $trace_mem (export "trace") 10000)
(memory $call_stack 1000)
(global $mem_pointer (export "trace_byte_length") (mut i32) (i32.const 0))
(global $call_stack_addr (mut i32) (i32.const 1))
(memory $shadow_mem 1 1)
)