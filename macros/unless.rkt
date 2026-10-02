#lang racket

(define-syntax unless1
  (syntax-rules ()
    [(unless1 cond body)
     (if (not cond)
         body
         (void))]))

(unless1 #t (println 11))
(unless1 #f (println 12))

(define-syntax unless
  (syntax-rules ()
    [(_ cond body ...)
     (if (not cond)
         (begin
           body
           ...)
         (void))]))
         
(unless #t (println 21))
(unless #f (println 22))
