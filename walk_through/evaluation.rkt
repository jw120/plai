#lang plait

;;
;; Parse
;;

(define (parse [s : S-Exp]) : Exp
  (cond
    [(s-exp-number? s) (numE (s-exp->number s))]
    [(s-exp-symbol? s) (error 'parse "cannot parse a symbol")]
    [(s-exp-boolean? s) (boolE (s-exp->boolean s))]
    [(s-exp-match? `(SYMBOL ANY ANY) s)
     (let* ([l (s-exp->list s)]
            [f (s-exp->symbol (first l))]
            [x (second l)]
            [y (third l)])
       (cond
         [(symbol=? f '+) (plusE (parse x) (parse y))]
         [else (error 'parse "unrecognized symbol applied")]))]
    [else (error 'parse "unrecognized form in s-expression")]))

;;
;; Expression
;;

(define-type Exp
  [numE (n : Number)]
  [boolE (b : Boolean)]
  [plusE (left : Exp) (right : Exp)]
  [cndE (test : Exp) (then : Exp) (else : Exp)])

;;
;; Value
;;

(define-type Value
  [numV (the-number : Number)]
  [boolV (the-boolean : Boolean)])

;;
;; Calculate
;;

(define (calc [e : Exp]) : Value 
  (type-case Exp e
             [(numE n) (numV n)]
             [(boolE b) (boolV b)]
             [(plusE l r) (add (calc l) (calc r))]
             [(cndE c t e) (if (boolean-decision (calc c))
                               (calc t)
                               (calc e))]))

(define (boolean-decision [v : Value]) : Boolean
  (type-case Value v
             [(boolV b) b]
             [else (error 'if "expects conditional to evaluate to a boolean")]))

(define (add [v1 : Value] [v2 : Value]) : Value
  (type-case Value v1
             [(numV n1)
              (type-case Value v2
                         [(numV n2) (numV (+ n1 n2))]
                         [else (error '+ "expects right hand side to be a number")])]
             [else (error '+ "expects left hand side to be a be a number")]))


;;
;; Run
;;

(define (run [s : S-Exp]) : Value
  (calc (parse s)))

