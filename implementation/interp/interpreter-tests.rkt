#lang racket

;; =============================================================================
;; Interpreter: interpreter-tests.rkt
;; =============================================================================

(require (only-in "interpreter.rkt" eval)
         "support.rkt"
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
  
  )


;;  (test-true "Works with lambda"
;;             (v-fun? (eval `{lam x 5})))
;;  (test-pred "Equivalent to the test case above, but with test-pred"
;;             v-fun? (eval `{lam x 5}))
;;  (test-raises-error? "Passing Str to + results in error"
;;                             (eval `{+ "bad" 1})))

;; DO NOT EDIT BELOW THIS LINE =================================================

(module+ main (run-tests student-tests))
