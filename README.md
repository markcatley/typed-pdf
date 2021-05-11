# Typed PDF

A nursery of additional types for PDF-RS (https://github.com/pdf-rs/pdf).

It currently contains an enum for page content operations (https://docs.rs/pdf/0.7.1/pdf/content/struct.Operation.html) and a binary that will attempt to parse a set of PDF files and print operations that it can't type.

At the moment, operations have the following issues:

- Text is parsed into rust strings, if they cannot be parsed into rust strings they're considered unknown operations. This happens a lot more regularly that I had expected.
- The following operations are not implemented, they're mostly property lists and binary data:
  
  - BDC
  - BI
  - BMC
  - BX
  - DP
  - EI
  - EMC
  - EX
  - ID
