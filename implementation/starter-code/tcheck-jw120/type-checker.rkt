#lang plait

;; =============================================================================
;; Type Checker (Fall 2024): type-checker.rkt
;; =============================================================================

(require "support.rkt"
         (rename-in (typed-in "err-support.rkt"
                              [raise-type-error : (String -> 'a)])))

(define (type-check [str : S-Exp]): Type
  (type-of (parse str)))

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define (type-of [expr : Expr]): Type
  ; TODO: Implement me!
  ....)
