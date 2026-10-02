#lang racket

;; =============================================================================
;; Lazy: my-lazy.rkt
;; =============================================================================

(provide #%module-begin #%datum #%top #%top-interaction define true false)

(struct computation-suspend (body))

(define true #t)
(define false #f)

;; DO NOT EDIT ABOVE THIS LINE =================================================

;; force =======================================================================

(provide force-and-cache)

(define (! cs)
  (match cs
    [(computation-suspend body)
     ; TODO: Evaluate the suspended computation to a concrete value
     ...]
    [else cs]))

(define (force-and-cache b)
  (! b))

;; function application ========================================================

(provide (rename-out [lazy-app #%app]))

(define-syntax lazy-app
  ; TODO: Implement me!
  ....)

;; lambdas =====================================================================

(provide (rename-out [lazy-lam lam]))

(define-syntax lazy-lam
  ; TODO: Implement me!
  ....)

;; let =========================================================================

(provide (rename-out [lazy-let let]))

(define-syntax lazy-let
  ; TODO: Implement me!
  ....)

;; conditionals ================================================================

(provide (rename-out [lazy-if if]))

(define-syntax lazy-if
  ; TODO: Implement me!
  ....)

;; list constructors ===========================================================

(provide (rename-out [lazy-pair pair]))

(define-syntax lazy-pair
  ; TODO: Implement me!
  ....)

;; list operators ==============================================================

(provide (rename-out [lazy-first first]
                     [lazy-second second]
                     [lazy-is-pair is-pair]))

(define-syntax lazy-first
  ; TODO: Implement me!
  ....)

(define-syntax lazy-second
  ; TODO: Implement me!
  ....)

(define-syntax lazy-is-pair
  ; TODO: Implement me!
  ....)

;; binary operators ============================================================

(provide (rename-out [lazy-plus +]
                     [lazy-append ++]
                     [lazy-num-eq num=]
                     [lazy-str-eq str=]))

(define-syntax lazy-plus
  ; TODO: Implement me!
  ....)

(define-syntax lazy-append
  ; TODO: Implement me!
  ....)

(define-syntax lazy-num-eq
  ; TODO: Implement me!
  ....)

(define-syntax lazy-str-eq
  ; TODO: Implement me!
  ....)
