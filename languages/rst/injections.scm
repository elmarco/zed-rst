((doctest_block) @content
  (#set! language "python"))

; Directives with nested content without arguments nor options
((directive
  name: (type) @_type
  body: (body) @content)
  (#set! language "rst")
  (#any-of? @_type
    "attention" "caution" "danger" "error" "hint" "important" "note" "tip" "warning" "admonition"
    "line-block" "parsed-literal" "epigraph" "highlights" "pull-quote" "compound" "header" "footer"
    "meta" "replace"))

; Directives with nested content without arguments, but with options
((directive
  name: (type) @_type
  body: (body
    (options)
    (content) @content))
  (#set! language "rst")
  (#any-of? @_type
    "attention" "caution" "danger" "error" "hint" "important" "note" "tip" "warning" "admonition"
    "line-block" "parsed-literal" "compound"))

; Directives with nested content with arguments and options
((directive
  name: (type) @_type
  body: (body
    (content) @content))
  (#set! language "rst")
  (#any-of? @_type
    "figure" "topic" "sidebar" "container" "table" "list-table" "class" "role"
    "restructuredtext-test-directive"))

; Special directives
((directive
  name: (type) @_type
  body: (body
    (arguments) @language
    (content) @content))
  (#any-of? @_type "raw" "code" "code-block" "sourcecode"))

((directive
  name: (type) @_type
  body: (body
    (content) @content))
  (#set! language "latex")
  (#eq? @_type "math"))

((directive
  name: (type) @_type
  body: (body
    (content) @content))
  (#set! language "csv")
  (#eq? @_type "csv-table"))

; Special roles - prefix
((interpreted_text
  (role) @_role
  "interpreted_text" @content)
  (#eq? @_role ":math:")
  (#set! language "latex"))

; Special roles - suffix
((interpreted_text
  "interpreted_text" @content
  (role) @_role)
  (#eq? @_role ":math:")
  (#set! language "latex"))

((comment) @injection.content
  (#set! injection.language "comment"))
