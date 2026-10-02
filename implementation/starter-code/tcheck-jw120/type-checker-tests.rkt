#lang racket

;; =============================================================================
;; Type Checker (Fall 2024): type-checker-tests.rkt
;; =============================================================================

(require (only-in "type-checker.rkt" type-check)
         "support.rkt"
         "test-support.rkt")

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define/provide-test-suite student-tests ;; DO NOT EDIT THIS LINE =========
  ; TODO: Add your own tests below!
  (test-equal? "Works with Num primitive"
               (type-check `2) (t-num))
  (test-raises-error? "Passing Str to + results in tc-err-bad-arg-to-op"
                                   (type-check `{+ "bad" 1})))

;; DO NOT EDIT BELOW THIS LINE =================================================

(module+ main (run-tests student-tests))
