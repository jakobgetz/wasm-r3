(module
  (global $env.memoryBase (;0;) (import "env" "memoryBase") i32)
  (func $env._acosf (;0;) (import "env" "_acosf") (param f64) (result f64))
  (func $env._alertError (;1;) (import "env" "_alertError") (param i32 i32 i32))
  (func $env._sinf (;2;) (import "env" "_sinf") (param f64) (result f64))
  (func $env._fmodf (;3;) (import "env" "_fmodf") (param f64 f64) (result f64))
  (memory $env.memory (;0;) (import "env" "memory") 256)
  (table $env.table (;0;) (import "env" "table") 0 funcref)
  (global $env.tableBase (;1;) (import "env" "tableBase") i32)
  (global $global2 (mut i32) (i32.const 0))
  (global $global3 (mut i32) (i32.const 0))
  (func $func4 (param $var0 i32) (param $var1 i32) (param $var2 i32) (param $var3 f32)
    (local $var4 f32)
    block
      local.get $var0
      f32.const 1
      local.get $var3
      f32.sub
      local.tee $var4
      local.get $var1
      f32.load
      f32.mul
      local.get $var2
      f32.load
      local.get $var3
      f32.mul
      f32.add
      f32.store
      local.get $var0
      local.get $var4
      local.get $var1
      f32.load offset=4
      f32.mul
      local.get $var2
      f32.load offset=4
      local.get $var3
      f32.mul
      f32.add
      f32.store offset=4
      local.get $var0
      local.get $var4
      local.get $var1
      f32.load offset=8
      f32.mul
      local.get $var2
      f32.load offset=8
      local.get $var3
      f32.mul
      f32.add
      f32.store offset=8
    end
  )
  (func $func5 (param $var0 f32) (param $var1 f32) (param $var2 f32) (result f32)
    local.get $var1
    local.get $var2
    f32.mul
    f32.const 1
    local.get $var2
    f32.sub
    local.get $var0
    f32.mul
    f32.add
  )
  (func $func6 (param $var0 i32) (param $var1 i32) (param $var2 i32) (param $var3 f32)
    (local $var4 f32)
    (local $var5 f32)
    (local $var6 f32)
    (local $var7 f32)
    (local $var8 f32)
    (local $var9 f32)
    (local $var10 f32)
    (local $var11 f32)
    (local $var12 f32)
    (local $var13 f32)
    (local $var14 f32)
    block
      local.get $var1
      f32.load
      local.tee $var11
      local.get $var2
      f32.load
      local.tee $var6
      f32.mul
      local.get $var1
      f32.load offset=4
      local.tee $var12
      local.get $var2
      f32.load offset=4
      local.tee $var7
      f32.mul
      f32.add
      local.get $var1
      f32.load offset=8
      local.tee $var13
      local.get $var2
      f32.load offset=8
      local.tee $var8
      f32.mul
      f32.add
      local.get $var1
      f32.load offset=12
      local.tee $var14
      local.get $var2
      f32.load offset=12
      local.tee $var9
      f32.mul
      f32.add
      local.tee $var4
      f32.const 0.0
      f32.lt
      if
        local.get $var6
        f32.neg
        local.set $var6
        local.get $var4
        f32.neg
        local.set $var4
        local.get $var7
        f32.neg
        local.set $var7
        local.get $var9
        f32.neg
        local.set $var9
        local.get $var8
        f32.neg
        local.set $var8
      end
      f32.const 1
      local.get $var4
      f32.sub
      f32.const 9.99999997e-07
      f32.gt
      if (result f32)
        local.get $var4
        f64.promote_f32
        call $env._acosf
        f32.demote_f64
        local.tee $var10
        f64.promote_f32
        call $env._sinf
        f32.demote_f64
        local.set $var5
        f32.const 1
        local.get $var3
        f32.sub
        local.get $var10
        f32.mul
        f64.promote_f32
        call $env._sinf
        f32.demote_f64
        local.get $var5
        f32.div
        local.set $var4
        local.get $var10
        local.get $var3
        f32.mul
        f64.promote_f32
        call $env._sinf
        f32.demote_f64
        local.get $var5
        f32.div
      else
        f32.const 1
        local.get $var3
        f32.sub
        local.set $var4
        local.get $var3
      end
      local.set $var5
      local.get $var0
      local.get $var11
      local.get $var4
      f32.mul
      local.get $var6
      local.get $var5
      f32.mul
      f32.add
      f32.store
      local.get $var0
      local.get $var12
      local.get $var4
      f32.mul
      local.get $var7
      local.get $var5
      f32.mul
      f32.add
      f32.store offset=4
      local.get $var0
      local.get $var13
      local.get $var4
      f32.mul
      local.get $var8
      local.get $var5
      f32.mul
      f32.add
      f32.store offset=8
      local.get $var0
      local.get $var14
      local.get $var4
      f32.mul
      local.get $var9
      local.get $var5
      f32.mul
      f32.add
      f32.store offset=12
    end
  )
  (func $__Z27setRotationTranslationScaleR4Mat4RK4QuatRK4Vec3S6_ (;7;) (export "__Z27setRotationTranslationScaleR4Mat4RK4QuatRK4Vec3S6_") (param $var0 i32) (param $var1 i32) (param $var2 i32) (param $var3 i32)
    (local $var4 f32)
    (local $var5 f32)
    (local $var6 f32)
    (local $var7 f32)
    (local $var8 f32)
    (local $var9 f32)
    (local $var10 f32)
    (local $var11 f32)
    (local $var12 f32)
    (local $var13 f32)
    (local $var14 f32)
    (local $var15 f32)
    (local $var16 f32)
    (local $var17 f32)
    (local $var18 f32)
    block
      local.get $var1
      f32.load
      local.tee $var4
      local.get $var4
      f32.add
      local.set $var11
      local.get $var4
      local.get $var1
      f32.load offset=4
      local.tee $var5
      local.get $var5
      f32.add
      local.tee $var6
      f32.mul
      local.set $var12
      local.get $var4
      local.get $var1
      f32.load offset=8
      local.tee $var7
      local.get $var7
      f32.add
      local.tee $var8
      f32.mul
      local.set $var13
      local.get $var11
      local.get $var1
      f32.load offset=12
      local.tee $var14
      f32.mul
      local.set $var15
      local.get $var3
      f32.load offset=4
      local.set $var9
      local.get $var3
      f32.load offset=8
      local.set $var10
      local.get $var0
      local.get $var3
      f32.load
      local.tee $var16
      f32.const 1
      local.get $var5
      local.get $var6
      f32.mul
      local.tee $var17
      local.get $var7
      local.get $var8
      f32.mul
      local.tee $var7
      f32.add
      f32.sub
      f32.mul
      f32.store
      local.get $var0
      local.get $var16
      local.get $var12
      local.get $var14
      local.get $var8
      f32.mul
      local.tee $var18
      f32.add
      f32.mul
      f32.store offset=4
      local.get $var0
      local.get $var16
      local.get $var13
      local.get $var6
      local.get $var14
      f32.mul
      local.tee $var6
      f32.sub
      f32.mul
      f32.store offset=8
      local.get $var0
      f32.const 0.0
      f32.store offset=12
      local.get $var0
      local.get $var12
      local.get $var18
      f32.sub
      local.get $var9
      f32.mul
      f32.store offset=16
      local.get $var0
      local.get $var9
      f32.const 1
      local.get $var4
      local.get $var11
      f32.mul
      local.tee $var4
      local.get $var7
      f32.add
      f32.sub
      f32.mul
      f32.store offset=20
      local.get $var0
      local.get $var5
      local.get $var8
      f32.mul
      local.tee $var5
      local.get $var15
      f32.add
      local.get $var9
      f32.mul
      f32.store offset=24
      local.get $var0
      f32.const 0.0
      f32.store offset=28
      local.get $var0
      local.get $var13
      local.get $var6
      f32.add
      local.get $var10
      f32.mul
      f32.store offset=32
      local.get $var0
      local.get $var5
      local.get $var15
      f32.sub
      local.get $var10
      f32.mul
      f32.store offset=36
      local.get $var0
      f32.const 1
      local.get $var4
      local.get $var17
      f32.add
      f32.sub
      local.get $var10
      f32.mul
      f32.store offset=40
      local.get $var0
      f32.const 0.0
      f32.store offset=44
      local.get $var0
      local.get $var2
      i32.load
      i32.store offset=48
      local.get $var0
      local.get $var2
      i32.load offset=4
      i32.store offset=52
      local.get $var0
      local.get $var2
      i32.load offset=8
      i32.store offset=56
      local.get $var0
      f32.const 1
      f32.store offset=60
    end
  )
  (func $__Z11setIdentityR4Mat4 (;8;) (export "__Z11setIdentityR4Mat4") (param $var0 i32)
    (local $var1 i32)
    block
      local.get $var0
      f32.const 1
      f32.store
      local.get $var0
      i32.const 4
      i32.add
      local.tee $var1
      i32.const 0
      i32.store
      local.get $var1
      i32.const 0
      i32.store offset=4
      local.get $var1
      i32.const 0
      i32.store offset=8
      local.get $var1
      i32.const 0
      i32.store offset=12
      local.get $var0
      f32.const 1
      f32.store offset=20
      local.get $var0
      i32.const 24
      i32.add
      local.tee $var1
      i32.const 0
      i32.store
      local.get $var1
      i32.const 0
      i32.store offset=4
      local.get $var1
      i32.const 0
      i32.store offset=8
      local.get $var1
      i32.const 0
      i32.store offset=12
      local.get $var0
      f32.const 1
      f32.store offset=40
      local.get $var0
      i32.const 44
      i32.add
      local.tee $var1
      i32.const 0
      i32.store
      local.get $var1
      i32.const 0
      i32.store offset=4
      local.get $var1
      i32.const 0
      i32.store offset=8
      local.get $var1
      i32.const 0
      i32.store offset=12
      local.get $var0
      f32.const 1
      f32.store offset=60
    end
  )
  (func $__Z5m4mulR4Mat4RKS_S2_ (;9;) (export "__Z5m4mulR4Mat4RKS_S2_") (param $var0 i32) (param $var1 i32) (param $var2 i32)
    (local $var3 i32)
    (local $var4 i32)
    (local $var5 i32)
    (local $var6 i32)
    (local $var7 i32)
    (local $var8 i32)
    (local $var9 i32)
    (local $var10 i32)
    (local $var11 i32)
    (local $var12 i32)
    (local $var13 i32)
    (local $var14 i32)
    (local $var15 i32)
    (local $var16 i32)
    (local $var17 i32)
    (local $var18 i32)
    (local $var19 i32)
    (local $var20 i32)
    (local $var21 i32)
    (local $var22 i32)
    (local $var23 i32)
    (local $var24 i32)
    (local $var25 f32)
    block
      local.get $var1
      i32.const 16
      i32.add
      local.set $var10
      local.get $var1
      i32.const 32
      i32.add
      local.set $var11
      local.get $var1
      i32.const 48
      i32.add
      local.set $var12
      local.get $var1
      i32.const 4
      i32.add
      local.set $var13
      local.get $var1
      i32.const 20
      i32.add
      local.set $var14
      local.get $var1
      i32.const 36
      i32.add
      local.set $var15
      local.get $var1
      i32.const 52
      i32.add
      local.set $var16
      local.get $var1
      i32.const 8
      i32.add
      local.set $var17
      local.get $var1
      i32.const 24
      i32.add
      local.set $var18
      local.get $var1
      i32.const 40
      i32.add
      local.set $var19
      local.get $var1
      i32.const 56
      i32.add
      local.set $var20
      local.get $var1
      i32.const 12
      i32.add
      local.set $var21
      local.get $var1
      i32.const 28
      i32.add
      local.set $var22
      local.get $var1
      i32.const 44
      i32.add
      local.set $var23
      local.get $var1
      i32.const 60
      i32.add
      local.set $var24
      i32.const 0
      local.set $var6
      loop $label0
        local.get $var2
        local.get $var6
        i32.const 2
        i32.shl
        local.tee $var4
        i32.const 2
        i32.shl
        i32.add
        local.set $var5
        local.get $var0
        local.get $var4
        i32.const 2
        i32.shl
        i32.add
        local.tee $var3
        f32.const 0.0
        f32.store
        local.get $var3
        local.get $var5
        f32.load
        local.get $var1
        f32.load
        f32.mul
        f32.const 0.0
        f32.add
        local.tee $var25
        f32.store
        local.get $var3
        local.get $var25
        local.get $var2
        local.get $var4
        i32.const 1
        i32.or
        i32.const 2
        i32.shl
        i32.add
        local.tee $var7
        f32.load
        local.get $var10
        f32.load
        f32.mul
        f32.add
        local.tee $var25
        f32.store
        local.get $var3
        local.get $var25
        local.get $var2
        local.get $var4
        i32.const 2
        i32.or
        i32.const 2
        i32.shl
        i32.add
        local.tee $var8
        f32.load
        local.get $var11
        f32.load
        f32.mul
        f32.add
        local.tee $var25
        f32.store
        local.get $var3
        local.get $var25
        local.get $var2
        local.get $var4
        i32.const 3
        i32.or
        i32.const 2
        i32.shl
        i32.add
        local.tee $var9
        f32.load
        local.get $var12
        f32.load
        f32.mul
        f32.add
        f32.store
        local.get $var0
        local.get $var4
        i32.const 1
        i32.or
        i32.const 2
        i32.shl
        i32.add
        local.tee $var3
        f32.const 0.0
        f32.store
        local.get $var3
        local.get $var5
        f32.load
        local.get $var13
        f32.load
        f32.mul
        f32.const 0.0
        f32.add
        local.tee $var25
        f32.store
        local.get $var3
        local.get $var25
        local.get $var7
        f32.load
        local.get $var14
        f32.load
        f32.mul
        f32.add
        local.tee $var25
        f32.store
        local.get $var3
        local.get $var25
        local.get $var8
        f32.load
        local.get $var15
        f32.load
        f32.mul
        f32.add
        local.tee $var25
        f32.store
        local.get $var3
        local.get $var25
        local.get $var9
        f32.load
        local.get $var16
        f32.load
        f32.mul
        f32.add
        f32.store
        local.get $var0
        local.get $var4
        i32.const 2
        i32.or
        i32.const 2
        i32.shl
        i32.add
        local.tee $var3
        f32.const 0.0
        f32.store
        local.get $var3
        local.get $var5
        f32.load
        local.get $var17
        f32.load
        f32.mul
        f32.const 0.0
        f32.add
        local.tee $var25
        f32.store
        local.get $var3
        local.get $var25
        local.get $var7
        f32.load
        local.get $var18
        f32.load
        f32.mul
        f32.add
        local.tee $var25
        f32.store
        local.get $var3
        local.get $var25
        local.get $var8
        f32.load
        local.get $var19
        f32.load
        f32.mul
        f32.add
        local.tee $var25
        f32.store
        local.get $var3
        local.get $var25
        local.get $var9
        f32.load
        local.get $var20
        f32.load
        f32.mul
        f32.add
        f32.store
        local.get $var0
        local.get $var4
        i32.const 3
        i32.or
        i32.const 2
        i32.shl
        i32.add
        local.tee $var4
        f32.const 0.0
        f32.store
        local.get $var4
        local.get $var5
        f32.load
        local.get $var21
        f32.load
        f32.mul
        f32.const 0.0
        f32.add
        local.tee $var25
        f32.store
        local.get $var4
        local.get $var25
        local.get $var7
        f32.load
        local.get $var22
        f32.load
        f32.mul
        f32.add
        local.tee $var25
        f32.store
        local.get $var4
        local.get $var25
        local.get $var8
        f32.load
        local.get $var23
        f32.load
        f32.mul
        f32.add
        local.tee $var25
        f32.store
        local.get $var4
        local.get $var25
        local.get $var9
        f32.load
        local.get $var24
        f32.load
        f32.mul
        f32.add
        f32.store
        local.get $var6
        i32.const 1
        i32.add
        local.tee $var6
        i32.const 4
        i32.ne
        br_if $label0
      end $label0
    end
  )
  (func $__Z13getStaticBoneR10StaticBoneRK9Animationj (;10;) (export "__Z13getStaticBoneR10StaticBoneRK9Animationj") (param $var0 i32) (param $var1 i32) (param $var2 i32) (result i32)
    (local $var3 i32)
    (local $var4 i32)
    (local $var5 i32)
    block (result i32)
      local.get $var1
      i32.load offset=4
      local.tee $var4
      i32.eqz
      if
        i32.const 0
        return
      end
      local.get $var1
      i32.load offset=8
      local.set $var5
      i32.const 0
      local.set $var1
      block $label2
        block $label1
          loop $label0
            local.get $var5
            local.get $var1
            i32.const 72
            i32.mul
            i32.add
            local.tee $var3
            i32.load
            local.get $var2
            i32.ne
            if
              local.get $var1
              i32.const 1
              i32.add
              local.tee $var1
              local.get $var4
              i32.lt_u
              if
                br $label0
              else
                i32.const 0
                local.set $var0
                br $label1
              end
              unreachable
            end
          end $label0
          br $label2
        end $label1
        local.get $var0
        return
      end $label2
      local.get $var0
      i32.const 72
      i32.add
      local.set $var1
      loop $label3
        local.get $var0
        local.get $var3
        i32.load
        i32.store
        local.get $var3
        i32.const 4
        i32.add
        local.set $var3
        local.get $var0
        i32.const 4
        i32.add
        local.tee $var0
        local.get $var1
        i32.lt_s
        br_if $label3
      end $label3
      i32.const 1
    end
  )
  (func $__Z15getAnimatedBoneR12AnimatedBoneRK9Animationj (;11;) (export "__Z15getAnimatedBoneR12AnimatedBoneRK9Animationj") (param $var0 i32) (param $var1 i32) (param $var2 i32) (result i32)
    (local $var3 i32)
    (local $var4 i32)
    (local $var5 i32)
    block (result i32)
      local.get $var1
      i32.load offset=12
      local.tee $var4
      i32.eqz
      if
        i32.const 0
        return
      end
      local.get $var1
      i32.load offset=16
      local.set $var5
      i32.const 0
      local.set $var1
      block $label2
        block $label1
          loop $label0
            local.get $var5
            local.get $var1
            i32.const 28
            i32.mul
            i32.add
            local.tee $var3
            i32.load
            local.get $var2
            i32.ne
            if
              local.get $var1
              i32.const 1
              i32.add
              local.tee $var1
              local.get $var4
              i32.lt_u
              if
                br $label0
              else
                i32.const 0
                local.set $var0
                br $label1
              end
              unreachable
            end
          end $label0
          br $label2
        end $label1
        local.get $var0
        return
      end $label2
      local.get $var0
      local.get $var3
      i32.load
      i32.store
      local.get $var0
      local.get $var3
      i32.load offset=4
      i32.store offset=4
      local.get $var0
      local.get $var3
      i32.load offset=8
      i32.store offset=8
      local.get $var0
      local.get $var3
      i32.load offset=12
      i32.store offset=12
      local.get $var0
      local.get $var3
      i32.load offset=16
      i32.store offset=16
      local.get $var0
      local.get $var3
      i32.load offset=20
      i32.store offset=20
      local.get $var0
      local.get $var3
      i32.load offset=24
      i32.store offset=24
      i32.const 1
    end
  )
  (func $func12 (param $var0 i32) (param $var1 i32) (param $var2 f32)
    (local $var3 i32)
    (local $var4 i32)
    (local $var5 i32)
    (local $var6 i32)
    (local $var7 i32)
    (local $var8 i32)
    (local $var9 i32)
    (local $var10 i32)
    (local $var11 i32)
    (local $var12 f32)
    (local $var13 f32)
    (local $var14 f32)
    (local $var15 f32)
    (local $var16 f32)
    (local $var17 f32)
    (local $var18 f32)
    (local $var19 f32)
    (local $var20 f32)
    (local $var21 f32)
    (local $var22 f32)
    (local $var23 f32)
    (local $var24 f32)
    block
      local.get $var1
      i32.load offset=8
      local.tee $var6
      i32.eqz
      if
        i32.const 0
        i32.const 29
        i32.const 0
        call $env._alertError
        return
      end
      local.get $var1
      i32.load offset=16
      local.tee $var5
      i32.eqz
      if
        i32.const 0
        i32.const 30
        i32.const 0
        call $env._alertError
        return
      end
      local.get $var1
      i32.load offset=24
      local.tee $var7
      i32.eqz
      if
        i32.const 0
        i32.const 31
        i32.const 0
        call $env._alertError
        return
      end
      local.get $var1
      i32.load offset=4
      local.tee $var3
      i32.const 1
      i32.eq
      if (result i32)
        local.get $var6
        i32.load offset=4
        local.set $var10
        local.get $var6
        i32.load offset=8
        local.set $var11
        local.get $var6
        i32.load offset=12
      else
        local.get $var2
        f64.promote_f32
        local.get $var6
        local.get $var3
        i32.const -1
        i32.add
        local.tee $var3
        i32.const 4
        i32.shl
        i32.add
        f32.load
        local.tee $var12
        f64.promote_f32
        call $env._fmodf
        f32.demote_f64
        local.set $var13
        local.get $var12
        local.get $var13
        f32.add
        local.set $var12
        local.get $var13
        f32.const 0.0
        f32.lt
        if
          local.get $var12
          local.set $var13
        end
        block $label0
          local.get $var3
          if
            i32.const 0
            local.set $var9
            loop $label2
              loop $label1
                local.get $var6
                local.get $var3
                local.get $var9
                i32.add
                i32.const 1
                i32.shr_u
                local.tee $var4
                i32.const 1
                i32.add
                local.tee $var8
                i32.const 4
                i32.shl
                i32.add
                f32.load
                local.tee $var12
                local.get $var13
                f32.le
                i32.eqz
                if
                  local.get $var6
                  local.get $var4
                  i32.const 4
                  i32.shl
                  i32.add
                  f32.load
                  local.get $var13
                  f32.ge
                  i32.eqz
                  br_if $label0
                  local.get $var9
                  local.get $var4
                  i32.const -1
                  i32.add
                  local.tee $var3
                  i32.lt_u
                  br_if $label1
                  br $label0
                end
              end $label1
              local.get $var8
              local.get $var3
              i32.lt_u
              if
                local.get $var8
                local.set $var9
                br $label2
              end
            end $label2
          else
            i32.const 0
            local.set $var4
            local.get $var6
            f32.load offset=16
            local.set $var12
          end
        end $label0
        f32.const 1
        local.get $var13
        local.get $var6
        local.get $var4
        i32.const 4
        i32.shl
        i32.add
        f32.load
        local.tee $var13
        f32.sub
        local.get $var12
        local.get $var13
        f32.sub
        f32.div
        local.tee $var13
        f32.sub
        local.set $var12
        local.get $var6
        local.get $var4
        i32.const 1
        i32.add
        local.tee $var3
        i32.const 4
        i32.shl
        i32.add
        f32.load offset=4
        local.get $var13
        f32.mul
        local.get $var6
        local.get $var4
        i32.const 4
        i32.shl
        i32.add
        f32.load offset=4
        local.get $var12
        f32.mul
        f32.add
        i32.reinterpret_f32
        local.set $var10
        local.get $var12
        local.get $var6
        local.get $var4
        i32.const 4
        i32.shl
        i32.add
        f32.load offset=8
        f32.mul
        local.get $var13
        local.get $var6
        local.get $var3
        i32.const 4
        i32.shl
        i32.add
        f32.load offset=8
        f32.mul
        f32.add
        i32.reinterpret_f32
        local.set $var11
        local.get $var12
        local.get $var6
        local.get $var4
        i32.const 4
        i32.shl
        i32.add
        f32.load offset=12
        f32.mul
        local.get $var13
        local.get $var6
        local.get $var3
        i32.const 4
        i32.shl
        i32.add
        f32.load offset=12
        f32.mul
        f32.add
        i32.reinterpret_f32
      end
      local.set $var6
      local.get $var1
      i32.load offset=12
      local.tee $var3
      i32.const 1
      i32.eq
      if (result f32)
        local.get $var5
        f32.load offset=4
        local.set $var18
        local.get $var5
        f32.load offset=8
        local.set $var20
        local.get $var5
        f32.load offset=12
        local.set $var17
        local.get $var5
        f32.load offset=16
      else
        local.get $var2
        f64.promote_f32
        local.get $var5
        local.get $var3
        i32.const -1
        i32.add
        local.tee $var3
        i32.const 20
        i32.mul
        i32.add
        f32.load
        local.tee $var12
        f64.promote_f32
        call $env._fmodf
        f32.demote_f64
        local.set $var13
        local.get $var12
        local.get $var13
        f32.add
        local.set $var12
        local.get $var13
        f32.const 0.0
        f32.lt
        if (result f32)
          local.get $var12
        else
          local.get $var13
        end
        local.set $var15
        block $label3
          local.get $var3
          if
            i32.const 0
            local.set $var9
            loop $label5
              loop $label4
                local.get $var5
                local.get $var3
                local.get $var9
                i32.add
                i32.const 1
                i32.shr_u
                local.tee $var4
                i32.const 1
                i32.add
                local.tee $var8
                i32.const 20
                i32.mul
                i32.add
                f32.load
                local.tee $var12
                local.get $var15
                f32.le
                i32.eqz
                if
                  local.get $var5
                  local.get $var4
                  i32.const 20
                  i32.mul
                  i32.add
                  f32.load
                  local.get $var15
                  f32.ge
                  i32.eqz
                  br_if $label3
                  local.get $var9
                  local.get $var4
                  i32.const -1
                  i32.add
                  local.tee $var3
                  i32.lt_u
                  br_if $label4
                  br $label3
                end
              end $label4
              local.get $var8
              local.get $var3
              i32.lt_u
              if
                local.get $var8
                local.set $var9
                br $label5
              end
            end $label5
          else
            i32.const 0
            local.set $var4
            local.get $var5
            f32.load offset=20
            local.set $var12
          end
        end $label3
        local.get $var5
        local.get $var4
        i32.const 20
        i32.mul
        i32.add
        f32.load
        local.set $var24
        local.get $var5
        local.get $var4
        i32.const 20
        i32.mul
        i32.add
        f32.load offset=4
        local.tee $var18
        local.get $var5
        local.get $var4
        i32.const 1
        i32.add
        local.tee $var3
        i32.const 20
        i32.mul
        i32.add
        f32.load offset=4
        local.tee $var21
        f32.mul
        local.get $var5
        local.get $var4
        i32.const 20
        i32.mul
        i32.add
        f32.load offset=8
        local.tee $var20
        local.get $var5
        local.get $var3
        i32.const 20
        i32.mul
        i32.add
        f32.load offset=8
        local.tee $var17
        f32.mul
        f32.add
        local.get $var5
        local.get $var4
        i32.const 20
        i32.mul
        i32.add
        f32.load offset=12
        local.tee $var23
        local.get $var5
        local.get $var3
        i32.const 20
        i32.mul
        i32.add
        f32.load offset=12
        local.tee $var16
        f32.mul
        f32.add
        local.get $var5
        local.get $var4
        i32.const 20
        i32.mul
        i32.add
        f32.load offset=16
        local.tee $var22
        local.get $var5
        local.get $var3
        i32.const 20
        i32.mul
        i32.add
        f32.load offset=16
        local.tee $var19
        f32.mul
        f32.add
        local.tee $var13
        f32.const 0.0
        f32.lt
        if (result f32)
          local.get $var21
          f32.neg
          local.set $var21
          local.get $var13
          f32.neg
          local.set $var14
          local.get $var17
          f32.neg
          local.set $var17
          local.get $var19
          f32.neg
          local.set $var19
          local.get $var16
          f32.neg
        else
          local.get $var13
          local.set $var14
          local.get $var16
        end
        local.set $var13
        local.get $var15
        local.get $var24
        f32.sub
        local.get $var12
        local.get $var24
        f32.sub
        f32.div
        local.set $var16
        f32.const 1
        local.get $var14
        f32.sub
        f32.const 9.99999997e-07
        f32.gt
        if (result f32)
          local.get $var14
          f64.promote_f32
          call $env._acosf
          f32.demote_f64
          local.tee $var15
          f64.promote_f32
          call $env._sinf
          f32.demote_f64
          local.set $var14
          f32.const 1
          local.get $var16
          f32.sub
          local.get $var15
          f32.mul
          f64.promote_f32
          call $env._sinf
          f32.demote_f64
          local.get $var14
          f32.div
          local.set $var12
          local.get $var16
          local.get $var15
          f32.mul
          f64.promote_f32
          call $env._sinf
          f32.demote_f64
          local.get $var14
          f32.div
        else
          f32.const 1
          local.get $var16
          f32.sub
          local.set $var12
          local.get $var16
        end
        local.set $var14
        local.get $var18
        local.get $var12
        f32.mul
        local.get $var21
        local.get $var14
        f32.mul
        f32.add
        local.set $var18
        local.get $var20
        local.get $var12
        f32.mul
        local.get $var17
        local.get $var14
        f32.mul
        f32.add
        local.set $var20
        local.get $var23
        local.get $var12
        f32.mul
        local.get $var13
        local.get $var14
        f32.mul
        f32.add
        local.set $var17
        local.get $var22
        local.get $var12
        f32.mul
        local.get $var19
        local.get $var14
        f32.mul
        f32.add
      end
      local.set $var23
      local.get $var1
      i32.load offset=20
      local.tee $var1
      i32.const 1
      i32.eq
      if (result f32)
        local.get $var7
        f32.load offset=4
        local.set $var22
        local.get $var7
        f32.load offset=8
        local.set $var15
        local.get $var7
        f32.load offset=12
      else
        local.get $var2
        f64.promote_f32
        local.get $var7
        local.get $var1
        i32.const -1
        i32.add
        local.tee $var1
        i32.const 4
        i32.shl
        i32.add
        f32.load
        local.tee $var2
        f64.promote_f32
        call $env._fmodf
        f32.demote_f64
        local.set $var12
        local.get $var2
        local.get $var12
        f32.add
        local.set $var2
        local.get $var12
        f32.const 0.0
        f32.lt
        if
          local.get $var2
          local.set $var12
        end
        block $label6
          local.get $var1
          if
            i32.const 0
            local.set $var8
            loop $label8
              loop $label7
                local.get $var7
                local.get $var1
                local.get $var8
                i32.add
                i32.const 1
                i32.shr_u
                local.tee $var3
                i32.const 1
                i32.add
                local.tee $var4
                i32.const 4
                i32.shl
                i32.add
                f32.load
                local.tee $var2
                local.get $var12
                f32.le
                i32.eqz
                if
                  local.get $var7
                  local.get $var3
                  i32.const 4
                  i32.shl
                  i32.add
                  f32.load
                  local.get $var12
                  f32.ge
                  i32.eqz
                  br_if $label6
                  local.get $var8
                  local.get $var3
                  i32.const -1
                  i32.add
                  local.tee $var1
                  i32.lt_u
                  br_if $label7
                  br $label6
                end
              end $label7
              local.get $var4
              local.get $var1
              i32.lt_u
              if
                local.get $var4
                local.set $var8
                br $label8
              end
            end $label8
          else
            i32.const 0
            local.set $var3
            local.get $var7
            f32.load offset=16
            local.set $var2
          end
        end $label6
        f32.const 1
        local.get $var12
        local.get $var7
        local.get $var3
        i32.const 4
        i32.shl
        i32.add
        f32.load
        local.tee $var12
        f32.sub
        local.get $var2
        local.get $var12
        f32.sub
        f32.div
        local.tee $var12
        f32.sub
        local.set $var2
        local.get $var7
        local.get $var3
        i32.const 1
        i32.add
        local.tee $var1
        i32.const 4
        i32.shl
        i32.add
        f32.load offset=4
        local.get $var12
        f32.mul
        local.get $var7
        local.get $var3
        i32.const 4
        i32.shl
        i32.add
        f32.load offset=4
        local.get $var2
        f32.mul
        f32.add
        local.set $var22
        local.get $var2
        local.get $var7
        local.get $var3
        i32.const 4
        i32.shl
        i32.add
        f32.load offset=8
        f32.mul
        local.get $var12
        local.get $var7
        local.get $var1
        i32.const 4
        i32.shl
        i32.add
        f32.load offset=8
        f32.mul
        f32.add
        local.set $var15
        local.get $var2
        local.get $var7
        local.get $var3
        i32.const 4
        i32.shl
        i32.add
        f32.load offset=12
        f32.mul
        local.get $var12
        local.get $var7
        local.get $var1
        i32.const 4
        i32.shl
        i32.add
        f32.load offset=12
        f32.mul
        f32.add
      end
      local.set $var14
      local.get $var18
      local.get $var20
      local.get $var20
      f32.add
      local.tee $var19
      f32.mul
      local.set $var13
      local.get $var17
      local.get $var17
      f32.add
      local.tee $var16
      local.get $var18
      f32.mul
      local.set $var21
      local.get $var0
      f32.const 1
      local.get $var17
      local.get $var16
      f32.mul
      local.tee $var12
      local.get $var20
      local.get $var19
      f32.mul
      local.tee $var17
      f32.add
      f32.sub
      local.get $var22
      f32.mul
      f32.store
      local.get $var0
      local.get $var23
      local.get $var16
      f32.mul
      local.tee $var2
      local.get $var13
      f32.add
      local.get $var22
      f32.mul
      f32.store offset=4
      local.get $var0
      local.get $var21
      local.get $var23
      local.get $var19
      f32.mul
      local.tee $var19
      f32.sub
      local.get $var22
      f32.mul
      f32.store offset=8
      local.get $var0
      f32.const 0.0
      f32.store offset=12
      local.get $var0
      local.get $var13
      local.get $var2
      f32.sub
      local.get $var15
      f32.mul
      f32.store offset=16
      local.get $var0
      f32.const 1
      local.get $var12
      local.get $var18
      local.get $var18
      local.get $var18
      f32.add
      local.tee $var2
      f32.mul
      local.tee $var13
      f32.add
      f32.sub
      local.get $var15
      f32.mul
      f32.store offset=20
      local.get $var0
      local.get $var20
      local.get $var16
      f32.mul
      local.tee $var12
      local.get $var23
      local.get $var2
      f32.mul
      local.tee $var2
      f32.add
      local.get $var15
      f32.mul
      f32.store offset=24
      local.get $var0
      f32.const 0.0
      f32.store offset=28
      local.get $var0
      local.get $var21
      local.get $var19
      f32.add
      local.get $var14
      f32.mul
      f32.store offset=32
      local.get $var0
      local.get $var12
      local.get $var2
      f32.sub
      local.get $var14
      f32.mul
      f32.store offset=36
      local.get $var0
      f32.const 1
      local.get $var17
      local.get $var13
      f32.add
      f32.sub
      local.get $var14
      f32.mul
      f32.store offset=40
      local.get $var0
      f32.const 0.0
      f32.store offset=44
      local.get $var0
      local.get $var10
      i32.store offset=48
      local.get $var0
      local.get $var11
      i32.store offset=52
      local.get $var0
      local.get $var6
      i32.store offset=56
      local.get $var0
      f32.const 1
      f32.store offset=60
    end
  )
  (func $func13 (param $var0 i32) (param $var1 i32) (param $var2 f32) (param $var3 i32)
    (local $var4 i32)
    (local $var5 i32)
    (local $var6 i32)
    (local $var7 i32)
    (local $var8 i32)
    (local $var9 i32)
    (local $var10 i32)
    (local $var11 i32)
    (local $var12 i32)
    (local $var13 f32)
    (local $var14 f32)
    (local $var15 f32)
    (local $var16 f32)
    (local $var17 f32)
    (local $var18 f32)
    (local $var19 f32)
    (local $var20 f32)
    (local $var21 f32)
    (local $var22 f32)
    (local $var23 f32)
    (local $var24 f32)
    (local $var25 f32)
    (local $var26 f32)
    (local $var27 f32)
    (local $var28 f32)
    (local $var29 f32)
    (local $var30 f32)
    (local $var31 f32)
    block
      global.get $global2
      local.set $var8
      global.get $global2
      i32.const 240
      i32.add
      global.set $global2
      local.get $var3
      i32.eqz
      if
        local.get $var0
        f32.const 1
        f32.store
        local.get $var0
        i32.const 4
        i32.add
        local.tee $var1
        i32.const 0
        i32.store
        local.get $var1
        i32.const 0
        i32.store offset=4
        local.get $var1
        i32.const 0
        i32.store offset=8
        local.get $var1
        i32.const 0
        i32.store offset=12
        local.get $var0
        f32.const 1
        f32.store offset=20
        local.get $var0
        i32.const 24
        i32.add
        local.tee $var1
        i32.const 0
        i32.store
        local.get $var1
        i32.const 0
        i32.store offset=4
        local.get $var1
        i32.const 0
        i32.store offset=8
        local.get $var1
        i32.const 0
        i32.store offset=12
        local.get $var0
        f32.const 1
        f32.store offset=40
        local.get $var0
        i32.const 44
        i32.add
        local.tee $var1
        i32.const 0
        i32.store
        local.get $var1
        i32.const 0
        i32.store offset=4
        local.get $var1
        i32.const 0
        i32.store offset=8
        local.get $var1
        i32.const 0
        i32.store offset=12
        local.get $var0
        f32.const 1
        f32.store offset=60
        local.get $var8
        global.set $global2
        return
      end
      local.get $var8
      i32.const 160
      i32.add
      local.set $var9
      local.get $var8
      i32.const 96
      i32.add
      local.set $var4
      local.get $var8
      i32.const 64
      i32.add
      local.set $var7
      local.get $var8
      local.set $var10
      block $label8
        block $label0
          local.get $var1
          i32.load offset=4
          local.tee $var12
          i32.eqz
          br_if $label0
          local.get $var1
          i32.load offset=8
          local.set $var5
          i32.const 0
          local.set $var6
          loop $label1
            local.get $var5
            local.get $var6
            i32.const 72
            i32.mul
            i32.add
            local.tee $var11
            i32.load
            local.get $var3
            i32.ne
            if
              local.get $var6
              i32.const 1
              i32.add
              local.tee $var6
              local.get $var12
              i32.ge_u
              br_if $label0
              br $label1
            end
          end $label1
          local.get $var9
          local.tee $var6
          i32.const 72
          i32.add
          local.set $var5
          loop $label2
            local.get $var6
            local.get $var11
            i32.load
            i32.store
            local.get $var11
            i32.const 4
            i32.add
            local.set $var11
            local.get $var6
            i32.const 4
            i32.add
            local.tee $var6
            local.get $var5
            i32.lt_s
            br_if $label2
          end $label2
          local.get $var4
          local.get $var1
          local.get $var2
          local.get $var9
          i32.load offset=4
          call $func13
          block $label6
            block $label3
              local.get $var1
              i32.load offset=12
              local.tee $var11
              i32.eqz
              br_if $label3
              local.get $var1
              i32.load offset=16
              local.set $var6
              i32.const 0
              local.set $var1
              loop $label4
                local.get $var6
                local.get $var1
                i32.const 28
                i32.mul
                i32.add
                local.tee $var5
                i32.load
                local.get $var3
                i32.ne
                if
                  local.get $var1
                  i32.const 1
                  i32.add
                  local.tee $var1
                  local.get $var11
                  i32.ge_u
                  br_if $label3
                  br $label4
                end
              end $label4
              local.get $var7
              local.get $var5
              i32.load
              i32.store
              local.get $var7
              local.get $var5
              i32.load offset=4
              i32.store offset=4
              local.get $var7
              local.get $var5
              i32.load offset=8
              i32.store offset=8
              local.get $var7
              local.get $var5
              i32.load offset=12
              i32.store offset=12
              local.get $var7
              local.get $var5
              i32.load offset=16
              i32.store offset=16
              local.get $var7
              local.get $var5
              i32.load offset=20
              i32.store offset=20
              local.get $var7
              local.get $var5
              i32.load offset=24
              i32.store offset=24
              local.get $var10
              local.get $var7
              local.get $var2
              call $func12
              local.get $var4
              f32.load
              local.set $var17
              local.get $var4
              f32.load offset=16
              local.set $var18
              local.get $var4
              f32.load offset=32
              local.set $var19
              local.get $var4
              f32.load offset=48
              local.set $var20
              local.get $var4
              f32.load offset=4
              local.set $var21
              local.get $var4
              f32.load offset=20
              local.set $var22
              local.get $var4
              f32.load offset=36
              local.set $var23
              local.get $var4
              f32.load offset=52
              local.set $var24
              local.get $var4
              f32.load offset=8
              local.set $var25
              local.get $var4
              f32.load offset=24
              local.set $var26
              local.get $var4
              f32.load offset=40
              local.set $var27
              local.get $var4
              f32.load offset=56
              local.set $var28
              local.get $var4
              f32.load offset=12
              local.set $var29
              local.get $var4
              f32.load offset=28
              local.set $var30
              local.get $var4
              f32.load offset=44
              local.set $var31
              local.get $var4
              f32.load offset=60
              local.set $var2
              i32.const 0
              local.set $var1
              loop $label5
                local.get $var10
                local.get $var1
                i32.const 2
                i32.shl
                local.tee $var3
                i32.const 2
                i32.shl
                i32.add
                f32.load
                local.set $var13
                local.get $var0
                local.get $var3
                i32.const 2
                i32.shl
                i32.add
                local.get $var13
                local.get $var17
                f32.mul
                f32.const 0.0
                f32.add
                local.get $var10
                local.get $var3
                i32.const 1
                i32.or
                i32.const 2
                i32.shl
                i32.add
                f32.load
                local.tee $var14
                local.get $var18
                f32.mul
                f32.add
                local.get $var10
                local.get $var3
                i32.const 2
                i32.or
                i32.const 2
                i32.shl
                i32.add
                f32.load
                local.tee $var15
                local.get $var19
                f32.mul
                f32.add
                local.get $var10
                local.get $var3
                i32.const 3
                i32.or
                i32.const 2
                i32.shl
                i32.add
                f32.load
                local.tee $var16
                local.get $var20
                f32.mul
                f32.add
                f32.store
                local.get $var0
                local.get $var3
                i32.const 1
                i32.or
                i32.const 2
                i32.shl
                i32.add
                local.get $var13
                local.get $var21
                f32.mul
                f32.const 0.0
                f32.add
                local.get $var14
                local.get $var22
                f32.mul
                f32.add
                local.get $var15
                local.get $var23
                f32.mul
                f32.add
                local.get $var16
                local.get $var24
                f32.mul
                f32.add
                f32.store
                local.get $var0
                local.get $var3
                i32.const 2
                i32.or
                i32.const 2
                i32.shl
                i32.add
                local.get $var13
                local.get $var25
                f32.mul
                f32.const 0.0
                f32.add
                local.get $var14
                local.get $var26
                f32.mul
                f32.add
                local.get $var15
                local.get $var27
                f32.mul
                f32.add
                local.get $var16
                local.get $var28
                f32.mul
                f32.add
                f32.store
                local.get $var0
                local.get $var3
                i32.const 3
                i32.or
                i32.const 2
                i32.shl
                i32.add
                local.get $var13
                local.get $var29
                f32.mul
                f32.const 0.0
                f32.add
                local.get $var14
                local.get $var30
                f32.mul
                f32.add
                local.get $var15
                local.get $var31
                f32.mul
                f32.add
                local.get $var16
                local.get $var2
                f32.mul
                f32.add
                f32.store
                local.get $var1
                i32.const 1
                i32.add
                local.tee $var1
                i32.const 4
                i32.ne
                br_if $label5
              end $label5
              br $label6
            end $label3
            local.get $var4
            f32.load
            local.set $var17
            local.get $var4
            f32.load offset=16
            local.set $var18
            local.get $var4
            f32.load offset=32
            local.set $var19
            local.get $var4
            f32.load offset=48
            local.set $var20
            local.get $var4
            f32.load offset=4
            local.set $var21
            local.get $var4
            f32.load offset=20
            local.set $var22
            local.get $var4
            f32.load offset=36
            local.set $var23
            local.get $var4
            f32.load offset=52
            local.set $var24
            local.get $var4
            f32.load offset=8
            local.set $var25
            local.get $var4
            f32.load offset=24
            local.set $var26
            local.get $var4
            f32.load offset=40
            local.set $var27
            local.get $var4
            f32.load offset=56
            local.set $var28
            local.get $var4
            f32.load offset=12
            local.set $var29
            local.get $var4
            f32.load offset=28
            local.set $var30
            local.get $var4
            f32.load offset=44
            local.set $var31
            local.get $var4
            f32.load offset=60
            local.set $var2
            i32.const 0
            local.set $var1
            loop $label7
              local.get $var9
              i32.const 8
              i32.add
              local.get $var1
              i32.const 2
              i32.shl
              local.tee $var3
              i32.const 2
              i32.shl
              i32.add
              f32.load
              local.set $var13
              local.get $var0
              local.get $var3
              i32.const 2
              i32.shl
              i32.add
              local.get $var13
              local.get $var17
              f32.mul
              f32.const 0.0
              f32.add
              local.get $var9
              i32.const 8
              i32.add
              local.get $var3
              i32.const 1
              i32.or
              i32.const 2
              i32.shl
              i32.add
              f32.load
              local.tee $var14
              local.get $var18
              f32.mul
              f32.add
              local.get $var9
              i32.const 8
              i32.add
              local.get $var3
              i32.const 2
              i32.or
              i32.const 2
              i32.shl
              i32.add
              f32.load
              local.tee $var15
              local.get $var19
              f32.mul
              f32.add
              local.get $var9
              i32.const 8
              i32.add
              local.get $var3
              i32.const 3
              i32.or
              i32.const 2
              i32.shl
              i32.add
              f32.load
              local.tee $var16
              local.get $var20
              f32.mul
              f32.add
              f32.store
              local.get $var0
              local.get $var3
              i32.const 1
              i32.or
              i32.const 2
              i32.shl
              i32.add
              local.get $var13
              local.get $var21
              f32.mul
              f32.const 0.0
              f32.add
              local.get $var14
              local.get $var22
              f32.mul
              f32.add
              local.get $var15
              local.get $var23
              f32.mul
              f32.add
              local.get $var16
              local.get $var24
              f32.mul
              f32.add
              f32.store
              local.get $var0
              local.get $var3
              i32.const 2
              i32.or
              i32.const 2
              i32.shl
              i32.add
              local.get $var13
              local.get $var25
              f32.mul
              f32.const 0.0
              f32.add
              local.get $var14
              local.get $var26
              f32.mul
              f32.add
              local.get $var15
              local.get $var27
              f32.mul
              f32.add
              local.get $var16
              local.get $var28
              f32.mul
              f32.add
              f32.store
              local.get $var0
              local.get $var3
              i32.const 3
              i32.or
              i32.const 2
              i32.shl
              i32.add
              local.get $var13
              local.get $var29
              f32.mul
              f32.const 0.0
              f32.add
              local.get $var14
              local.get $var30
              f32.mul
              f32.add
              local.get $var15
              local.get $var31
              f32.mul
              f32.add
              local.get $var16
              local.get $var2
              f32.mul
              f32.add
              f32.store
              local.get $var1
              i32.const 1
              i32.add
              local.tee $var1
              i32.const 4
              i32.ne
              br_if $label7
            end $label7
          end $label6
          br $label8
        end $label0
        i32.const 2
        i32.const 152
        local.get $var3
        call $env._alertError
      end $label8
      local.get $var8
      global.set $global2
    end
  )
  (func $func14 (param $var0 i32) (param $var1 i32) (param $var2 i32) (param $var3 f32)
    (local $var4 i32)
    (local $var5 i32)
    (local $var6 i32)
    (local $var7 i32)
    (local $var8 i32)
    (local $var9 i32)
    (local $var10 i32)
    (local $var11 i32)
    (local $var12 i32)
    (local $var13 i32)
    (local $var14 i32)
    (local $var15 i32)
    (local $var16 i32)
    (local $var17 i32)
    (local $var18 i32)
    (local $var19 i32)
    (local $var20 i32)
    (local $var21 i32)
    (local $var22 i32)
    (local $var23 i32)
    (local $var24 i32)
    (local $var25 i32)
    (local $var26 i32)
    (local $var27 i32)
    (local $var28 i32)
    (local $var29 i32)
    (local $var30 i32)
    (local $var31 i32)
    (local $var32 f32)
    (local $var33 f32)
    (local $var34 f32)
    (local $var35 f32)
    (local $var36 f32)
    (local $var37 f32)
    (local $var38 f32)
    (local $var39 f32)
    (local $var40 f32)
    (local $var41 f32)
    (local $var42 f32)
    (local $var43 f32)
    (local $var44 f32)
    (local $var45 f32)
    (local $var46 f32)
    (local $var47 f32)
    (local $var48 f32)
    block
      global.get $global2
      local.set $var8
      global.get $global2
      i32.const 64
      i32.add
      global.set $global2
      local.get $var0
      i32.eqz
      if
        i32.const 0
        i32.const 178
        i32.const 0
        call $env._alertError
        local.get $var8
        global.set $global2
        return
      end
      local.get $var1
      i32.eqz
      if
        i32.const 0
        i32.const 179
        i32.const 0
        call $env._alertError
        local.get $var8
        global.set $global2
        return
      end
      local.get $var2
      i32.eqz
      if
        i32.const 0
        i32.const 180
        i32.const 0
        call $env._alertError
        local.get $var8
        global.set $global2
        return
      end
      local.get $var2
      i32.const 8
      i32.add
      local.tee $var15
      i32.load
      i32.eqz
      if
        i32.const 0
        i32.const 181
        i32.const 0
        call $env._alertError
        local.get $var8
        global.set $global2
        return
      end
      local.get $var2
      i32.load
      i32.eqz
      if
        local.get $var8
        global.set $global2
        return
      end
      local.get $var2
      i32.const 4
      i32.add
      local.set $var16
      local.get $var8
      local.tee $var4
      i32.const 32
      i32.add
      local.set $var17
      local.get $var4
      i32.const 48
      i32.add
      local.set $var18
      local.get $var4
      i32.const 4
      i32.add
      local.set $var19
      local.get $var4
      i32.const 20
      i32.add
      local.set $var20
      local.get $var4
      i32.const 36
      i32.add
      local.set $var21
      local.get $var4
      i32.const 52
      i32.add
      local.set $var22
      local.get $var4
      i32.const 8
      i32.add
      local.set $var23
      local.get $var4
      i32.const 24
      i32.add
      local.set $var24
      local.get $var4
      i32.const 40
      i32.add
      local.set $var25
      local.get $var4
      i32.const 56
      i32.add
      local.set $var26
      local.get $var4
      i32.const 12
      i32.add
      local.set $var27
      local.get $var4
      i32.const 16
      i32.add
      local.set $var28
      local.get $var4
      i32.const 28
      i32.add
      local.set $var29
      local.get $var4
      i32.const 44
      i32.add
      local.set $var30
      local.get $var4
      i32.const 60
      i32.add
      local.set $var31
      i32.const 0
      local.set $var7
      loop $label1
        local.get $var4
        local.get $var1
        local.get $var3
        local.get $var16
        i32.load
        local.get $var7
        i32.const 2
        i32.shl
        i32.add
        i32.load
        call $func13
        local.get $var15
        i32.load
        local.set $var9
        local.get $var17
        f32.load
        local.set $var33
        local.get $var18
        f32.load
        local.set $var34
        local.get $var19
        f32.load
        local.set $var35
        local.get $var20
        f32.load
        local.set $var36
        local.get $var21
        f32.load
        local.set $var37
        local.get $var22
        f32.load
        local.set $var38
        local.get $var23
        f32.load
        local.set $var39
        local.get $var24
        f32.load
        local.set $var40
        local.get $var25
        f32.load
        local.set $var41
        local.get $var26
        f32.load
        local.set $var42
        local.get $var27
        f32.load
        local.set $var43
        local.get $var4
        f32.load
        local.set $var44
        local.get $var28
        f32.load
        local.set $var45
        local.get $var29
        f32.load
        local.set $var46
        local.get $var30
        f32.load
        local.set $var47
        local.get $var31
        f32.load
        local.set $var48
        i32.const 0
        local.set $var11
        loop $label0
          local.get $var9
          local.get $var7
          i32.const 6
          i32.shl
          i32.add
          local.get $var11
          i32.const 2
          i32.shl
          local.tee $var6
          i32.const 2
          i32.shl
          i32.add
          local.set $var10
          local.get $var0
          local.get $var7
          i32.const 6
          i32.shl
          i32.add
          local.get $var6
          i32.const 2
          i32.shl
          i32.add
          local.tee $var5
          f32.const 0.0
          f32.store
          local.get $var5
          local.get $var10
          f32.load
          local.get $var44
          f32.mul
          f32.const 0.0
          f32.add
          local.tee $var32
          f32.store
          local.get $var5
          local.get $var32
          local.get $var9
          local.get $var7
          i32.const 6
          i32.shl
          i32.add
          local.get $var6
          i32.const 1
          i32.or
          i32.const 2
          i32.shl
          i32.add
          local.tee $var12
          f32.load
          local.get $var45
          f32.mul
          f32.add
          local.tee $var32
          f32.store
          local.get $var5
          local.get $var32
          local.get $var9
          local.get $var7
          i32.const 6
          i32.shl
          i32.add
          local.get $var6
          i32.const 2
          i32.or
          i32.const 2
          i32.shl
          i32.add
          local.tee $var13
          f32.load
          local.get $var33
          f32.mul
          f32.add
          local.tee $var32
          f32.store
          local.get $var5
          local.get $var32
          local.get $var9
          local.get $var7
          i32.const 6
          i32.shl
          i32.add
          local.get $var6
          i32.const 3
          i32.or
          i32.const 2
          i32.shl
          i32.add
          local.tee $var14
          f32.load
          local.get $var34
          f32.mul
          f32.add
          f32.store
          local.get $var0
          local.get $var7
          i32.const 6
          i32.shl
          i32.add
          local.get $var6
          i32.const 1
          i32.or
          i32.const 2
          i32.shl
          i32.add
          local.tee $var5
          f32.const 0.0
          f32.store
          local.get $var5
          local.get $var10
          f32.load
          local.get $var35
          f32.mul
          f32.const 0.0
          f32.add
          local.tee $var32
          f32.store
          local.get $var5
          local.get $var32
          local.get $var12
          f32.load
          local.get $var36
          f32.mul
          f32.add
          local.tee $var32
          f32.store
          local.get $var5
          local.get $var32
          local.get $var13
          f32.load
          local.get $var37
          f32.mul
          f32.add
          local.tee $var32
          f32.store
          local.get $var5
          local.get $var32
          local.get $var14
          f32.load
          local.get $var38
          f32.mul
          f32.add
          f32.store
          local.get $var0
          local.get $var7
          i32.const 6
          i32.shl
          i32.add
          local.get $var6
          i32.const 2
          i32.or
          i32.const 2
          i32.shl
          i32.add
          local.tee $var5
          f32.const 0.0
          f32.store
          local.get $var5
          local.get $var10
          f32.load
          local.get $var39
          f32.mul
          f32.const 0.0
          f32.add
          local.tee $var32
          f32.store
          local.get $var5
          local.get $var32
          local.get $var12
          f32.load
          local.get $var40
          f32.mul
          f32.add
          local.tee $var32
          f32.store
          local.get $var5
          local.get $var32
          local.get $var13
          f32.load
          local.get $var41
          f32.mul
          f32.add
          local.tee $var32
          f32.store
          local.get $var5
          local.get $var32
          local.get $var14
          f32.load
          local.get $var42
          f32.mul
          f32.add
          f32.store
          local.get $var0
          local.get $var7
          i32.const 6
          i32.shl
          i32.add
          local.get $var6
          i32.const 3
          i32.or
          i32.const 2
          i32.shl
          i32.add
          local.tee $var6
          f32.const 0.0
          f32.store
          local.get $var6
          local.get $var10
          f32.load
          local.get $var43
          f32.mul
          f32.const 0.0
          f32.add
          local.tee $var32
          f32.store
          local.get $var6
          local.get $var32
          local.get $var12
          f32.load
          local.get $var46
          f32.mul
          f32.add
          local.tee $var32
          f32.store
          local.get $var6
          local.get $var32
          local.get $var13
          f32.load
          local.get $var47
          f32.mul
          f32.add
          local.tee $var32
          f32.store
          local.get $var6
          local.get $var32
          local.get $var14
          f32.load
          local.get $var48
          f32.mul
          f32.add
          f32.store
          local.get $var11
          i32.const 1
          i32.add
          local.tee $var11
          i32.const 4
          i32.ne
          br_if $label0
        end $label0
        local.get $var7
        i32.const 1
        i32.add
        local.tee $var7
        local.get $var2
        i32.load
        i32.lt_u
        br_if $label1
      end $label1
      local.get $var8
      global.set $global2
    end
  )
  (func $func15 (param $var0 i32) (param $var1 i32) (param $var2 i32) (param $var3 i32) (param $var4 f32) (param $var5 f32) (param $var6 f32)
    (local $var7 i32)
    (local $var8 i32)
    (local $var9 i32)
    (local $var10 i32)
    (local $var11 i32)
    (local $var12 i32)
    (local $var13 i32)
    (local $var14 i32)
    (local $var15 i32)
    (local $var16 i32)
    (local $var17 i32)
    (local $var18 i32)
    (local $var19 i32)
    (local $var20 i32)
    (local $var21 i32)
    (local $var22 i32)
    (local $var23 i32)
    (local $var24 i32)
    (local $var25 i32)
    (local $var26 i32)
    (local $var27 i32)
    (local $var28 i32)
    (local $var29 i32)
    (local $var30 i32)
    (local $var31 i32)
    (local $var32 i32)
    (local $var33 i32)
    (local $var34 i32)
    (local $var35 i32)
    (local $var36 i32)
    (local $var37 i32)
    (local $var38 i32)
    (local $var39 i32)
    (local $var40 i32)
    (local $var41 i32)
    (local $var42 i32)
    (local $var43 i32)
    (local $var44 i32)
    (local $var45 i32)
    (local $var46 i32)
    (local $var47 i32)
    (local $var48 f32)
    (local $var49 f32)
    (local $var50 f32)
    (local $var51 f32)
    (local $var52 f32)
    (local $var53 f32)
    (local $var54 f32)
    (local $var55 f32)
    (local $var56 f32)
    (local $var57 f32)
    (local $var58 f32)
    (local $var59 f32)
    (local $var60 f32)
    (local $var61 f32)
    (local $var62 f32)
    (local $var63 f32)
    (local $var64 f32)
    (local $var65 f32)
    (local $var66 f32)
    (local $var67 f32)
    (local $var68 f32)
    block
      global.get $global2
      local.set $var13
      global.get $global2
      i32.const 256
      i32.add
      global.set $global2
      local.get $var3
      i32.load
      i32.eqz
      if
        local.get $var13
        global.set $global2
        return
      end
      local.get $var13
      i32.const 64
      i32.add
      local.set $var14
      local.get $var13
      local.set $var15
      local.get $var3
      i32.const 4
      i32.add
      local.set $var16
      local.get $var3
      i32.const 8
      i32.add
      local.set $var17
      f32.const 1
      local.get $var6
      f32.sub
      local.set $var68
      local.get $var13
      i32.const 192
      i32.add
      local.tee $var7
      i32.const 16
      i32.add
      local.set $var18
      local.get $var7
      i32.const 32
      i32.add
      local.set $var19
      local.get $var7
      i32.const 48
      i32.add
      local.set $var20
      local.get $var7
      i32.const 4
      i32.add
      local.set $var21
      local.get $var7
      i32.const 20
      i32.add
      local.set $var22
      local.get $var7
      i32.const 36
      i32.add
      local.set $var23
      local.get $var7
      i32.const 52
      i32.add
      local.set $var24
      local.get $var7
      i32.const 8
      i32.add
      local.set $var25
      local.get $var7
      i32.const 24
      i32.add
      local.set $var26
      local.get $var7
      i32.const 40
      i32.add
      local.set $var27
      local.get $var7
      i32.const 56
      i32.add
      local.set $var28
      local.get $var7
      i32.const 12
      i32.add
      local.set $var29
      local.get $var7
      i32.const 28
      i32.add
      local.set $var30
      local.get $var7
      i32.const 44
      i32.add
      local.set $var31
      local.get $var7
      i32.const 60
      i32.add
      local.set $var32
      local.get $var13
      i32.const 128
      i32.add
      local.tee $var8
      i32.const 16
      i32.add
      local.set $var33
      local.get $var8
      i32.const 32
      i32.add
      local.set $var34
      local.get $var8
      i32.const 48
      i32.add
      local.set $var35
      local.get $var8
      i32.const 4
      i32.add
      local.set $var36
      local.get $var8
      i32.const 20
      i32.add
      local.set $var37
      local.get $var8
      i32.const 36
      i32.add
      local.set $var38
      local.get $var8
      i32.const 52
      i32.add
      local.set $var39
      local.get $var8
      i32.const 8
      i32.add
      local.set $var40
      local.get $var8
      i32.const 24
      i32.add
      local.set $var41
      local.get $var8
      i32.const 40
      i32.add
      local.set $var42
      local.get $var8
      i32.const 56
      i32.add
      local.set $var43
      local.get $var8
      i32.const 12
      i32.add
      local.set $var44
      local.get $var8
      i32.const 28
      i32.add
      local.set $var45
      local.get $var8
      i32.const 44
      i32.add
      local.set $var46
      local.get $var8
      i32.const 60
      i32.add
      local.set $var47
      i32.const 0
      local.set $var10
      loop $label3
        local.get $var7
        local.get $var1
        local.get $var4
        local.get $var16
        i32.load
        local.get $var10
        i32.const 2
        i32.shl
        i32.add
        i32.load
        call $func13
        local.get $var8
        local.get $var2
        local.get $var5
        local.get $var16
        i32.load
        local.get $var10
        i32.const 2
        i32.shl
        i32.add
        i32.load
        call $func13
        local.get $var17
        i32.load
        local.set $var12
        local.get $var7
        f32.load
        local.set $var52
        local.get $var18
        f32.load
        local.set $var53
        local.get $var19
        f32.load
        local.set $var54
        local.get $var20
        f32.load
        local.set $var55
        local.get $var21
        f32.load
        local.set $var56
        local.get $var22
        f32.load
        local.set $var57
        local.get $var23
        f32.load
        local.set $var58
        local.get $var24
        f32.load
        local.set $var59
        local.get $var25
        f32.load
        local.set $var60
        local.get $var26
        f32.load
        local.set $var61
        local.get $var27
        f32.load
        local.set $var62
        local.get $var28
        f32.load
        local.set $var63
        local.get $var29
        f32.load
        local.set $var64
        local.get $var30
        f32.load
        local.set $var65
        local.get $var31
        f32.load
        local.set $var66
        local.get $var32
        f32.load
        local.set $var67
        i32.const 0
        local.set $var11
        loop $label0
          local.get $var12
          local.get $var10
          i32.const 6
          i32.shl
          i32.add
          local.get $var11
          i32.const 2
          i32.shl
          local.tee $var9
          i32.const 2
          i32.shl
          i32.add
          f32.load
          local.set $var48
          local.get $var14
          local.get $var9
          i32.const 2
          i32.shl
          i32.add
          local.get $var48
          local.get $var52
          f32.mul
          f32.const 0.0
          f32.add
          local.get $var12
          local.get $var10
          i32.const 6
          i32.shl
          i32.add
          local.get $var9
          i32.const 1
          i32.or
          i32.const 2
          i32.shl
          i32.add
          f32.load
          local.tee $var49
          local.get $var53
          f32.mul
          f32.add
          local.get $var12
          local.get $var10
          i32.const 6
          i32.shl
          i32.add
          local.get $var9
          i32.const 2
          i32.or
          i32.const 2
          i32.shl
          i32.add
          f32.load
          local.tee $var50
          local.get $var54
          f32.mul
          f32.add
          local.get $var12
          local.get $var10
          i32.const 6
          i32.shl
          i32.add
          local.get $var9
          i32.const 3
          i32.or
          i32.const 2
          i32.shl
          i32.add
          f32.load
          local.tee $var51
          local.get $var55
          f32.mul
          f32.add
          f32.store
          local.get $var14
          local.get $var9
          i32.const 1
          i32.or
          i32.const 2
          i32.shl
          i32.add
          local.get $var48
          local.get $var56
          f32.mul
          f32.const 0.0
          f32.add
          local.get $var49
          local.get $var57
          f32.mul
          f32.add
          local.get $var50
          local.get $var58
          f32.mul
          f32.add
          local.get $var51
          local.get $var59
          f32.mul
          f32.add
          f32.store
          local.get $var14
          local.get $var9
          i32.const 2
          i32.or
          i32.const 2
          i32.shl
          i32.add
          local.get $var48
          local.get $var60
          f32.mul
          f32.const 0.0
          f32.add
          local.get $var49
          local.get $var61
          f32.mul
          f32.add
          local.get $var50
          local.get $var62
          f32.mul
          f32.add
          local.get $var51
          local.get $var63
          f32.mul
          f32.add
          f32.store
          local.get $var14
          local.get $var9
          i32.const 3
          i32.or
          i32.const 2
          i32.shl
          i32.add
          local.get $var48
          local.get $var64
          f32.mul
          f32.const 0.0
          f32.add
          local.get $var49
          local.get $var65
          f32.mul
          f32.add
          local.get $var50
          local.get $var66
          f32.mul
          f32.add
          local.get $var51
          local.get $var67
          f32.mul
          f32.add
          f32.store
          local.get $var11
          i32.const 1
          i32.add
          local.tee $var11
          i32.const 4
          i32.ne
          br_if $label0
        end $label0
        local.get $var17
        i32.load
        local.set $var12
        local.get $var8
        f32.load
        local.set $var52
        local.get $var33
        f32.load
        local.set $var53
        local.get $var34
        f32.load
        local.set $var54
        local.get $var35
        f32.load
        local.set $var55
        local.get $var36
        f32.load
        local.set $var56
        local.get $var37
        f32.load
        local.set $var57
        local.get $var38
        f32.load
        local.set $var58
        local.get $var39
        f32.load
        local.set $var59
        local.get $var40
        f32.load
        local.set $var60
        local.get $var41
        f32.load
        local.set $var61
        local.get $var42
        f32.load
        local.set $var62
        local.get $var43
        f32.load
        local.set $var63
        local.get $var44
        f32.load
        local.set $var64
        local.get $var45
        f32.load
        local.set $var65
        local.get $var46
        f32.load
        local.set $var66
        local.get $var47
        f32.load
        local.set $var67
        i32.const 0
        local.set $var11
        loop $label1
          local.get $var12
          local.get $var10
          i32.const 6
          i32.shl
          i32.add
          local.get $var11
          i32.const 2
          i32.shl
          local.tee $var9
          i32.const 2
          i32.shl
          i32.add
          f32.load
          local.set $var48
          local.get $var15
          local.get $var9
          i32.const 2
          i32.shl
          i32.add
          local.get $var48
          local.get $var52
          f32.mul
          f32.const 0.0
          f32.add
          local.get $var12
          local.get $var10
          i32.const 6
          i32.shl
          i32.add
          local.get $var9
          i32.const 1
          i32.or
          i32.const 2
          i32.shl
          i32.add
          f32.load
          local.tee $var49
          local.get $var53
          f32.mul
          f32.add
          local.get $var12
          local.get $var10
          i32.const 6
          i32.shl
          i32.add
          local.get $var9
          i32.const 2
          i32.or
          i32.const 2
          i32.shl
          i32.add
          f32.load
          local.tee $var50
          local.get $var54
          f32.mul
          f32.add
          local.get $var12
          local.get $var10
          i32.const 6
          i32.shl
          i32.add
          local.get $var9
          i32.const 3
          i32.or
          i32.const 2
          i32.shl
          i32.add
          f32.load
          local.tee $var51
          local.get $var55
          f32.mul
          f32.add
          f32.store
          local.get $var15
          local.get $var9
          i32.const 1
          i32.or
          i32.const 2
          i32.shl
          i32.add
          local.get $var48
          local.get $var56
          f32.mul
          f32.const 0.0
          f32.add
          local.get $var49
          local.get $var57
          f32.mul
          f32.add
          local.get $var50
          local.get $var58
          f32.mul
          f32.add
          local.get $var51
          local.get $var59
          f32.mul
          f32.add
          f32.store
          local.get $var15
          local.get $var9
          i32.const 2
          i32.or
          i32.const 2
          i32.shl
          i32.add
          local.get $var48
          local.get $var60
          f32.mul
          f32.const 0.0
          f32.add
          local.get $var49
          local.get $var61
          f32.mul
          f32.add
          local.get $var50
          local.get $var62
          f32.mul
          f32.add
          local.get $var51
          local.get $var63
          f32.mul
          f32.add
          f32.store
          local.get $var15
          local.get $var9
          i32.const 3
          i32.or
          i32.const 2
          i32.shl
          i32.add
          local.get $var48
          local.get $var64
          f32.mul
          f32.const 0.0
          f32.add
          local.get $var49
          local.get $var65
          f32.mul
          f32.add
          local.get $var50
          local.get $var66
          f32.mul
          f32.add
          local.get $var51
          local.get $var67
          f32.mul
          f32.add
          f32.store
          local.get $var11
          i32.const 1
          i32.add
          local.tee $var11
          i32.const 4
          i32.ne
          br_if $label1
          i32.const 0
          local.set $var11
        end $label1
        loop $label2
          local.get $var0
          local.get $var10
          i32.const 6
          i32.shl
          i32.add
          local.get $var11
          i32.const 2
          i32.shl
          i32.add
          local.get $var68
          local.get $var14
          local.get $var11
          i32.const 2
          i32.shl
          i32.add
          f32.load
          f32.mul
          local.get $var15
          local.get $var11
          i32.const 2
          i32.shl
          i32.add
          f32.load
          local.get $var6
          f32.mul
          f32.add
          f32.store
          local.get $var11
          i32.const 1
          i32.add
          local.tee $var11
          i32.const 16
          i32.ne
          br_if $label2
        end $label2
        local.get $var10
        i32.const 1
        i32.add
        local.tee $var10
        local.get $var3
        i32.load
        i32.lt_u
        br_if $label3
      end $label3
      local.get $var13
      global.set $global2
    end
  )
  (func $runPostSets (;16;) (export "runPostSets")
    nop
  )
  (func $__post_instantiate (;17;) (export "__post_instantiate")
    block
      global.get $env.memoryBase
      global.set $global2
      global.get $global2
      i32.const 5242880
      i32.add
      global.set $global3
      call $runPostSets
    end
  )
  (func $_getSingleAnimation (;18;) (export "_getSingleAnimation") (param $var0 i32) (param $var1 i32) (param $var2 i32) (param $var3 f64)
    local.get $var0
    local.get $var1
    local.get $var2
    local.get $var3
    f32.demote_f64
    call $func14
  )
  (func $_getBlendedAnimation (;19;) (export "_getBlendedAnimation") (param $var0 i32) (param $var1 i32) (param $var2 i32) (param $var3 i32) (param $var4 f64) (param $var5 f64) (param $var6 f64)
    local.get $var0
    local.get $var1
    local.get $var2
    local.get $var3
    local.get $var4
    f32.demote_f64
    local.get $var5
    f32.demote_f64
    local.get $var6
    f32.demote_f64
    call $func15
  )
  (func $__Z5flerpfff (;20;) (export "__Z5flerpfff") (param $var0 f64) (param $var1 f64) (param $var2 f64) (result f64)
    local.get $var0
    f32.demote_f64
    local.get $var1
    f32.demote_f64
    local.get $var2
    f32.demote_f64
    call $func5
    f64.promote_f32
  )
  (func $__Z24getAnimatedNodeTransformR4Mat4RK9Animationfj (;21;) (export "__Z24getAnimatedNodeTransformR4Mat4RK9Animationfj") (param $var0 i32) (param $var1 i32) (param $var2 f64) (param $var3 i32)
    local.get $var0
    local.get $var1
    local.get $var2
    f32.demote_f64
    local.get $var3
    call $func13
  )
  (func $__Z5vlerpR4Vec3RKS_S2_f (;22;) (export "__Z5vlerpR4Vec3RKS_S2_f") (param $var0 i32) (param $var1 i32) (param $var2 i32) (param $var3 f64)
    local.get $var0
    local.get $var1
    local.get $var2
    local.get $var3
    f32.demote_f64
    call $func4
  )
  (func $__Z18getTransformAtTimeR4Mat4RK12AnimatedBonef (;23;) (export "__Z18getTransformAtTimeR4Mat4RK12AnimatedBonef") (param $var0 i32) (param $var1 i32) (param $var2 f64)
    local.get $var0
    local.get $var1
    local.get $var2
    f32.demote_f64
    call $func12
  )
  (func $__Z6qslerpR4QuatRKS_S2_f (;24;) (export "__Z6qslerpR4QuatRKS_S2_f") (param $var0 i32) (param $var1 i32) (param $var2 i32) (param $var3 f64)
    local.get $var0
    local.get $var1
    local.get $var2
    local.get $var3
    f32.demote_f64
    call $func6
  )
)