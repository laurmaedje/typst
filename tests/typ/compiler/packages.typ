// Test package imports

--- import-from-package-bare ---
// Test import without items.
#import "@test/adder:0.1.0"
#test(adder.add(2, 8), 10)

--- import-from-package-items ---
// Test import with items.
#import "@test/adder:0.1.0": add
#test(add(2, 8), 10)

--- import-from-package-required-compiler-version ---
// Test too high required compiler version.
// Error: 9-29 package requires typst 1.0.0 or newer (current version is VERSION)
#import "@test/future:0.1.0": future

--- import-from-package-namespace-invalid-1 ---
// Error: 9-13 `@` is not a valid package namespace
#import "@@": *

--- import-from-package-name-missing-1 ---
// Error: 9-16 package specification is missing name
#import "@heya": *

--- import-from-package-namespace-invalid-2 ---
// Error: 9-15 `123` is not a valid package namespace
#import "@123": *

--- import-from-package-name-missing-2 ---
// Error: 9-17 package specification is missing name
#import "@test/": *

--- import-from-package-version-missing-1 ---
// Error: 9-22 package specification is missing version
#import "@test/mypkg": *

--- import-from-package-name-invalid ---
// Error: 9-20 `$$$` is not a valid package name
#import "@test/$$$": *

--- import-from-package-version-missing-2 ---
// Error: 9-23 package specification is missing version
#import "@test/mypkg:": *

--- import-from-package-version-missing-minor ---
// Error: 9-24 version number is missing minor version
#import "@test/mypkg:0": *

--- import-from-package-version-major-invalid-1 ---
// Error: 9-29 `latest` is not a valid major version
#import "@test/mypkg:latest": *

--- import-from-package-version-major-invalid-2 ---
// Error: 9-29 `-3` is not a valid major version
#import "@test/mypkg:-3.0.0": *

--- import-from-package-version-missing-patch-1 ---
// Error: 9-26 version number is missing patch version
#import "@test/mypkg:0.3": *

--- import-from-package-version-missing-patch-2 ---
// Error: 9-27 version number is missing patch version
#import "@test/mypkg:0.3.": *

--- import-from-file-package-lookalike ---
// Error: 9-28 file not found (searched at typ/compiler/#test/mypkg:1.0.0)
#import "#test/mypkg:1.0.0": *
