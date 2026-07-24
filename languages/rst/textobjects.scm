; Sections as classes (navigable with [[, ]], [], ][)
(section) @class.around

; Directives as functions (navigable with [m, ]m, [M, ]M)
(directive) @function.around

(directive
  body: (body) @function.inside)

; Comments
(comment) @comment.around
