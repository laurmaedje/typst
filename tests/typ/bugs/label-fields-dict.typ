// Tests whether the label is accessible through the has, field,
// and fields accessors

--- issue-content-label-has-method ---
// Test whether the label is accessible through the has method
#show heading: it => {
  assert(it.has("label"))
  it
}

= Hello, world! <my_label>

--- issue-content-label-field-access ---
// Test whether the label is accessible through the field method
#show heading: it => {
  assert(str(it.label) == "my_label")
  it
}

= Hello, world! <my_label>

--- issue-content-label-fields-method ---
// Test whether the label is accessible through the fields method
#show heading: it => {
  assert("label" in it.fields())
  assert(str(it.fields().label) == "my_label")
  it
}

= Hello, world! <my_label>
