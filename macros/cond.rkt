#lang racket

(define-syntax my-cond
  (syntax-rules ()
    [(my-cond) (error 'my-cond "Should not get here")]
    [(my-cond [q0 a0] [q1 a1] ...)
     (if q0
         a0
         (my-cond [q1 a1] ...))]))

(define (my-sgn x)
  (my-cond
   [(< x 0) -1]
   [(= x 0) 0]
   [#t 1]))

(map my-sgn '(2 -3 4 0))
