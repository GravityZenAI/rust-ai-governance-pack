# ERROR_PATTERNS

Biblioteca de patrones para diagnóstico rápido.

**Uso recomendado:**
- Cuando aparezca un error E0xxx: buscar aquí el patrón, aplicar fix canónico, y luego ejecutar `./scripts/verify.sh --fast`.
- Si el patrón no existe o está incompleto: agregar entrada.

---

## Índice (Top 30)

### rustc (E0xxx)
1. E0106 — missing lifetime specifier
2. E0119 — conflicting implementations
3. E0223 — ambiguous associated type
4. E0271 — type mismatch resolving associated type
5. E0277 — trait bound not satisfied
6. E0308 — mismatched types
7. E0382 — use of moved value
8. E0405 — cannot find trait in scope
9. E0425 — cannot find value in scope
10. E0432 — unresolved import
11. E0433 — failed to resolve (module/crate)
12. E0499 — multiple mutable borrows
13. E0502 — mutable + immutable borrow conflict
14. E0505 — move out because borrowed
15. E0507 — move out of borrowed content
16. E0515 — return reference to local/temporary
17. E0521 — borrowed data escapes
18. E0594 — cannot assign through `&`
19. E0596 — cannot borrow as mutable (missing `mut`)
20. E0597 — does not live long enough
21. E0599 — method not found
22. E0609 — no such field
23. E0614 — cannot dereference
24. E0621 — explicit lifetime required
25. E0716 — temporary value dropped while borrowed
26. E0782 — trait objects must include `dyn`

### Clippy (lints)
27. clippy::unwrap_used
28. clippy::clone_on_copy
29. clippy::needless_return
30. clippy::redundant_closure

---

## Patrones

### E0106 — missing lifetime specifier

**Causa raíz:** la firma retorna una referencia, pero no hay lifetime explícito y el elision no aplica.

**Fix canónico:**
- Retornar ownership (`String` en vez de `&String`), o
- Introducir `'a` y vincularlo a un input.

**Mínimo repro (❌):**
```rust
fn dangle() -> &String {
    let s = String::from("x");
    &s
}
```

**Fix (✅):**
```rust
fn no_dangle() -> String {
    String::from("x")
}
```

**Test mínimo:**
```rust
#[test]
fn t_no_dangle() {
    assert_eq!(no_dangle(), "x");
}
```

---

### E0119 — conflicting implementations

**Causa raíz:** dos `impl Trait for Type` que se solapan.

**Fix canónico:**
- Elimina el impl duplicado, o
- Restringe con generics/bounds para que no se solapen.

**Mínimo repro (❌):**
```rust
trait MyTrait {}
struct X;

impl MyTrait for X {}
impl MyTrait for X {} // conflict
```

**Fix (✅):**
```rust
trait MyTrait {}
struct X;
impl MyTrait for X {}
```

**Test mínimo:**
```rust
#[test]
fn t_conflict_removed() {
    // compila == pasa
    assert!(true);
}
```

---

### E0223 — ambiguous associated type

**Causa raíz:** se refiere a un tipo asociado sin desambiguar el trait.

**Fix canónico:** usar UFCS / fully-qualified syntax:
`<Type as Trait>::Assoc`.

**Mínimo repro (❌):**
```rust
trait T { type A; }
struct S;

fn f(_: S::A) {} // ambiguous
```

**Fix (✅):**
```rust
trait T { type A; }
struct S;
impl T for S { type A = i32; }

fn f(_: <S as T>::A) {}
```

**Test mínimo:**
```rust
#[test]
fn t_assoc_type() {
    f(1);
}
```

---

### E0271 — type mismatch resolving associated type

**Causa raíz:** el tipo asociado resuelto no coincide con el esperado por un bound.

**Fix canónico:**
- Ajustar el bound, o
- Proveer el associated type correcto en el `impl`, o
- Convertir/adaptar el tipo.

