#lang mystery-languages/strings

;; L3 adds a space when concatenating
(TEST (++ "a" "b") "ab" "ab" "a b")
;; L2 is case-insensitive
(TEST (string=? "A" "a") #f #t #f)