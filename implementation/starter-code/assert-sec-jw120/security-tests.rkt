#lang racket

;; =============================================================================
;; Assertions to Security (Fall 2022): security-tests.rkt
;; =============================================================================

(require (only-in "security.rkt" assert d/dx limit-calls make-revocable
                  revocable revocable-f revocable-revoker)
         "test-support.rkt")

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define/provide-test-suite student-tests ;; DO NOT EDIT THIS LINE ===========
  ; TODO: Add your own tests below!
  (test-equal? "part 1: valid num assertion" ((assert 'num) 3) 3))

;; DO NOT EDIT BELOW THIS LINE =================================================

(module+ main (run-tests student-tests))