**Mínimo repro (❌):**
```rust
trait IterLike { type Item; fn next(&mut self) -> Option<Self::Item>; }
struct It;
impl IterLike for It {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> { Some(1) }
}

fn wants_string<I: IterLike<Item = String>>(_: &mut I) {}
fn main() {
    let mut it = It;
    wants_string(&mut it);
}
```

**Fix (✅):**
```rust
fn wants_i32<I: IterLike<Item = i32>>(_: &mut I) {}

#[test]
fn t_assoc_match() {
    let mut it = It;
    wants_i32(&mut it);
}
```

---

### E0277 — trait bound not satisfied

**Causa raíz:** se requiere `T: Trait` pero `T` no lo implementa.

**Fix canónico:**
- Implementar el trait, o
- Cambiar el tipo, o
- Agregar el bound genérico correcto.

**Mínimo repro (❌):**
```rust
fn clone_twice<T: Clone>(x: &T) -> (T, T) {
    (x.clone(), x.clone())
}

struct NoClone;

fn main() {
    let x = NoClone;
    let _ = clone_twice(&x);
}
```

**Fix (✅):**
```rust
#[derive(Clone)]
struct NoClone;

#[test]
fn t_trait_bound() {
    let x = NoClone;
    let _ = clone_twice(&x);
}
```

---

### E0308 — mismatched types

**Causa raíz:** el tipo real difiere del tipo esperado.

**Fix canónico:**
- Conversión explícita (`as`, `into()`, `From`/`TryFrom`), o
- Ajustar la firma.

**Mínimo repro (❌):**
```rust
fn add_one(x: i32) -> i32 { x + 1 }

fn main() {
    let x: u32 = 1;
    let _ = add_one(x);
}
```

**Fix (✅):**
```rust
fn add_one_u32(x: u32) -> u32 { x + 1 }

#[test]
fn t_mismatched_types() {
    assert_eq!(add_one_u32(1), 2);
}
```

---

### E0382 — use of moved value

**Causa raíz:** un valor fue movido y luego se intenta usar el binding original.

**Fix canónico:**
- Prestar (`&T`) en vez de mover, o
- Clonar si el costo es aceptable, o
- Reestructurar para consumir una sola vez.

**Mínimo repro (❌):**
```rust
fn main() {
    let s = String::from("hi");
    let t = s;         // move
    println!("{s}");   // use after move
}
```

**Fix (✅):**
```rust
fn takes_ref(s: &str) -> usize { s.len() }

#[test]
fn t_use_after_move_fix() {
    let s = String::from("hi");
    let n = takes_ref(&s);
    assert_eq!(n, 2);
    // s todavía válido
    assert_eq!(s, "hi");
}
```

---

### E0405 — cannot find trait in scope

**Causa raíz:** trait no está importado o no existe.

**Fix canónico:**
- Importar (`use path::Trait;`), o
- Referenciarlo con ruta completa.

**Mínimo repro (❌):**
```rust
fn main() {
    let _ = 1u8.to_string();
    // si intentas usar un trait no importado (ej. FromStr sin use) en contexto específico,
    // aparecerá este tipo de error.
}
```

**Fix (✅):**
```rust
use std::str::FromStr;

#[test]
fn t_trait_in_scope() {
    let n = i32::from_str("42").unwrap();
    assert_eq!(n, 42);
}
```

---

### E0425 — cannot find value in scope

**Causa raíz:** nombre no existe en scope (typo, shadowing, no `let`).

**Fix canónico:**
- Definir el binding correcto, o
- Corregir nombre.

**Mínimo repro (❌):**
```rust
fn main() {
    let x = 1;
    println!("{}", y);
}
```

**Fix (✅):**
```rust
#[test]
fn t_value_in_scope() {
    let x = 1;
    assert_eq!(x, 1);
}
```

---

### E0432 — unresolved import

**Causa raíz:** ruta incorrecta o módulo no expuesto.

**Fix canónico:**
- Revisar `mod`, `pub mod`, y paths.
- Usar `crate::` desde 2018+.

**Mínimo repro (❌):**
```rust
use crate::does_not_exist::Thing;
```

