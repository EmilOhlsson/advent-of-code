#!guile -s
!#

(add-to-load-path "../..")
(import (aoc-helpers input)    ;; file->lines function
	(rnrs lists))          ;; find function

;; Read input, and convert to numbers
(define input (map string->number (file->lines "input.txt")))
;(define input (map string->number (input->lines)))

;; Find a pair with sum of 2020 and return product of the pair
(define (find-pairs nums)
  (let*
    ((head (car nums))
     (tail (cdr nums))
     ;; Search for a value in tail that sums to 2020 with head
     (result (find (lambda (number) (= 2020 (+ number head))) tail)))
    (if result
      (* head result)
      (find-pairs tail))))

(format #t "~a\n" (find-pairs input))

(define (find-triplets nums)
  (let
    ((a-head (car nums))
     (a-tail (cdr nums)))
    (let recurse
      ((b-head (cadr nums))
       (b-tail (cddr nums)))
      (let
	((result (find (lambda (number) (= 2020 (+ number a-head b-head))) b-tail)))
	(cond 
	  (result (* a-head b-head result))
	  ((null? (cdr b-tail)) (find-triplets a-tail))
	  (else (recurse (car b-tail) (cdr b-tail))))))))

(format #t "~a\n" (find-triplets input))
