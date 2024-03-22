(module
    (type $t (func (param i32 i32) (result i32)))
    (func (export "main") (result i32) (local i32)
        i32.const 0
        i32.const 0
        i32.const 1
        (if (type $t)
            (then drop) 
            (else drop))
    )
    (memory $mem 1 1)
    (data $mem (i32.const 0) "\ff\ff\ff\ff")
)

