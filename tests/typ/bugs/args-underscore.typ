// Test that lone underscore works.

--- args-lone-underscore ---
#test((1, 2, 3).map(_ => {}).len(), 3)
