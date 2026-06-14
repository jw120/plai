#lang plait

;; =============================================================================
;; Interpreter: interpreter.rkt
;; =============================================================================

(require "support.rkt")

(define (eval [str : S-Exp]): Value
  (interp (parse str)))

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define (interp [expr : Expr]) : Value
  (interp_ expr (hash '())))
           
(define (interp_ [expr : Expr] [nv : Env]): Value
  (type-case Expr expr
    [(e-num value) (v-num value)]
    [(e-str value) (v-str value)]
    [(e-bool value) (v-bool value)]
    [(e-op op left right)
     (let ([l (interp_ left nv)]
           [r (interp_ right nv)])
       (type-case Operator op
         [(op-plus) (v-num (+ (as-num l) (as-num r)))]
         [(op-append) (v-str (string-append (as-str l) (as-str r)))]
         [(op-str-eq) (v-bool (string=? (as-str l) (as-str r)))]
         [(op-num-eq) (v-bool (= (as-num l) (as-num r)))]))]
    [(e-if condition consq altern)
     (if (as-bool (interp_ condition nv))
         (interp_ consq nv)
         (interp_ altern nv))]
    [(e-var name) (lookup name nv)]
    [(e-lam param body) (v-fun param body nv)]
    [(e-app func arg)
     (type-case Value (interp_ func nv)
       [(v-fun param body func_nv)
        (let* ([arg-value (interp_ arg nv)]
               [body_nv (hash-set func_nv param arg-value)])
          (interp_ body body_nv))]
       [else (error 'interp_ "cannot apply a non-function")])]))

;; Lookup symbol in environment, error if not found
(define (lookup [s : Symbol] [nv : Env]) : Value
  (type-case (Optionof Value) (hash-ref nv s)
             [(none) (error s "not bound")]
             [(some v) v]))

;; Return the value as a number (or throw an error)
(define (as-num [value : Value]) : Number
  (type-case Value value
             [(v-num value) value]
             [else (error 'as-num "Not a number")]))

;; Return the value as a string (or throw an error)
(define (as-str [value : Value]) : String
  (type-case Value value
             [(v-str value) value]
             [else (error 'as-str "Not a string")]))

;; Return the value as a boolean (or throw an error)
(define (as-bool [value : Value]) : Boolean
  (type-case Value value
             [(v-bool value) value]
             [else (error 'as-bool "Not a boolean")]))