**Fix (✅):**
```rust
mod m {
    pub struct Thing;
}

use crate::m::Thing;

#[test]
fn t_import_ok() {
    let _ = Thing;
}
```

---

### E0433 — failed to resolve (module/crate)

**Causa raíz:** se refiere a un módulo/crate no declarado o no accesible.

**Fix canónico:**
- Agregar dependencia en `Cargo.toml`, o
- Declarar `mod`, o
- Corregir path.

**Mínimo repro (❌):**
```rust
fn main() {
    let _ = serde::Serialize;
}
```

**Fix (✅):**
```rust
#[test]
fn t_resolve_fix() {
    // Si necesitas serde, decláralo en Cargo.toml.
    // Este test solo recuerda la regla.
    assert!(true);
}
```

---

### E0499 — cannot borrow as mutable more than once at a time

**Causa raíz:** dos préstamos mutables simultáneos al mismo valor.

**Fix canónico:**
- Separar scopes.
- Usar `split_at_mut` / iteradores para obtener referencias disjuntas.

**Mínimo repro (❌):**
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let a = &mut v[0];
    let b = &mut v[1];
    *a += *b;
}
```

**Fix (✅):**
```rust
#[test]
fn t_two_mut_borrows_fix() {
    let mut v = vec![1, 2, 3];
    let (left, right) = v.split_at_mut(1);
    left[0] += right[0];
    assert_eq!(v, vec![3, 2, 3]);
}
```

---

### E0502 — mutable + immutable borrow conflict

**Causa raíz:** se intenta mutar mientras existe un borrow inmutable activo (o viceversa).

**Fix canónico:**
- Reordenar para que el borrow termine antes de la mutación.
- Copiar/extraer el dato necesario antes.

**Mínimo repro (❌):**
```rust
fn main() {
    let mut x = 1;
    let r = &x;      // immutable borrow
    x += 1;          // mutable borrow
    println!("{}", r);
}
```

**Fix (✅):**
```rust
#[test]
fn t_borrow_conflict_fix() {
    let mut x = 1;
    let r = x; // copia (Copy)
    x += 1;
    assert_eq!(r, 1);
    assert_eq!(x, 2);
}
```

---

### E0505 — cannot move out because it is borrowed

**Causa raíz:** intentas mover un valor mientras existe un préstamo activo sobre él.

**Fix canónico:**
- Terminar el borrow antes del move (scope), o
- Clonar.

**Mínimo repro (❌):**
```rust
fn main() {
    let s = String::from("hi");
    let r = &s;
    let t = s; // move while borrowed
    println!("{r}");
}
```

**Fix (✅):**
```rust
#[test]
fn t_move_while_borrowed_fix() {
    let s = String::from("hi");
    let t = s.clone();
    let r = &s;
    assert_eq!(r, "hi");
    assert_eq!(t, "hi");
}
```

---

### E0507 — cannot move out of borrowed content

**Causa raíz:** intentas mover un campo desde un `&T`.

**Fix canónico:**
- Prestar el campo (`&t.field`), o
- Clonar, o
- Cambiar a `&mut` y usar `std::mem::take`.

**Mínimo repro (❌):**
```rust
struct S { name: String }

fn take_name(s: &S) -> String {
    s.name // move out of borrowed content
}
```

**Fix (✅):**
```rust
struct S { name: String }

fn name_ref(s: &S) -> &str { &s.name }

#[test]
fn t_move_out_of_borrowed_fix() {
    let s = S { name: "a".to_string() };
    assert_eq!(name_ref(&s), "a");
}
```

---

### E0515 — cannot return reference to local/temporary

**Causa raíz:** se retorna referencia a algo que vive menos que el retorno.

**Fix canónico:** retornar ownership o recibir referencia externa.

**Mínimo repro (❌):**
```rust
fn bad() -> &str {
    let s = String::from("x");
    &s
}
```

**Fix (✅):**
```rust
fn good() -> String {
    "x".to_string()
}

