---
marp: true
theme: default
class: invert
backgroundImage:
paginate: true
---

<!-- Some CSS styling for the page numbers. Ignore this. -->
<style>
section::after {
 content: attr(data-marpit-pagination) '/' attr(data-marpit-pagination-total);
}
</style>

<!-- More styling for image positions. -->
<style>
img[alt~="top-right"] {
  position: absolute;
  top: 30px;
  right: 30px;
  height: 10%
}
</style>

<style>
img[alt~="top-right-med"] {
  position: absolute;
  top: 30px;
  right: 30px;
  height: 25%
}
</style>

<style>
img[alt~="top-right-big"] {
  position: absolute;
  top: 30px;
  right: 30px;
  height: 40%
}
</style>

<style>
img[alt~="top-left"] {
  position: absolute;
  top: 30px;
  left: 30px;
  height: 10%
}
</style>

<style>
img[alt~="top-left-med"] {
  position: absolute;
  top: 30px;
  left: 30px;
  height: 25%
}
</style>

<style>
img[alt~="top-left-big"] {
  position: absolute;
  top: 30px;
  left: 30px;
  height: 40%
}
</style>

<style>
img[alt~="bottom-right"] {
  position: absolute;
  bottom: 30px;
  right: 30px;
  height: 10%
}
</style>

<style>
img[alt~="bottom-left"] {
  position: absolute;
  bottom: 30px;
  left: 30px;
  height: 10%
}
</style>

<!-- paginate: skip -->

# Rust lernen, aber wie?

