#lang racket

;; =============================================================================
;; Typed Interpreter: updated-tcheck-tests.rkt
;; =============================================================================

(require (only-in "updated-tcheck.rkt" type-check)
         "support.rkt"
         "test-support.rkt")

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define/provide-test-suite student-tests ;; DO NOT EDIT THIS LINE =========
  ; TODO: Add your own tests below!
  
  (test-equal? "Works with Num primitive"
               (type-check `2) (t-num)))

;; DO NOT EDIT BELOW THIS LINE =================================================

(module+ main (run-tests student-tests))
