(define-module (aoc-helpers lists)
	       #:export (accumulate count))

;; Accumulate
(define (accumulate op init seq)
  (if (null? seq) 
    init
    (op (car seq)
	(accumulate op init (cdr seq)))))

;; Count valid symbols in list
(define (count pred? sequence)
  (if (null? sequence)
    0
    (+ (if (pred? (car sequence)) 1 0) 
       (count pred? (cdr sequence)))))
