#lang racket

(define-syntax or-2
  (syntax-rules ()
    [(_ e1 e2)
     (if e1
         e1
         e2)]))

(or-2 (member 'y '(x y z)) "not found")
(or-2 (member 'q '(x y z)) "not found")

(or-2 (print "hello") "not found")

(define-syntax or-2b
  (syntax-rules ()
    [(_ e1 e2)
     (let ([v e1])
       (if v
           v
           e2))]))

(or-2b (member 'y '(x y z)) "not found")
(or-2b (member 'q '(x y z)) "not found")
(or-2b (print "hello") "not found")

(define-syntax orN
  (syntax-rules ()
    [(_) false]
    ([_ e1 e2 ...]
     (let ([v e1])
       (if v
           v
           (orN e2 ...))))))

(orN)
(orN #t)
(orN (= 2 3) (> 2 3) 23)

(define-syntax orNb
  (syntax-rules ()
    [(_) false]
    ([_ e ...]
     (let ([v e])
       (if v
           v
           (orNb ...))))))