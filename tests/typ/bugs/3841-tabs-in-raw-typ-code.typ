// Issue 3841 Tab chars are not rendered in raw blocks with lang: "typ(c)"
// https://github.com/typst/typst/issues/3841

--- issue-3841-tabs-in-raw-type-code ---
#raw("#if true {\n\tf()\t// typ\n}", lang: "typ")

#raw("if true {\n\tf()\t// typc\n}", lang: "typc")

```typ
#if true {
	// tabs around f()
	f()	// typ
}
```

```typc
if true {
	// tabs around f()
	f()	// typc
}
```
