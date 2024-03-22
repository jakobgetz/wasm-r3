(module
  (type (;0;) (func))
  (import "r3" "check_mem" (func $check_mem (;0;) (type 0)))
  (import "env" "foo" (func (;1;) (type 0)))
  (import "env" "bar" (func (;2;) (type 0)))
  (import "env" "baz" (func (;3;) (type 0)))
  (import "env" "mem" (memory (;0;) 1))
  (func (;4;) (type 0)
    (local $addr_local i32) (local $i32 i32) (local $i64 i64) (local $f32 f32) (local $f64 f64) (local $funcref funcref)
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
    if ;; label = @1
      global.get $mem_pointer
      i32.const 2
      i32.store8 $trace_mem
      global.get $mem_pointer
      i32.const 3
      i32.store $trace_mem offset=1
      global.get $mem_pointer
      i32.const 0
      i32.store $trace_mem offset=5
      global.get $mem_pointer
      i32.const 9
      i32.add
      global.set $mem_pointer
      memory.size
      memory.size $shadow_mem
      i32.ne
      if ;; label = @2
        global.get $mem_pointer
        i32.const 63
        i32.store8 $trace_mem
        global.get $mem_pointer
        memory.size
        memory.size $shadow_mem
        i32.sub
        local.tee $i32
        i32.store $trace_mem offset=1
        local.get $i32
        memory.grow $shadow_mem
        drop
        global.get $mem_pointer
        i32.const 5
        i32.add
        global.set $mem_pointer
      else
      end
    else
    end
    i32.const 0
    local.tee $addr_local
    i32.const 4
    i32.mul
    i32.load $func_map_mem
    local.tee $i32
    i32.const -1
    i32.ne
    if (type 0) ;; label = @1
      global.get $call_stack_addr
      i32.const 1
      i32.add
      global.set $call_stack_addr
      global.get $call_stack_addr
      i32.const 0
      i32.store8 $call_stack
      global.get $mem_pointer
      i32.const 17
      i32.store8 $trace_mem
      global.get $mem_pointer
      local.get $addr_local
      i32.store $trace_mem offset=1
      global.get $mem_pointer
      i32.const 5
      i32.add
      global.set $mem_pointer
      local.get $addr_local
      call_indirect (type 0)
      global.get $call_stack_addr
      i32.const 1
      i32.sub
      global.set $call_stack_addr
      global.get $mem_pointer
      i32.const 255
      i32.store8 $trace_mem
      global.get $mem_pointer
      local.get $i32
      i32.store $trace_mem offset=1
      global.get $mem_pointer
      i32.const 0
      i32.store $trace_mem offset=5
      global.get $mem_pointer
      i32.const 9
      i32.add
      global.set $mem_pointer
      memory.size
      memory.size $shadow_mem
      i32.ne
      if ;; label = @2
        global.get $mem_pointer
        i32.const 63
        i32.store8 $trace_mem
        global.get $mem_pointer
        memory.size
        memory.size $shadow_mem
        i32.sub
        local.tee $i32
        i32.store $trace_mem offset=1
        local.get $i32
        memory.grow $shadow_mem
        drop
        global.get $mem_pointer
        i32.const 5
        i32.add
        global.set $mem_pointer
      else
      end
    else
      local.get $addr_local
      call_indirect (type 0)
    end
    i32.const 1
    local.tee $addr_local
    i32.const 4
    i32.mul
    i32.load $func_map_mem
    local.tee $i32
    i32.const -1
    i32.ne
    if (type 0) ;; label = @1
      global.get $call_stack_addr
      i32.const 1
      i32.add
      global.set $call_stack_addr
      global.get $call_stack_addr
      i32.const 0
      i32.store8 $call_stack
      global.get $mem_pointer
      i32.const 17
      i32.store8 $trace_mem
      global.get $mem_pointer
      local.get $addr_local
      i32.store $trace_mem offset=1
      global.get $mem_pointer
      i32.const 5
      i32.add
      global.set $mem_pointer
      local.get $addr_local
      call_indirect (type 0)
      global.get $call_stack_addr
      i32.const 1
      i32.sub
      global.set $call_stack_addr
      global.get $mem_pointer
      i32.const 255
      i32.store8 $trace_mem
      global.get $mem_pointer
      local.get $i32
      i32.store $trace_mem offset=1
      global.get $mem_pointer
      i32.const 0
      i32.store $trace_mem offset=5
      global.get $mem_pointer
      i32.const 9
      i32.add
      global.set $mem_pointer
      memory.size
      memory.size $shadow_mem
      i32.ne
      if ;; label = @2
        global.get $mem_pointer
        i32.const 63
        i32.store8 $trace_mem
        global.get $mem_pointer
        memory.size
        memory.size $shadow_mem
        i32.sub
        local.tee $i32
        i32.store $trace_mem offset=1
        local.get $i32
        memory.grow $shadow_mem
        drop
        global.get $mem_pointer
        i32.const 5
        i32.add
        global.set $mem_pointer
      else
      end
    else
      local.get $addr_local
      call_indirect (type 0)
    end
    i32.const 2
    local.tee $addr_local
    i32.const 4
    i32.mul
    i32.load $func_map_mem
    local.tee $i32
    i32.const -1
    i32.ne
    if (type 0) ;; label = @1
      global.get $call_stack_addr
      i32.const 1
      i32.add
      global.set $call_stack_addr
      global.get $call_stack_addr
      i32.const 0
      i32.store8 $call_stack
      global.get $mem_pointer
      i32.const 17
      i32.store8 $trace_mem
      global.get $mem_pointer
      local.get $addr_local
      i32.store $trace_mem offset=1
      global.get $mem_pointer
      i32.const 5
      i32.add
      global.set $mem_pointer
      local.get $addr_local
      call_indirect (type 0)
      global.get $call_stack_addr
      i32.const 1
      i32.sub
      global.set $call_stack_addr
      global.get $mem_pointer
      i32.const 255
      i32.store8 $trace_mem
      global.get $mem_pointer
      local.get $i32
      i32.store $trace_mem offset=1
      global.get $mem_pointer
      i32.const 0
      i32.store $trace_mem offset=5
      global.get $mem_pointer
      i32.const 9
      i32.add
      global.set $mem_pointer
      memory.size
      memory.size $shadow_mem
      i32.ne
      if ;; label = @2
        global.get $mem_pointer
        i32.const 63
        i32.store8 $trace_mem
        global.get $mem_pointer
        memory.size
        memory.size $shadow_mem
        i32.sub
        local.tee $i32
        i32.store $trace_mem offset=1
        local.get $i32
        memory.grow $shadow_mem
        drop
        global.get $mem_pointer
        i32.const 5
        i32.add
        global.set $mem_pointer
      else
      end
    else
      local.get $addr_local
      call_indirect (type 0)
    end
    i32.const 1
    local.tee $addr_local
    i32.load
    local.tee $i32
    local.get $addr_local
    i32.load $shadow_mem
    i32.ne
    if ;; label = @1
      global.get $mem_pointer
      i32.const 40
      i32.store8 $trace_mem
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
    else
    end
    local.get $i32
    drop
    global.get $call_stack_addr
    i32.const 1
    i32.sub
    global.set $call_stack_addr
    global.get $call_stack_addr
    i32.const 1
    i32.eq
    if ;; label = @1
      global.get $mem_pointer
      i32.const 15
      i32.store8 $trace_mem
      global.get $mem_pointer
      i32.const 1
      i32.add
      global.set $mem_pointer
    else
    end
    i32.const 44800000
    global.get $mem_pointer
    i32.le_u
    if ;; label = @1
      call $check_mem
    end
  )
  (func (;5;) (type 0)
    (local $addr_local i32) (local $i32 i32) (local $i64 i64) (local $f32 f32) (local $f64 f64) (local $funcref funcref)
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
    if ;; label = @1
      global.get $mem_pointer
      i32.const 2
      i32.store8 $trace_mem
      global.get $mem_pointer
      i32.const 4
      i32.store $trace_mem offset=1
      global.get $mem_pointer
      i32.const 0
      i32.store $trace_mem offset=5
      global.get $mem_pointer
      i32.const 9
      i32.add
      global.set $mem_pointer
      memory.size
      memory.size $shadow_mem
      i32.ne
      if ;; label = @2
        global.get $mem_pointer
        i32.const 63
        i32.store8 $trace_mem
        global.get $mem_pointer
        memory.size
        memory.size $shadow_mem
        i32.sub
        local.tee $i32
        i32.store $trace_mem offset=1
        local.get $i32
        memory.grow $shadow_mem
        drop
        global.get $mem_pointer
        i32.const 5
        i32.add
        global.set $mem_pointer
      else
      end
    else
    end
    i32.const 0
    local.tee $addr_local
    i32.load
    local.tee $i32
    local.get $addr_local
    i32.load $shadow_mem
    i32.ne
    if ;; label = @1
      global.get $mem_pointer
      i32.const 40
      i32.store8 $trace_mem
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
    else
    end
    local.get $i32
    drop
    global.get $call_stack_addr
    i32.const 1
    i32.sub
    global.set $call_stack_addr
    global.get $call_stack_addr
    i32.const 1
    i32.eq
    if ;; label = @1
      global.get $mem_pointer
      i32.const 15
      i32.store8 $trace_mem
      global.get $mem_pointer
      i32.const 1
      i32.add
      global.set $mem_pointer
    else
    end
    i32.const 44800000
    global.get $mem_pointer
    i32.le_u
    if ;; label = @1
      call $check_mem
    end
  )
  (table (;0;) 3 3 funcref)
  (memory $trace_mem (;1;) 10000)
  (memory $call_stack (;2;) 1000)
  (memory $func_map_mem (;3;) 1)
  (memory $shadow_mem (;4;) 1)
  (global $mem_pointer (;0;) (mut i32) i32.const 0)
  (global $call_stack_addr (;1;) (mut i32) i32.const 1)
  (export "entry" (func 4))
  (export "reentry" (func 5))
  (export "trace" (memory $trace_mem))
  (export "trace_byte_length" (global $mem_pointer))
  (elem (;0;) (i32.const 0) func 1 2 3)
  (data (;0;) (memory $func_map_mem) (i32.const 0) "\ff\ff\ff\ff\ff\ff\ff\ff\00\00\00\00")
)