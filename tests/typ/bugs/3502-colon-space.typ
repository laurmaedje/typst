// Test that a space after a named parameter is permissible.
// https://github.com/typst/typst/issues/3502

--- issue-3502-space-around-param-colon ---
#let f( param : v ) = param
#test(f( param /* ok */ : 2 ), 2)

--- issue-3502-space-and-comments-around-destructuring-colon ---
#let ( key :  /* hi */ binding ) = ( key: "ok" )
#test(binding, "ok")

--- issue-3502-space-around-dict-colon ---
#test(( key : "value" ).key, "value")
