#lang racket

(define-syntax my-let1
  (syntax-rules ()
    [(my-let1 (var val) body)
     ((lambda (var) body) val)]))

(my-let1 (x 2) (+ x x))

(define-syntax my-let2
  (syntax-rules ()
    [(my-let2 ([var val] ...) body)
     ((lambda (var ...) body) val ...)]))

(my-let2 ([x 2] [y 3] [z 4]) (* x y z))
