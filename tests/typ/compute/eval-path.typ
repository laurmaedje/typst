// Test file loading in eval.

--- eval-path-resolve ---
// Test absolute path.
#eval("image(\"/assets/images/tiger.jpg\", width: 50%)")

--- eval-path-resolve-in-show-rule ---
#show raw: it => eval(it.text, mode: "markup")

```
#show emph: image("/assets/images/tiger.jpg", width: 50%)
_Tiger!_
```

--- eval-path-resolve-relative ---
// Test relative path.
#test(eval(`"HELLO" in read("./eval-path.typ")`.text), true)
