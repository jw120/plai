#lang racket

;; =============================================================================
;; Desugar: desugar-tests.rkt
;; =============================================================================

(require (only-in "desugar.rkt" eval)
         "support.rkt"
         "desugar-support.rkt"
         "test-support.rkt")

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define/provide-test-suite student-tests ;; DO NOT EDIT THIS LINE ==========

 ;; Value expressions
  (test-equal? "Works with Num primitive"
               (eval `2) (v-num 2))
  (test-equal? "Works with Bool primitive"
               (eval `true) (v-bool #t))
  (test-equal? "Works with Str primitive"
               (eval `"abc") (v-str "abc"))

  ;; Operator expressions
  (test-equal? "Plus works"
               (eval `(+ 2 3)) (v-num 5))
  (test-raises-error? "Plus catches wrong left type"
                      (eval `(+ "ab" 3)))
  (test-raises-error? "Plus catches wrong right type"
                      (eval `(+ 3 false)))
  (test-equal? "Append works"
               (eval `(++ "abc" "de")) (v-str "abcde"))
  (test-raises-error? "Plus catches wrong left type"
                      (eval `(+ 3 "ab")))
  (test-raises-error? "Plus catches wrong right type"
                      (eval `(+ "ab" false)))
  (test-equal? "num= works on false"
               (eval `(num= 2 3)) (v-bool #f))
  (test-equal? "num= works on true"
               (eval `(num= 4 4 )) (v-bool #t))
  (test-raises-error? "num= catches wrong left type"
                      (eval `(num= 3 "ab")))
  (test-raises-error? "str= catches wrong right type"
                      (eval `(num= 3 false)))
  (test-equal? "str= works on false"
               (eval `(str= "abc" "abd")) (v-bool #f))
  (test-equal? "str= works on true"
               (eval `(str= "abc" "abc")) (v-bool #t))
  (test-raises-error? "str= catches wrong left type"
                      (eval `(str= 3 "ab")))
  (test-raises-error? "str= catches wrong right type"
                      (eval `(str= "abc" false)))

  ;; If expression
  (test-equal? "if works with true"
               (eval `(if (num= 4 (+ 2 2)) 42 19)) (v-num 42))
  (test-equal? "if works with false"
               (eval `(if (num= 4 (+ 2 3)) 42 19)) (v-num 19))
  (test-equal? "if true does not evaluate altern"
               (eval `(if (num= 4 (+ 2 2)) 42 (num= "a" "b"))) (v-num 42))
  (test-equal? "if false does not evaluate consq"
               (eval `(if (num= 4 (+ 2 3)) (++ 2 3) 19)) (v-num 19))

  ;; Lambdas
  (test-true "Works with lambda"
             (v-fun? (eval `{lam x 5})))
  (test-equal? "simple add"
               (eval `((lam x (+ x 1)) 41)) (v-num 42))
  (test-equal? "nested functions"
               (eval `(((lam x (lam y (+ x y))) 2) 3)) (v-num 5))
  (test-raises-error? "applying non-function"
                      (eval `(2 3)))
  (test-raises-error? "lambda with non-symbol"
                      (eval `(lam 2 (+ 2 3))))
  (test-raises-error? "unbound variable"
                      (eval `(+ x 2)))
  (test-equal? "lambda shadows"
               (eval `(((lam x (lam x x)) 2) 3)) (v-num 3))
  (test-equal? "example 1"
               (eval `((lam x (+ x 3)) 2)) (v-num 5))
  (test-equal? "example 2"
               (eval `((lam y 5) 1)) (v-num 5))

  ;; sugar-and
  (test-equal? "Sugar-and works with true true"
             (eval `(and true true)) (v-bool #t))
  (test-equal? "Sugar-and works with true false"
             (eval `(and true false)) (v-bool #f))
  (test-equal? "Sugar-and works with false true"
             (eval `(and false true)) (v-bool #f))
  (test-equal? "Sugar-and works with false false"
             (eval `(and false false)) (v-bool #f))
  (test-equal? "Sugar-and works with false fail"
             (eval `(and false (str= 1 0))) (v-bool #f))

  ;; sugar-or
  (test-equal? "Sugar-or works with true true"
             (eval `(or true true)) (v-bool #t))
  (test-equal? "Sugar-or works with true false"
             (eval `(or true false)) (v-bool #t))
  (test-equal? "Sugar-or works with false true"
             (eval `(or false true)) (v-bool #t))
  (test-equal? "Sugar-or works with false false"
             (eval `(or false false)) (v-bool #f))
  (test-equal? "Sugar-or works with true fail"
             (eval `(or true (str= 1 0))) (v-bool #t))

  ;; sugar-let
  (test-equal? "Sugar-let works simply"
               (eval `(let (x 1) (+ x 2))) (v-num 3))
  (test-equal? "Sugar-let works with two levels"
               (eval `(let (x 6) (let (y 7) (+ x y)))) (v-num 13))
  (test-equal? "Sugar-let works with shadowing"
               (eval `(let (x 6) (let (y 7) (let (x 5) (+ x y))))) (v-num 12))
  
  )


;; DO NOT EDIT BELOW THIS LINE =================================================

(module+ main (run-tests student-tests))
