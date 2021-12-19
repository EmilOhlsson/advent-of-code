(add-to-load-path "../..")
(import 
  [srfi srfi-1]
  [srfi srfi-13]
  [ice-9 match]
  [ice-9 q]
  [aoc-helpers input]
  [aoc-helpers lists]
  [aoc-helpers conversion])

(define input (file->lines "input"))
(define input-simple (file->lines "input-simple"))

(define (build-boards input)
  "Take a list of lines, and build a set of boards to play on."
  "The boards will be represented as (board, row, col)->value,"
  "as well as value->(board, row, col) along with number of boards"
  (let ([position->value (make-hash-table)]
        [value->positions (make-hash-table)])
    ;; Loop over lines. An empty line indicates a new board. When
    ;; all lines have been processed, return a pair of hash-tables
    ;; that map board positions and numbers
    (let next-line ([input input]
                    [board-number 0]
                    [row 0])
      (cond
        [(null? input) ;; Done, return result
         (list (+ 1 board-number) position->value value->positions)]
        [(equal? (car input) "") ;; New board, increase board number
         (next-line (cdr input)
                    (+ 1 board-number)
                    0)]
        [else
          ;; Split line into tokens, and insert tokens in tables
          (let ([tokens (string-split (car input) char-whitespace?)])
            (let next-column ([col 0]
                              [tokens tokens])
              (if (null? tokens)
                (next-line (cdr input) board-number (+ 1 row))
                ;; Skip empty tokens
                (if (equal? (car tokens) "")
                  (next-column col (cdr tokens))
                  (begin
                    (hash-set!
                      position->value
                      (list board-number row col)
                      (string->number (car tokens)))
                    ;; Append postion to list of positions
                    (let ([positions (hash-ref value->positions (string->number (car tokens)) '())])
                      (hash-set! value->positions
                                 (string->number (car tokens))
                                 (append positions (list (list board-number row col)))))
                    (next-column (+ 1 col) (cdr tokens)))))))]))))

(define (solve-part-1 input)
  (match-let
    ([called-numbers (map string->number (string-split (car input) #\,))]
     [(number-of-tables position->value value->positions) (build-boards (cddr input))]
     [marked-positions (make-hash-table)]
     [completed-tables (make-hash-table)]
     [scores (make-q)])

    ;; Check if all boards have completed
    (define (game-done?)
      (equal? (q-length scores) number-of-tables))

    (define (calculate-score board-number)
      (apply +
             (map
               (λ (pos)
                  (let ([value (hash-ref position->value pos)])
                    value))
               (filter
                 (λ (pos)
                    (not (hash-ref marked-positions pos)))
                 (cartesian-product (iota 5) (iota 5) (list board-number)  )))))

    ;; Mark a position, and check if it resulted in a win board win
    (define (mark-position pos)
      (let ([board (car pos)]
            [row (cadr pos)]
            [col (caddr pos)])
        (hash-set! marked-positions pos #t)
        (or (every 
              (λ (r) (hash-ref marked-positions (list board r col)))
              (iota 5))
            (every 
              (λ (c) (hash-ref marked-positions (list board row c)))
              (iota 5)))))

    (define (mark-and-add-score pos num)
      (let ([board-number (car pos)])
        (when (mark-position pos)
          ;; Only enqueue score if score hasn't already been added
          (unless (hash-ref completed-tables board-number)
            (hash-set! completed-tables board-number #t)
            (enq! scores (* num (calculate-score board-number)))))))

    ;; Loop over numbers, and check for victories
    (let call-next-number ([nums called-numbers])
      (when (null? nums)
        (error "No more numbers to call!"))
      (let ([called (car nums)])
        (if  
          ;; Check if all tables have completed, if so calculate
          ;; score of first and last
          (game-done?)
          (list (q-front scores) (q-rear scores))
          (let next-position
            ;; Work through the list of positions, mark them
            ;; and if the board is completed, mark board as
            ;; done, and enqueue it to ranking list
            ([positions (hash-ref value->positions called)])
            (if (null? positions)
              (call-next-number (cdr nums))
              (begin
                (mark-and-add-score (car positions) called)
                (next-position (cdr positions))))))))))

;;(solve-part-1 input-simple)
(solve-part-1 input)
