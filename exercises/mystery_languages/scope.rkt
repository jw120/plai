#lang mystery-languages/scope

;;(defvar z 3)

;;(let ([z 7]) ((lambda (x) (+ x z)) 0))

(defvar f (lambda (x) (+ x 1)))
(defvar twice (lambda (g) (lambda (z) (g (g z)))))

;(lambda f (lamba (x) (f (f x))))
;((lambda g (lambda (x) (g (g x)))) (lambda (z) (+ 1 z)))

(twice f)
((twice f) 0)

(((lambda (f) (lambda (z) (f (f z)))) (lambda (x) (+ x 1))) 0)