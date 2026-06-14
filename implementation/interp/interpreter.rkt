#lang plait

;; =============================================================================
;; Interpreter: interpreter.rkt
;; =============================================================================

(require "support.rkt")

(define (eval [str : S-Exp]): Value
  (interp (parse str)))

;; DO NOT EDIT ABOVE THIS LINE =================================================

(define (interp [expr : Expr]): Value
  (type-case Expr expr
             [(e-num value) (v-num value)]
             [(e-str value) (v-str value)]
             [(e-bool value) (v-bool value)]
             [(e-op op left right)
              (let ([l (interp left)]
                    [r (interp right)])
                (type-case Operator op
                           [(op-plus) (v-num (+ (as-num l) (as-num r)))]
                           [(op-append) (v-str (string-append (as-str l) (as-str r)))]
                           [(op-str-eq) (v-bool (string=? (as-str l) (as-str r)))]
                           [(op-num-eq) (v-bool (= (as-num l) (as-num r)))]))]
             [else (error 'interp "NYI")]))

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

