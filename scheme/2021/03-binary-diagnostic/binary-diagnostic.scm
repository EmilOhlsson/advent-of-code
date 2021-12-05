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
  (define (make-initial-ratio row)
    "Create an initial, empty list of zeroed ratios"
    (if (null? row) '()
      (cons (list 0 0)
            (make-initial-ratio (cdr row)))))
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
  (let loop ([ratios (make-initial-ratio (car input))]
             [input input])
    (if (null? input)
      (ratio->answer ratios)
      (let ([row (car input)])
        (loop (process-row row ratios)
              (cdr input))))))

(format #t "~a\n" (solve-part1 input))

"#### Part 2 ####"

(define (solve-part2 input)

  (define (ratio-for-column input column)
    "Count the ratio of zeros and ones in a column of input"
    (let next ([input input]
               [zeros 0]
               [ones 0])
      (if (null? input)
        (list zeros ones)
        (let* ([row (car input)]
               [bit (list-ref row column)])
          (if (equal? bit #\1)
            (next (cdr input) zeros (+ 1 ones))
            (next (cdr input) (+ 1 zeros) ones))))))

  (define (bits->number bits)
    "Convert a list of 0\1 chars to a number"
    (string->number (list->string bits) 2))

  (define (get-rating bit-filter input)
    "Continously filter input using `bit-filter`, until only one left"
    "Then return result as an interger"
    (let next ([input input]
               [column 0])
      (if (null? (cdr input))
        (bits->number (car input))
        (let ([ratio (ratio-for-column input column)])
          (next
            (filter (bit-filter ratio column) input)
            (+ 1 column))))))

  (let ([oxygen
          (get-rating
            (λ (ratio column)
               (let* ([zeros (car ratio)]
                      [ones (cadr ratio)]
                      [bit (if (> zeros ones) #\0 #\1)])
                 (λ (line)
                    (equal? (list-ref line column) bit))))
            input)]
        [co2
          (get-rating
            (λ (ratio column)
               (let* ([zeros (car ratio)]
                      [ones (cadr ratio)]
                      [bit (if (<= zeros ones) #\0 #\1)])
                 (λ (line)
                    (equal? (list-ref line column) bit))))
            input)])
    (* co2 oxygen)))

(format #t "~a\n" (solve-part2 input))
