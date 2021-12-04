(define-module (aoc-helpers conversion)
               #:export (bool->number))

(define (bool->number b) (if b 1 0))
