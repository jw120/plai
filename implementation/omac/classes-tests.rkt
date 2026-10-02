#lang racket

;; =============================================================================
;; OMac (Fall 2024): classes-tests.rkt
;; =============================================================================

(require (only-in "classes.rkt" class call new)
         "test-support.rkt")

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define/provide-test-suite student-tests ;; DO NOT EDIT THIS LINE ==============
  ; TODO: Add your own tests below!
  (test-equal? "Basic test"
               (begin
                 (class MyTestClass
                   (fields [my-field 1])
                   (methods [get-my-field (lambda (self) my-field)]))
                 (call (new MyTestClass) get-my-field)) 1))

;; DO NOT EDIT BELOW THIS LINE =================================================

(module+ main (run-tests student-tests))
