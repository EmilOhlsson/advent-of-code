(import (srfi srfi-1))

(define-module (aoc-helpers lists)
               #:export (cartesian-product try-take windows))

;; Return a cartesian product of given `seq`s
(define (cartesian-product . seq)
  (fold (λ (xs ys)
           (apply append
                  (map (λ (x)
                          (map (λ (y) (cons x y))
                               ys))
                       xs)))
        '(())
        seq))

;; Attempt to take `n` elenments from `lst`
(define (try-take n lst)
  (let loop ([n n] [lst lst] [try-build identity])
    (cond [(= n 0) (try-build '())]
          [(null? lst) '()]
          [else (loop (- n 1)
                      (cdr lst)
                      (λ (rest) (try-build (cons (car lst) rest))))])))

;; Build a series of `n` sized windows of the list `lst`
(define (windows n lst)
 (let loop ([lst lst] [window (try-take n lst)] [try-build identity])
   (cond [(null? window) (try-build '())]
         [(null? lst) (try-build '())]
         [else (loop (cdr lst)
                     (try-take n (cdr lst))
                     (λ (rest) (try-build (cons window rest))))])))
