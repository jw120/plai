#lang racket

;; =============================================================================
;; Typed Interpreter: updated-interp-tests.rkt
;; =============================================================================

(require (only-in "updated-interp.rkt" eval)
         "support.rkt"
         "test-support.rkt")

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define/provide-test-suite student-tests ;; DO NOT EDIT THIS LINE ==========
  ; TODO: Add your own tests below!
  
  (test-equal? "Works with Num primitive"
               (eval `2) (v-num 2)))

;; DO NOT EDIT BELOW THIS LINE =================================================

(module+ main (run-tests student-tests))
