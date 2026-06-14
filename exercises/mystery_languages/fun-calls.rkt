#lang mystery-languages/fun-calls

(deffun (add x y) (+ x y))
(deffun (inc z) (++ z "!"))
(deffun (second x y) y)

;; L2 seems to return the first deffun result called in the evaluation (irrespetive of type)
(TEST (second (inc "") 4) 4 "!" 4)
(TEST (add (inc "") 2) failure "!" failure)

;; L3 seems to return first result when used recursively
(TEST (inc (inc "")) "!!" "!" "!")
(TEST (add (add 1 2) (add 3 4)) 10 3 3)
