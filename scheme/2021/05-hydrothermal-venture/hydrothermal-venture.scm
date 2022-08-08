(add-to-load-path "../..")
(import
  [srfi srfi-1]	 ;; List operations
  [srfi srfi-13] ;; String operations
  [srfi srfi-41] ;; Streams
  [ice-9 regex]
  [aoc-helpers input]
  [aoc-helpers debug]
  [aoc-helpers utils])

(define input (file->lines "input"))
(define input-simple (file->lines "input-simple"))

(define line-regexp (make-regexp "([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)"))
(define (match str) (regexp-exec line-regexp str))

;; Helpers for fetching specific parts of point pairs
(define (pp->x0 pp) (caar pp))
(define (pp->x1 pp) (cadr pp))
(define (pp->y0 pp) (cdar pp))
(define (pp->y1 pp) (cddr pp))

;; Add two points together
(define (addp p s)
  (cons (+ (car p) (car s))
        (+ (cdr p) (cdr s))))

(define (line->points line)
  "Parse a line into pair of coordinates"
  (let* ((m (match line))
         (x0 (string->number (match:substring m 1)))
         (y0 (string->number (match:substring m 2)))
         (x1 (string->number (match:substring m 3)))
         (y1 (string->number (match:substring m 4))))
    (cons (cons x0 y0) (cons x1 y1))))

(define (straight? pp)
  "Tests if a point-pair is straight"
  (or
    (= (pp->x0 pp) (pp->x1 pp))
    (= (pp->y0 pp) (pp->y1 pp))))

(define (pp->pl pp)
  "Converts a point pair to point list"
  (let next-point 
    ([point  (car pp)]
     [step (cons (clamp (- (pp->x1 pp) (pp->x0 pp)) -1 1)
                 (clamp (- (pp->y1 pp) (pp->y0 pp)) -1 1))]
     [steps (+ 1 (max (abs (- (pp->x1 pp) (pp->x0 pp)))
                      (abs (- (pp->y1 pp) (pp->y0 pp)))))])
    (if (= steps 0)
      '()
      (cons point
            (next-point (addp point step)
                        step
                        (- steps 1))))))

(define (increment-count h p)
  (hash-set! h p (+ (hash-ref h p 0) 1 )))

(define (add-points-to-table table points)
  "Increment the count of all the given points"
  (for-each
    (λ (p) 
       (increment-count table p))
    points))

(define (solve pred lines)
  (let ([pp-lines (filter pred (map line->points lines))]
        [points (make-hash-table)])

    ;; Build a count table of all set points
    (for-each 
      (λ (pp-line) 
         (add-points-to-table points (pp->pl pp-line)))
      pp-lines)

    ;(print-table points)
    (hash-count 
      (λ (point count)
         (> count 1))
      points)))


(define (solve-p1 lines) (solve straight? lines))
(define (solve-p2 lines) (solve (const #t) lines))

(format #t "Part 1: ~a\n" (solve-p1 input))
(format #t "Part 2: ~a\n" (solve-p2 input))