#[test]
fn t_return_local_ref_fix() {
    assert_eq!(good(), "x");
}
```

---

### E0521 — borrowed data escapes outside of function

**Causa raíz:** una referencia con lifetime corto “escapa” (p.ej., se guarda en algo con lifetime más largo).

**Fix canónico:**
- Cambiar a ownership (clonar/poseer), o
- Ajustar lifetimes/struct para que el lifetime real sea el correcto.

**Mínimo repro (❌):**
```rust
fn push_str<'a>(v: &mut Vec<&'a str>, s: &str) {
    v.push(s); // s no necesariamente vive 'a
}
```

**Fix (✅):**
```rust
fn push_owned(v: &mut Vec<String>, s: &str) {
    v.push(s.to_string());
}

#[test]
fn t_borrow_escape_fix() {
    let mut v = vec![];
    let s = String::from("hi");
    push_owned(&mut v, &s);
    assert_eq!(v[0], "hi");
}
```

---

### E0594 — cannot assign through `&`

**Causa raíz:** se intenta asignar a través de una referencia inmutable.

**Fix canónico:**
- Cambiar a `&mut`, o
- Usar interior mutability (`Cell/RefCell`) si aplica.

**Mínimo repro (❌):**
```rust
fn inc(x: &i32) {
    *x += 1;
}
```

**Fix (✅):**
```rust
fn inc(x: &mut i32) {
    *x += 1;
}

#[test]
fn t_assign_through_ref_fix() {
    let mut x = 1;
    inc(&mut x);
    assert_eq!(x, 2);
}
```

---

### E0596 — cannot borrow as mutable (missing `mut`)

**Causa raíz:** binding no declarado `mut`.

**Fix canónico:** marcar `mut`.

**Mínimo repro (❌):**
```rust
fn main() {
    let v = vec![1,2];
    v.push(3);
}
```

**Fix (✅):**
```rust
#[test]
fn t_missing_mut_fix() {
    let mut v = vec![1,2];
    v.push(3);
    assert_eq!(v, vec![1,2,3]);
}
```

---

### E0597 — does not live long enough

**Causa raíz:** una referencia vive más que el valor referenciado.

**Fix canónico:**
- Reordenar scopes.
- Introducir bindings intermedios para extender vida.

**Mínimo repro (❌):**
```rust
fn main() {
    let r: &str;
    {
        let s = String::from("hi");
        r = &s;
    }
    println!("{r}");
}
```

**Fix (✅):**
```rust
#[test]
fn t_lifetime_scope_fix() {
    let s = String::from("hi");
    let r = s.as_str();
    assert_eq!(r, "hi");
}
```

---

### E0599 — method not found

**Causa raíz:** el método no existe para el tipo; suele ser un error de tipo, trait no importado, o método en otro tipo.

**Fix canónico:**
- Revisa el tipo real.
- Importa el trait si el método viene de un trait.
- Usa el método correcto (`iter()` vs `into_iter()`, etc.).

**Mínimo repro (❌):**
```rust
fn main() {
    let v = vec![1,2,3];
    v.map(|x| x + 1);
}
```

**Fix (✅):**
```rust
#[test]
fn t_method_not_found_fix() {
    let v = vec![1,2,3];
    let out: Vec<i32> = v.iter().map(|x| x + 1).collect();
    assert_eq!(out, vec![2,3,4]);
}
```

---

### E0609 — no such field

**Causa raíz:** acceso a campo inexistente (typo o tipo diferente).

**Fix canónico:** corregir nombre o estructura.

**Mínimo repro (❌):**
```rust
struct S { a: i32 }
fn main() {
    let s = S { a: 1 };
    let _ = s.b;
}
```

**Fix (✅):**
```rust
#[test]
fn t_no_field_fix() {
    struct S { a: i32 }
    let s = S { a: 1 };
    assert_eq!(s.a, 1);
}
```

---

### E0614 — cannot dereference

**Causa raíz:** `*x` usado en algo que no es un puntero/referencia.

**Fix canónico:**
- Remover deref, o
- Tomar referencia primero.

**Mínimo repro (❌):**
```rust
fn main() {
    let x = 1;
    let _ = *x;
}
```

**Fix (✅):**
```rust
#[test]
fn t_deref_fix() {
    let x = 1;
    let r = &x;
    assert_eq!(*r, 1);
}
```

---

### E0621 — explicit lifetime required

**Causa raíz:** el compilador no puede inferir lifetime en una función con referencias.

**Fix canónico:** agregar `'a` explícito.

