#lang racket

;; =============================================================================
;; Assertions to Security (Fall 2022): security.rkt
;; =============================================================================

(require test-engine/racket-tests)
(provide assert d/dx limit-calls make-revocable
         revocable revocable-f revocable-revoker)

(define (raise-assertion-error v type)
  (error 'assertion-failure "value ~a isn't a ~a" v type))

(define (raise-limit-error limit)
  (error 'limit-exceeded "made more than ~a calls" limit))

(struct revocable (f revoker) #:transparent)

(define (raise-revocation-error)
  (error 'access-revoked "function access has been revoked"))

;; DO NOT EDIT ABOVE THIS LINE =================================================

;; Parts 1 and 2 ===============================================================

(define (assert ct)
  ; TODO: Implement me!
  ...)

;; Part 3 ======================================================================

(define (raw-d/dx f)
  ; TODO: Implement me!
  ...)

; TODO: Define me!
(define d/dx ...)

;; Part 4 ======================================================================

(define (limit-calls f limit)
  ; TODO: Implement me!
  ...)

;; Part 5 ======================================================================

(define (make-revocable f)
  ; TODO: Implement me!
  ...)
