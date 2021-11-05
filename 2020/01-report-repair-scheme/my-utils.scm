(define-module (my-utils)
	       #:export (file->lines input->lines))

(import (ice-9 rdelim))

;; Return all lines read from port as a list
(define (read-lines port)
  (let loop ((lines '()))
    (let ((line (read-line port)))
      (if (eof-object? line)
	(reverse! lines)
	(loop (cons line lines))))))

;; Open file, read lines and return all read lines as list
(define (file->lines file)
  (let* ((port (open-input-file file))
	 (lines (read-lines port)))
    (close-port port)
    lines))

;; Read all lines from stdin and return as list
(define (input->lines)
  (read-lines (current-input-port)))
