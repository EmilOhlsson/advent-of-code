(add-to-load-path "../..")
(import [aoc-helpers input]
        [aoc-helpers lists]
        [aoc-helpers conversion]
        [srfi srfi-1])

(define input (map string->number (file->lines "input")))

(define input-simple (map string->number (file->lines "input-simple")))

(define (solve-part1 input)
  (apply +
         (map bool->number
              (map (λ (ab) (apply < ab))
                   (windows 2 input)))))

(define (solve-part2 input)
  (let [(filtered (map (λ (l) (apply + l))
                       (windows 3 input)))]
    (apply +
           (map bool->number
                (map (λ (ab) (apply < ab))
                     (windows 2 filtered))))))

(format #t "~a\n" (solve-part1 input))
(format #t "~a\n" (solve-part2 input))
