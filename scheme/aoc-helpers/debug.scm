(define-module (aoc-helpers debug)
	       #:export (dbg print-table))

(define (dbg value . annotation)
  "Print a value and return it, with an optional annotation"
  (if (null? annotation)
    (format #t "~a\n" value)
    (format #t "~a: ~a\n" (car annotation) value))
  value)

(define (print-table table)
  "Print a hash table"
  (format #t "table:\n")
  (hash-for-each
    (λ (p . v) (format #t " ~a: ~a\n" p v))
    table))
