// In this bug, the dot at the end was causing the right parenthesis to be
// parsed as an identifier instead of the closing right parenthesis.
// Issue: https://github.com/typst/typst/issues/2044

--- issue-2044-invalid-parsed-ident ---
$floor(phi.alt.)$
$floor(phi.alt. )$
