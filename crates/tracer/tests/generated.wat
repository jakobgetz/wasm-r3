(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (import "r3" "check_mem" (func $check_mem (;0;) (type 0)))
  (import "r3" "check_table" (func $check_table (;1;) (type 0)))
  (import "env" "changeMem" (func (;2;) (type 0)))
  (func (;3;) (type 0)
    (local $addr_local i32) (local $i32 i32) (local $i64 i64) (local $f32 f32) (local $f64 f64) (local $funcref funcref)
    global.get $internal_call
    i32.eqz
    if ;; label = @1
      global.get $mem_pointer
      i32.const 2
      i32.store8 $trace_mem
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
    else
    end
    call 4
    global.get $mem_pointer
    i32.const 46
    i32.store8 $trace_mem
    local.tee $i32
    global.get $mem_pointer
    local.get $i32
    i32.store $trace_mem offset=1
    i32.load16_u
    local.tee $i32
    global.get $mem_pointer
    local.get $i32
    i32.store $trace_mem offset=5
    global.get $mem_pointer
    i32.const 9
    i32.add
    global.set $mem_pointer
    i32.eqz
    if ;; label = @1
      unreachable
    end
    call 4
    global.get $mem_pointer
    i32.const 46
    i32.store8 $trace_mem
    local.tee $i32
    global.get $mem_pointer
    local.get $i32
    i32.store $trace_mem offset=1
    i32.load16_s
    local.tee $i32
    global.get $mem_pointer
    local.get $i32
    i32.store $trace_mem offset=5
    global.get $mem_pointer
    i32.const 9
    i32.add
    global.set $mem_pointer
    i32.eqz
    if ;; label = @1
      unreachable
    end
    global.get $internal_call
    i32.eqz
    if ;; label = @1
      global.get $mem_pointer
      i32.const 15
      i32.store8 $trace_mem
      global.get $mem_pointer
      i32.const 1
      i32.add
      global.set $mem_pointer
    else
      global.get $internal_call
      i32.const 1
      i32.sub
      global.set $internal_call
    end
    i32.const 448000000
    global.get $mem_pointer
    i32.le_u
    if ;; label = @1
      call $check_mem
    end
    i32.const 80000
    global.get $table_pointer
    i32.le_u
    if ;; label = @1
      call $check_table
    end
  )
  (func (;4;) (type 1) (result i32)
    (local $addr_local i32) (local $i32 i32) (local $i64 i64) (local $f32 f32) (local $f64 f64) (local $funcref funcref) (local i32)
    i32.const 1
    i32.const 0
    global.get $mem_pointer
    i32.const 54
    i32.store8 $trace_mem
    local.tee $i32
    global.get $mem_pointer
    local.get $i32
    i32.store $trace_mem offset=1
    local.set $i32
    local.tee $addr_local
    global.get $mem_pointer
    local.get $addr_local
    i32.store
    local.get $i32
    i32.store
    i32.const 0
    global.set $internal_call
    global.get $mem_pointer
    i32.const 16
    i32.store8 $trace_mem
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
    i32.store8 $trace_mem
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
    i32.const 0
  )
  (table $table (;0;) 100000 funcref)
  (memory (;0;) 1)
  (memory $trace_mem (;1;) 10000)
  (global $mem_pointer (;0;) (mut i32) i32.const 0)
  (global $table_pointer (;1;) (mut i32) i32.const 0)
  (global $internal_call (;2;) (mut i32) i32.const 0)
  (export "entry" (func 3))
  (export "memory" (memory 0))
  (export "trace" (memory $trace_mem))
  (export "trace_byte_length" (global $mem_pointer))
  (export "lookup" (table $table))
  (export "lookup_table_pointer" (global $table_pointer))
)