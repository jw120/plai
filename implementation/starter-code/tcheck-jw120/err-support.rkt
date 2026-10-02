#lang racket

;; =============================================================================
;; Type Checker: err-support.rkt
;; =============================================================================

(struct exn:fail:type exn:fail ())
(struct exn:fail:interp exn:fail ())

(define (raise-type-error msg)
  (raise (exn:fail:type msg (current-continuation-marks))))

(provide (struct-out exn:fail:type)
         raise-type-error)