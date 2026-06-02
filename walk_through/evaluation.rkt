#lang plait
(print-only-errors #true)

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

(module+ test
  (test (parse `1) (numE 1))
  (test (parse `2.3) (numE 2.3))
  (test (parse `{+ 1 2}) (plusE (numE 1) (numE 2)))
  (test (parse `{+ 1 {+ {+ 2 3} 4}})
        (plusE (numE 1)
               (plusE (plusE (numE 2) (numE 3))(numE 4))))
  (test (parse `#f) (boolE #f))
  (test (parse `#t) (boolE #t))
  (test (parse `#false) (boolE #f))
  (test (parse `#true) (boolE #t))
  (test/exn (parse `z) "cannot parse a symbol")
  (test/exn (parse `{- 3 2}) "unrecognized symbol")
  (test/exn (parse `{1 + 2}) "unrecognized form"))

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

(module+ test
  (test (calc (numE 1)) (numV 1))
  (test (calc (numE 2.3)) (numV 2.3))
  (test (calc (plusE (numE 1) (numE 2))) (numV 3))
  (test (calc (plusE (plusE (numE 1) (numE 2)) (numE 3))) (numV 6))
  (test (calc (plusE (numE 1) (plusE (numE 2) (numE 4)))) (numV 7))
  (test (calc (plusE (numE 1) (plusE (plusE (numE 2) (numE 3))(numE 4)))) (numV 10))
  (test/exn (calc (plusE (numE 1) (boolE #t))) "right hand side")
  (test/exn (calc (plusE (boolE #f) (boolE #t))) "left hand side")
  (test (calc (cndE (boolE #f) (numE 2) (numE 3))) (numV 3))
  (test (calc (cndE (boolE #t) (numE 2) (numE 3))) (numV 2))
  (test/exn (calc (cndE (numE 2) (numE 2) (numE 3))) "boolean"))


;;
;; Run
;;

(define (run [s : S-Exp]) : Value
  (calc (parse s)))

(module+ test
  (test (run `1) (numV 1))
  (test (run `2.3) (numV 2.3))
  (test (run `{+ 1 2}) (numV 3))
  (test (run `{+ {+ 1 2} 3}) (numV 6))
  (test (run `{+ 1 {+ 2 3}}) (numV 6))
  (test (run `{+ 1 {+ {+ 2 3} 4}}) (numV 10)))
