#lang racket

;; =============================================================================
;; Macros (Fall 2024): python-if.rkt
;; =============================================================================

(require test-engine/racket-tests)
(provide python-if)

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define-syntax python-if
  ; TODO: Implement me!
  ....)

; Provided sample tests. These basic tests should work on your implementation.
; There are many more cases that we're not checking for here, so make sure to
; check more than just this test before submitting!
(check-expect (python-if true "foo" "bar") "foo")
(check-expect (python-if false "foo" "bar") "bar")
(test)
