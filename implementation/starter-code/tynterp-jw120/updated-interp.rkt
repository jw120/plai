#lang plait

;; =============================================================================
;; Typed Interpreter: updated-interp.rkt
;; =============================================================================

(require "support.rkt"
         (rename-in (typed-in "err-support.rkt"
                              [raise-type-error : (String -> 'a)]
                              [raise-interp-error : (String -> 'b)])))

(define (eval [str : S-Exp]): Value
  (interp (desugar (parse str))))

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define (desugar [expr : Expr+]): Expr
  (type-case Expr+ expr
    [(e-num+ n) (e-num n)]
    [(e-str+ s) (e-str s)]
    [(e-bool+ b) (e-bool b)]
    [(e-empty+ t) (e-empty t)]
    [(e-op+ o l r) (e-op o (desugar l) (desugar r))]
    [(e-un-op+ o a) (e-un-op o (desugar a))]
    [(e-if+ cd cs a) (e-if (desugar cd) (desugar cs) (desugar a))]
    [(e-lam+ p t b) (e-lam p t (desugar b))]
    [(e-app+ f a) (e-app (desugar f) (desugar a))]
    [(e-var+ n) (e-var n)]
    [(sugar-and l r)
     (e-if (desugar l)
           (e-if (desugar r)
                 (e-bool #t)
                 (e-bool #f))
           (e-bool #f))]
    [(sugar-or l r)
     (e-if (desugar l)
           (e-bool #t)
           (e-if (desugar r)
                 (e-bool #t)
                 (e-bool #f)))]
    [(sugar-let id val t body)
     (e-app (e-lam id t
                   (desugar body))
            (desugar val))]))

(define (interp [expr : Expr]): Value
  ; TODO: Implement me!
  ....)
