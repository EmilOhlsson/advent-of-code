(define-module (aoc-helpers utils)
	       #:export (clamp))

(define (clamp val lo hi)
  "Clamp value to range between low and high"
  (min hi (max val lo)))
