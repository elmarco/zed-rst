; Section titles
(section
  (title) @name) @item

; Top-level directives (not inside substitution_definition)
(document
  (directive
    name: (type) @name
    body: (body
      (arguments) @context)?) @item)

; Footnotes
(footnote
  name: (label) @name) @item

; Citations
(citation
  name: (label) @name) @item

; Substitution definitions
(substitution_definition
  name: (substitution) @name) @item
