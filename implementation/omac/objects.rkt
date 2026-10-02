#lang racket

;; =============================================================================
;; OMac (Fall 2024): objects.rkt
;; =============================================================================

(provide object call)

(require "err-support.rkt")

;; DO NOT EDIT ABOVE THIS LINE =================================================

(struct object-struct
        (fields methods))

(define-syntax object
  (syntax-rules (fields methods)
    [(_ (fields [fn iv] ...) (methods [mv imp] ...))
     (lambda (mn)
       (let ([fn iv] ...)
         (case mn
           [(mv) imp] ...
           [else (raise-method-not-found-exception "fail")])))]))

(define-syntax call
  (syntax-rules ()
    [(_ o mn arg ...)
     ((o 'mn) 'mn arg ...)]))
     
(define o1 (object
 (fields [x 2])
 (methods [get-x (lambda (self) x)]
          [get-x2 (lambda (self) (call self get-x))]
          [inc-x (lambda (self) (set! x (+ 1 x)))]
          [get-x+y (lambda (self y) (+ x y))])))

((o1 'get-x) o1)
((o1 'get-x+y) o1 3)

(call o1 get-x)
(call o1 get-x2)
(call o1 get-x+y 3) 
(call o1 inc-x)
(call o1 get-x)

(define cowboy-object
  (object
   (fields [name "Timmy the Cowboy"])
   (methods
    [say-howdy-to
     (lambda (self to)
       (string-append name " says: Howdy " to "!"))]
    [get-name
     (lambda (self) name)]
    [set-name
     (lambda (self x) (set! name x))])))

(call cowboy-object get-name)
(call cowboy-object set-name "Fred")
(call cowboy-object get-name)
(call cowboy-object say-howdy-to "Pardner")
