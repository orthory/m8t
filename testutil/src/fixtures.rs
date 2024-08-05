pub const TestDocumentBuffer: &str = r"
---
rules:
- id: rule-1
  languages:
    - c
    - cpp
- id: rule-2
  languages:
    - rust
---

Hello World
asdfasf
asdfasf

- item 1
- item 2
- item 3

/comment @kjessec,123413123
hahahahhaa
/comment
/comment @kjessec,9999999
hahahahhaa
/comment
";