**Mínimo repro (❌):**
```rust
fn id(x: &str) -> &str { x }
```

**Fix (✅):**
```rust
fn id<'a>(x: &'a str) -> &'a str { x }

#[test]
fn t_explicit_lifetime_fix() {
    let s = String::from("x");
    assert_eq!(id(&s), "x");
}
```

---

### E0716 — temporary value dropped while borrowed

**Causa raíz:** una referencia apunta a un temporal que se destruye demasiado pronto.

**Fix canónico:** asignar el temporal a un binding con vida suficiente.

**Mínimo repro (❌):**
```rust
fn main() {
    let r = &String::from("hi");
    println!("{r}");
}
```

**Fix (✅):**
```rust
#[test]
fn t_temporary_dropped_fix() {
    let s = String::from("hi");
    let r = &s;
    assert_eq!(r, "hi");
}
```

---

### E0782 — trait objects must include `dyn`

**Causa raíz:** en ediciones modernas, trait objects deben escribirse `dyn Trait`.

**Fix canónico:** añadir `dyn`.

**Mínimo repro (❌):**
```rust
trait T { fn f(&self) {} }
fn takes(_: Box<T>) {}
```

**Fix (✅):**
```rust
trait T { fn f(&self) {} }
fn takes(_: Box<dyn T>) {}

#[test]
fn t_dyn_trait_object() {
    struct X;
    impl T for X {}
    takes(Box::new(X));
}
```

---

## Clippy lints

Estos patrones son “errores de calidad” que preferimos cortar temprano.

### clippy::unwrap_used

**Causa raíz:** uso de `.unwrap()` en `Option/Result` → posible `panic!`.

**Fix canónico:** `?`, `match`, o mapping a error de dominio.

**Mínimo repro (❌):**
```rust
fn parse_u32(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}
```

**Fix (✅):**
```rust
fn parse_u32(s: &str) -> Result<u32, std::num::ParseIntError> {
    s.parse::<u32>()
}

#[test]
fn t_unwrap_used_fix() {
    assert_eq!(parse_u32("42").unwrap(), 42);
}
```

---

### clippy::clone_on_copy

**Causa raíz:** clonar un tipo `Copy` es ruido.

**Fix canónico:** copiar (`let y = x;`).

**Mínimo repro (❌):**
```rust
fn f(x: i32) -> i32 {
    x.clone()
}
```

**Fix (✅):**
```rust
fn f(x: i32) -> i32 { x }

#[test]
fn t_clone_on_copy_fix() {
    assert_eq!(f(3), 3);
}
```

---

### clippy::needless_return

**Causa raíz:** `return` explícito al final de una función/bloque cuando no aporta.

**Fix canónico:** usar tail expression.

**Mínimo repro (❌):**
```rust
fn f() -> i32 {
    return 1;
}
```

**Fix (✅):**
```rust
fn f() -> i32 { 1 }

#[test]
fn t_needless_return_fix() {
    assert_eq!(f(), 1);
}
```

---

### clippy::redundant_closure

**Causa raíz:** closure que solo llama a otra función sin modificar args.

**Fix canónico:** pasar el fn item directamente.

**Mínimo repro (❌):**
```rust
fn inc(x: i32) -> i32 { x + 1 }

fn map(v: &[i32]) -> Vec<i32> {
    v.iter().copied().map(|x| inc(x)).collect()
}
```

**Fix (✅):**
```rust
fn map(v: &[i32]) -> Vec<i32> {
    v.iter().copied().map(inc).collect()
}

#[test]
fn t_redundant_closure_fix() {
    assert_eq!(map(&[1,2,3]), vec![2,3,4]);
}
```

