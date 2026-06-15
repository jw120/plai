#lang plait

;; =============================================================================
;; Desugar: desugar.rkt
;; =============================================================================

(require "support.rkt" "desugar-support.rkt" "interpreter.rkt")

(define (eval [str : S-Exp]): Value
  (interp (desugar (parse+ str))))

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define (desugar [expr : Expr+]): Expr
  (type-case Expr+ expr
    [(e-num+ value)
     (e-num value)]
    [(e-str+ value)
     (e-str value)]
    [(e-bool+ value)
     (e-bool value)]
    [(e-op+ op left right)
     (e-op op (desugar left) (desugar right))]
    [(e-if+ condition consq altern)
     (e-if (desugar condition) (desugar consq) (desugar altern))]
    [(e-lam+ param body)
     (e-lam param (desugar body))]
    [(e-app+ func arg)
     (e-app (desugar func) (desugar arg))]
    [(e-var+ name)
     (e-var name)]
    [(sugar-and left right)
     (e-if (desugar left) (desugar right) (e-bool #false))]
    [(sugar-or left right)
     (e-if (desugar left) (e-bool #true) (desugar right))]
    [(sugar-let var value body)
     (e-app (e-lam var (desugar body)) (desugar value))]))
     
