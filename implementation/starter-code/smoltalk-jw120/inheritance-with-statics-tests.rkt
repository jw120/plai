#lang racket

;; =============================================================================
;; SMoLTalk (Fall 2024): inheritance-with-statics-tests.rkt
;; =============================================================================

(require (only-in "inheritance-with-statics.rkt" Root class call new)
         "test-support.rkt")

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define/provide-test-suite student-tests ;; DO NOT EDIT THIS LINE =============
  ; TODO: Add your own tests below!
  (test-equal? "Basic test"
               (begin
                 (class MyTestClass extends Root
                   (statics [my-static-field 1])
                   (fields [my-field 1])
                   (methods [get-my-field (lambda (self) my-field)]))
                 (call (new MyTestClass) get-my-field)) 1))

;; DO NOT EDIT BELOW THIS LINE =================================================

(module+ main (run-tests student-tests))
