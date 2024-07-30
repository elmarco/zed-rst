(title) @title

; Markup
(emphasis) @emphasis
(strong) @emphasis.strong
(standalone_hyperlink) @link_uri

; Lists
; Definition lists
(list_item
  (term) @emphasis.strong
  (classifier)? @emphasis)

; Field lists
(field
  (field_name) @constant)

; Roles
(role) @function

((role) @function.builtin
  (#any-of? @function.builtin
    ; https://docutils.sourceforge.io/docs/ref/rst/roles.html
    ":emphasis:" ":literal:" ":code:" ":math:" ":pep-reference:" ":PEP:" ":rfc-reference:" ":RFC:"
    ":strong:" ":subscript:" ":sub:" ":superscript:" ":sup:" ":title-reference:" ":title:" ":t:"
    ":raw:"))

[
  "interpreted_text"
  (literal)
] @text.literal

; Prefix role
((interpreted_text
  (role) @_role
  "interpreted_text" @emphasis)
  (#eq? @_role ":emphasis:"))

((interpreted_text
  (role) @_role
  "interpreted_text" @emphasis.strong)
  (#eq? @_role ":strong:"))

((interpreted_text
  (role) @_role
  "interpreted_text" @text.literal)
  (#eq? @_role ":math:"))

; Suffix role
((interpreted_text
  "interpreted_text" @emphasis
  (role) @_role)
  (#eq? @_role ":emphasis:"))

((interpreted_text
  "interpreted_text" @emphasis.strong
  (role) @_role)
  (#eq? @_role ":strong:"))

((interpreted_text
  "interpreted_text" @text.literal
  (role) @_role)
  (#eq? @_role ":math:"))

[
  (inline_target)
  (substitution_reference)
  (footnote_reference)
  (citation_reference)
  (reference)
] @link_text


; Directives
(directive
  name: (type) @function)

(directive
  body: (body
    (arguments) @variable.special))

((directive
  name: (type) @keyword)
  (#eq? @keyword "include"))

(directive
  name: (type) @function.builtin
  (#any-of? @function.builtin
    ; https://docutils.sourceforge.io/docs/ref/rst/directives.html
    "attention" "caution" "danger" "error" "hint" "important" "note" "tip" "warning" "admonition"
    "image" "figure" "topic" "sidebar" "line-block" "parsed-literal" "code" "math" "rubric"
    "epigraph" "highlights" "pull-quote" "compound" "container" "table" "csv-table" "list-table"
    "contents" "sectnum" "section-numbering" "header" "footer" "target-notes" "meta" "replace"
    "unicode" "date" "raw" "class" "role" "default-role" "title" "restructuredtext-test-directive"))

; Blocks
[
  (literal_block)
  (line_block)
] @text.literal

(block_quote
  (attribution)? @emphasis) @text.literal

(substitution_definition
  name: (substitution) @constant)

(footnote
  name: (label) @constant)

(citation
  name: (label) @constant)

(target
  name: (name)? @label
  link: (_)? @link_text)

(doctest_block) @text.literal

; Others
(comment) @comment

(comment
  "..") @comment

[
  ".."
  "|"
  "--"
  "__"
  ":"
  "::"
  (transition)
] @punctuation.special

[
  "bullet"
  "adornment"
] @punctuation.list_marker
