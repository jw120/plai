#lang mystery-languages/conditionals

;; L1 requires a boolean for if
(TEST (if 1 2 3) failure 2 2)

;; L3 and/or returns a boolean
(TEST (or 3 2) failure 3 #t)
