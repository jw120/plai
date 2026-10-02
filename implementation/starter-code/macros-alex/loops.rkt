#lang racket

;; =============================================================================
;; Macros (Fall 2024): loops.rkt
;; =============================================================================

(require test-engine/racket-tests)
(provide for-right for-wrong)

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define-syntax for-right
  ; TODO: Implement me!
  ....)

(define-syntax for-wrong
  ; TODO: Implement me!
  ....)

; Provided sample tests. These basic tests should work on your implementation.
; There are many more cases that we're not checking for here, so make sure to
; check more than just this test before submitting!
(check-expect (for-right x (list 1 2 4) (* x x)) '(1 4 16))
(check-expect (for-wrong x (list 1 2 4) (* x x)) '(1 4 16))
(test)
