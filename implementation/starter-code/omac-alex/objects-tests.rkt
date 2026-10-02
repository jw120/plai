#lang racket

;; =============================================================================
;; OMac (Fall 2024): objects-tests.rkt
;; =============================================================================

(require (only-in "objects.rkt" object call)
         "test-support.rkt")

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define/provide-test-suite student-tests ;; DO NOT EDIT THIS LINE ==============
  ; TODO: Add your own tests below!
  (test-equal? "Basic test"
               (let ([my-test-object
                      (object
                       (fields [my-field 1])
                       (methods [get-my-field (lambda (self) my-field)]))])
                 (call my-test-object get-my-field)) 1))

;; DO NOT EDIT BELOW THIS LINE =================================================

(module+ main (run-tests student-tests))
