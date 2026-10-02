#lang plait

;; =============================================================================
;; Typed Interpreter: tynterp.rkt
;; =============================================================================

(require "support.rkt"
         (rename-in (typed-in "err-support.rkt"
                              [raise-type-error : (String -> 'a)]
                              [raise-interp-error : (String -> 'b)])))

(define (eval [str : S-Exp]): Value
  (let* ([expr (desugar (parse str))]
         [t (type-of expr)])
    (interp expr)))

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define (desugar [expr : Expr]): Expr
  ; TODO: Implement me!
  ....)

(define (type-of [expr : Expr]) : Type
  ; TODO: Implement me!
  ....)

(define (interp [expr : Expr]): Value
  ; TODO: Implement me!
  ....)
