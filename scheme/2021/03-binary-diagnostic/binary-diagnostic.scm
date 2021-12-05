(add-to-load-path "../..")
(import [aoc-helpers input]
        [aoc-helpers lists]
        [aoc-helpers conversion]
        [srfi srfi-1])

(define input (map string->list (file->lines "input")))

(define input-simple (map string->list (file->lines "input-simple")))

(define (solve-part1 input)
  (define (process-row row ratios)
    "Process a row, and return an updated list of ratios"
    (if (null? row) '()
      (let ([zeros (caar ratios)]
            [ones (cadar ratios)])
        (if (equal? (car row) #\1)
          (cons (list zeros (+ 1 ones))
                (process-row (cdr row) (cdr ratios)))
          (cons (list (+ 1 zeros) ones)
                (process-row (cdr row) (cdr ratios)))))))
  (define (make-ratio row)
    "Create an initial, empty list of zeroed ratios"
    (if (null? row) '()
      (cons (list 0 0)
            (make-ratio (cdr row)))))
  (define (ratio->answer ratios)
    "Process given ratios, and return the product of epsilon and gamma"
    (let loop ([gamma 0]
               [epsilon 0]
               [ratios ratios])
      (if (null? ratios)  ;; Done
        (* gamma epsilon)
        (let* ([zeros (caar ratios)]
               [ones (cadar ratios)]
               [gamma-bit (if (< zeros ones) 0 1)]
               [epsilon-bit (if (> zeros ones) 0 1)])
          (loop (logior (ash gamma 1) gamma-bit)
                (logior (ash epsilon 1) epsilon-bit)
                (cdr ratios))))))
  (let loop ([ratios (make-ratio (car input))]
             [input input])
    (if (null? input)
      (ratio->answer ratios)
      (let ([row (car input)])
        (loop (process-row row ratios)
              (cdr input))))))

(format #t "~a\n" (solve-part1 input))
