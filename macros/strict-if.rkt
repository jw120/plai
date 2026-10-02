#lang racket

;; (define (strict-if C T E)
;;   (if (boolean? C)
;;       (if C T E)
;;       (error 'strict-if "expected a boolean")))

;(strict-if true 2 3)
;(strict-if 1 2 3)
;(strict-if true 2 (/ 3 0))
;(if true 2 (/ 3 0))

(define-syntax strict-if
  (syntax-rules ()
    [(strict-if C T E)
     (if (boolean? C)
         (if C T E)
         (error 'strict-if "expected a boolean"))]))

(strict-if true 2 3)
;(strict-if 1 2 3)
(strict-if true 2 (/ 3 0))
