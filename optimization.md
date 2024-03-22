# Shadow memory opt

### Instrument loads without shadow memory opt `i32.load`:

```wasm
;; trace type (3 instructions)
global.get $mem_pointer
i32.const 0x28
i32.store8 $trace_mem
;; trace address (4 instructions)
local.tee $i32
global.get $mem_pointer
local.get $i32
i32.store offset=1 $trace_mem
;; original load
i32.load
;; trace loaded value (4 instructions)
local.tee $i32
global.get $mem_pointer
local.get $i32
i32.store offset=5 $trace_mem
;; increment mem pointer (4 instructions)
global.get $mem_pointer
i32.const 9
i32.add
global.set $mem_pointer
```

- Without shadow memory opt, we need to add 15 (3 additional memory instructions) instrumentation instructions to each load (in reality even more to resolve offset).
- If we want to perform shadow memory opt asynchronously, we need a similar amount of instructions for each store.

### Instrument loads (and stores) wth shadow memory opt
- `i32.store`
```wasm
;; clone store (6 instructions)
local.set $i32
local.tee $i32_2
local.get $i32
i32.store $shadow_mem
local.get $i32_2
local.get $i32
;; original store
i32.store
```
- `i32.load`
```wasm
;; preserve address (1 instruction)
local.tee $i32
;; original load
i32.load
;; perform comparison (4 instructions)
local.tee $i32_2
local.get $i32
i32.load $shadow_mem
i32.ne
(if (then
;; now trace the load (<15 instructions)
) (else))
```
- With shadow memory instruction we have 6 (1 memory) additional instructions per store and 5 (1 memory) additional instructions per load
- In case of a unique load we have of course >15 instructions per load, but as effectiveness of opt optimization shows
this happens very rarely.

=> Applying shadow opt during record is in most cases more memory and performance efficient than not applying it.