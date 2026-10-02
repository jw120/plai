#lang racket

;; =============================================================================
;; OMAC (Fall 2024): err-support.rkt
;; =============================================================================

(struct exn:fail:method-not-found exn:fail ())

(define (raise-method-not-found-exception msg)
  (raise (exn:fail:method-not-found msg (current-continuation-marks))))

(provide (struct-out exn:fail:method-not-found)
         raise-method-not-found-exception)
