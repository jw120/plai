#lang plait

;; =============================================================================
;; Desugar: desugar.rkt
;; =============================================================================

(require "support.rkt" "desugar-support.rkt" "interpreter.rkt")

(define (eval [str : S-Exp]): Value
  (interp (desugar (parse+ str))))

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define (desugar [expr : Expr+]): Expr
  ; TODO: Implement me!
  ....)
