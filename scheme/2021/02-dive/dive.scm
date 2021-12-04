(add-to-load-path "../..")
(import [aoc-helpers input]
        [aoc-helpers lists]
        [aoc-helpers conversion]
        [srfi srfi-1])

(define input (map string-tokenize (file->lines "input")))

(define (solve-part1 input)
  (let next ([depth 0]
             [distance 0]
             [input input])
    (if (null? input)
      (* distance depth)
      (let ([instr (caar input)]
            [amount (string->number (cadar input))])
        (cond [(equal? "forward" instr)
               (next depth (+ distance amount) (cdr input))]
              [(equal? "down" instr)
               (next (+ depth amount) distance (cdr input))]
              [(equal? "up" instr)
               (next (- depth amount) distance (cdr input))])))))

(define (solve-part2 input)
  (let next ([depth 0]
             [distance 0]
             [aim 0]
             [input input])
    (if (null? input)
      (* distance depth)
      (let ([instr (caar input)]
            [amount (string->number (cadar input))])
        (cond [(equal? "forward" instr)
               (next (+ depth (* aim amount)) (+ distance amount) aim (cdr input))]
              [(equal? "down" instr)
               (next depth distance (+ aim amount) (cdr input))]
              [(equal? "up" instr)
               (next depth distance (- aim amount) (cdr input))])))))

(format #t "~a\n" (solve-part1 input))
(format #t "~a\n" (solve-part2 input))
