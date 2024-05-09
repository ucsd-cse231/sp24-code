# README
Public code for snek compilers for SP24


## Week 3

- [] `input`
    - in0.snek
    - in1.snek

- [] `bool`
    - true.snek
    - false.snek
- [] `eq`
    - eq0.snek
    - eq1.snek
    - eq2.snek
- [] `if-bool`
    - if2.snek


- [] type tests
    - [] check on `add`
    - [] check on `if`
    - [] check on `eq`

    - runtime/start.rs [ function to call ]
    - src/main.rs      [ label to jump ]
    - src/main.rs      [ actual tests ]

    - ty0.snek
    - ty1.snek
    - ty2.snek

- [] set!
    - set0.snek
    - set1.snek
- [] block
    - block0.snek
- [] loop!
- [] break
    - loop0.snek

- [] print

## Week 5: Data on the Heap

- [x] tests
- [x] types
- [x] parser
- [x] strategy
    - [x] runtime
    - [x] alloc
    - [x] print
    - [x] access

## Week 6: Closures

```clojure
(let (f (fn (x) (+ x 1)))
   (f 10)
)








### Start

- Refactor grammar and types
- todo! for `Defn` and `Call`

### Label

Use code label as closure value

- **compile-defn:** jump to end, save start-label in rax

```
    jmp fun_finish_{f}
fun_start_{f}:
    {fun_entry}
fun_body_{f}:
    {body_code}
fun_exit_{f}:
    {fun_exit}
fun_finish_{f}:
    mov rax, fun_start_{f}"
```

- **compile-call:** load callee into `rax` before call

```
{eval_args}
{push_args}
{eval_f}
push rax
call rax
{pop_args}"
```

- `lam0.snek`
- `lam1.snek`
- `lam2.snek`

### Label + Self

Pass "self" as 0th arg to handle recursion

- **compile-defn:** change `init_env`
- **compile-call:** `push rax` before call

- `lam-fac.snek`

### Label + Self + Arity

- label + arity
  Yikes seg fault.

- **compile-defn:** change `init_env`
    - closure = (label, arity)

```
  jmp fun_finish_{f}
fun_start_{f}:
  {fun_entry}
fun_body_{f}:
  {body_code}
fun_exit_{f}:
  {fun_exit}
fun_finish_{f}:
  {alloc_tuple}
  add rax, 5
```


- **compile-call:** `push rax` before call

```
{eval_args}
{push_args}
{eval_f}
{test_closure}
{test_arity}
push rax
sub rax, 5
call [rax]
{pop_args}
```

- `lam-arity.snek`

### Label + Self + Arity + Free

- free_vars
- expr_vars (should add free_vars)

- **compile-defn**
    - tuple + "alloc_free_vars"
    - body  + "restore_free_vars"

- **compile-call**
    - same!

```
(let* ((one 1)
       (f   (fn (it) (it 5)))
       (inc (fn (n) (+ n one))))
  (f inc))
```

```
(let* ((add (fn (n) (fn (m) (+ n m))))
       (f   (fn (it) (it 5)))
       (plus1  (add 1))
       (plus10 (add 10)))
  (vec (f plus1) (f plus10)))
```


- [] tests
- [] types
- [] parser
- [] strategy
    - [] repr       (0x101 so add 5, strip 5)
    - [] runtime
    - [] label
    - [] label + arity
    - [] label + arity + env
