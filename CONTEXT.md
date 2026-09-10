# RustCalculator

A small Rust calculator with a shared expression evaluator, a CLI (REPL + one-shot), and a native `egui`/`eframe` GUI.

## Language

**Operator key**:
A calculator button that performs a binary arithmetic operation on the current expression: `÷`, `×`, `−`, `+`, `%`, `^`. Styled with an accent color to signal it acts mathematically on the expression, distinct from a Control key.
_Avoid_: math button, function key

**Control key**:
A calculator button that manages the expression buffer or input structure rather than computing a value: `C` (clear), `Del` (backspace), `(`, `)`. Styled with a muted, neutral color.
_Avoid_: function key, utility key

**Equals key**:
The `=` button. Triggers evaluation of the current expression via the shared evaluator and replaces the display with the result (or an error). Given the most visually prominent accent color of any key.

**Number key**:
Digits `0`–`9` and the decimal point `.`. Styled with the neutral, lowest-emphasis color.