Christopher Hock
Mail: byteotter@gmail.com
Website: [byteotter.gay](https://byteotter.gay)
Matrix: @chris:kde.org

![bg right 100%](../assets/not_committing/old_man_yells_at_rust.jpg)

<!-- _footer: "Slides
https://github.com/ByteOtter/talks/LDC_24/ \n  © 2024 by Christopher Hock. Licensed under CC BY-NC-SA 4.0"
-->

---

# Wer bin ich?

![bg right](../assets/images/misc/foss_presentation/otter.jpg)

- Chris
- 25 Jahre alt
- Azubi bei SUSE
- Programmieren in der Uni angefangen
- Größtenteils Python Erfahrung
- Open Source contributions seit 2021/22

---

# Index

1. Rusts "Lernkurve"
2. Ein paar wichtige Konzepte
3. Der Compiler ist dein Freund
4. Ressourcen und Tipps

---

# Frage: Sollte man Rust als Anfänger lernen?

---

# Nein.

---

# Doch ...

---

<!-- paginate: true -->

# Die Lernerfahrung

![bg right](./learning_experience.gif)

---

# Die Lernerfahrung

## Was ist Rust?

- System-level Sprache mit modernen Features

---

# Die Lernerfahrung

- Was es nicht gibt:
  - Objektorientierung
  - Garbage Collection
  - Implizites `None`
  - Global state
  - Null Pointers (Dank Options)

---

# Die Lernerfahrung

- Dinge, die es gibt
  - Ownership Prinzip
  - Borrow-Checker
  - Immutability by default
  - Speichersicherheit dank Ownership / Borrowing
  - Null Safety

---

# Wichtige Konzepte

---

# [Ownership](https://www.youtube.com/watch?v=DJdUjjOmyx8)

> Satz an Regeln, die Speichersicherheit ohne Garbage Collection ermöglichen

- Alle Daten haben ihren Besitzer / Owner
- Es kann nur einen Owner geben
- Geht der Besitzer Out-of-Scope, wird der Wert aufgeräumt
- Regeln werden zu Compiletime geprüft

---

# Ownership (Beispiel)

Python
```python
def some_func(x: str):
    # ...

x: str = "Hello"

some_func(x)

print(x)
```
---

# Ownership (Beispiel)

Rust
```rust
fn main() {
  let x: String = String::from("Hello");

  some_func(x); // Ownership wird an some_func abgegeben

  println!("{}", x); // Error
}
```
```
error[E0382]: borrow of moved value: `x`
  --> src/main.rs:10:20
   |
6  |     let x: String = "Hello".to_string();
   |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
7  |
8  |     some_func(x);
   |               - value moved here
9  |
10 |     println!("{}", x);
   |                    ^ value borrowed here after move
   |
note: consider changing this parameter type in function `some_func` to borrow instead if owning the value isn't necessary
```

---

# Ownership (Beispiel)

Rust (fixed)
```rust
fn some_func(x: &str) {...}

fn main() {
  let x: String = String::from("Hello");
  some_func(&x);
  println!("{}", x);
}
```

---

## Der Borrow-Checker

![bg right](./knifehand.gif)

---

## Der Borrow-Checker

- Teil des Rust Compilers
- Überprüft, ob die Ownership Regeln eingehalten werden
- Ähnlich wie Adress-Sanitizer für C/C++, aber nicht optional
- Eine der häufigsten Quellen für Compilerfehler

---

# Die String Typen

---

# Die String Typen

Zwei wichtige Typen:

- `String`
  - Klassischer Stringtyp als `Vec<u8>`
  - Dynamisch veränderbar
  - heap-alloziert
  - UTF-8 kodiert
  - Nicht null-terminiert
- `&str` - Der "String slice"
  - Referenz auf UTF-8 Byte Sequenz
  - *Keine Ownership*

<!--Referenz auf statischen Memory-->

---

# Die String Typen
## Warum `&str`?

- Ermöglichen die Arbeit mit Teilen von Strings
- Erlaubt Flexibilität in der Arbeit mit Strings oder String-literalen
- Keine Ownership
- Aber: Konstante Referenz. Änderungen nicht möglich. (Read-Only)

---

# Generics

- Platzhalter für beliebigen Datentypen, der bestimmtes `Trait` implementiert
- Die Art möglicher Datentypen kann eingeschränkt werden (`Traits`)

```rust
fn write_output<T: std::format::Display>(parameter: T) -> Result<(), IOError> {
    // ...
}
```

---

# Typed Enums

- Auswahl an möglichen Datentypen
- Erlauben die Generalisierung von Funktionen
- Beschränkung eines Variablentyps auf eine Anzahl von Möglichkeiten

<!--
- Kann verschieden States (Variants) haben
-->

---

# Null Safety

- Zwei spezielle enums: `Result<T, E>`, `Option<T>`
- Beide geben Auskunft über Erfolg/Misserfolg
- `Result<T, E>`:
    - Entweder `Ok(T)`
    - Oder `Err(E)`
    => Zwingt zum expliziten Error Handling.
- `Option<T>`
    - Entweder `Some(T)`
    - Oder `None`
    => Verhindert nicht behandelte Null states.

<!--
- Es gibt nie einen invalden state (außer unsafe dann selbst schuld)
-->

---

# Null Safety & Error Handling

```rust
fn get_devices() -> Result<Vec<Devices>, ApiError> {
    // ...
}

fn main() {
    let devices: Vec<Devices> = match get_devices() {
        Ok(device_list) => device_list,
        Err(err) => {
            panic!("Error occured while retrieving device list: {}", err);
        }
    }
}
```

---

# Tipps & Strategien

- Nicht einschüchtern lassen
- Compilerfehler beachten (`rustc --explain`)
- Fokus auf die obigen Konzepte
- [`rustlings`](https://github.com/rust-lang/rustlings) 
- Nicht scheuen einfache Lösungen zu verwenden

---

# Der Compiler ist dein Freund

![bg right 70%](../assets/not_committing/love_companion_rustc.jpg)

<!-- _footer: Original Companion Cube image by CalicoStonewolf @ Devientart-->

---

# Der compiler ist dein Freund

Findet den Fehler:
```rust
fn main() {
    let x = 5;
    x = 3;

    println!("{}", x);
}
```

---

# Der Compiler ist dein Freund

```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     x = 8;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
warning: `error_examples` (bin "error_examples") generated 2 warnings
error: could not compile `error_examples` (bin "error_examples") due to 1 previous error; 2 warnings emitted
```

- gibt die fehlerhafte Codestellen aus
- Schlägt Lösungen vor

---

# Ressourcen

- Rust Book ([Online-Buch](https://doc.rust-lang.org/stable/book/) | [Interaktiver Guide](https://rust-book.cs.brown.edu/))
- Rustlings Übungsaufgaben
- Videokurse:
    - [Let's get Rusty](https://www.youtube.com/watch?v=usJDUSrcwqI)
    - Low Level Learning
- Rust in der Praxis:
    - [Jon Gjengset](https://www.youtube.com/@jonhoo/videos)
- Codebeispiele in [`Rust by Example`](https://doc.rust-lang.org/stable/rust-by-example/)
- Umsetzungsbeispiele & Snippets im [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)

---

# Abschluss

- Rust hat eine hohe Lernkurve
- Verwendet Lösungen, die zu eurem Kenntnisstand passen
- Seid offen zu lernen und geht Fehlern auf den Grund

---

# Vielen Dank!

Folien auf https://github.com/ByteOtter/talks/LDC_24/
