#lang plait

;;
;; Parse
;;

(define (parse [s : S-Exp]) : Exp
  (cond
    ;; Terminals
    [(s-exp-number? s) (numE (s-exp->number s))]
    [(s-exp-symbol? s) (varE (s-exp->symbol s))]
    [(s-exp-boolean? s) (boolE (s-exp->boolean s))]

    ;; Special forms
    [(s-exp-match? `(let1 (SYMBOL ANY) ANY) s)
     (let* ([l (s-exp->list s)]
            [var-list (s-exp->list (second l))]
            [var (s-exp->symbol (first var-list))]
            [value (second var-list)]
            [body (third l)])
       (let1E var (parse value) (parse body)))]
    [(s-exp-match? `(lam SYMBOL ANY) s)
     (let* ([l (s-exp->list s)]
            [var (s-exp->symbol (second l))]
            [body (third l)])
       (lamE var (parse body)))]
    [(s-exp-match? `(if ANY ANY ANY) s)
     (let* ([l (s-exp->list s)]
            [test-sexp (second l)]
            [then-sexp (third l)]
            [else-sexp (fourth l)])
       (cndE (parse test-sexp) (parse then-sexp) (parse else-sexp)))]

    ;; Unary function
    [(s-exp-match? `(ANY ANY) s)
     (let* ([l (s-exp->list s)]
            [fun (first l)]
            [arg (second l)])
       (appE (parse fun) (parse arg)))]
    
    ;; Binary functions
    [(s-exp-match? `(SYMBOL ANY ANY) s)
     (let* ([l (s-exp->list s)]
            [f (s-exp->symbol (first l))]
            [x (second l)]
            [y (third l)])
       (cond
         [(symbol=? f '+) (addE (parse x) (parse y))]
         [(symbol=? f '-) (subE (parse x) (parse y))]
         [(symbol=? f '*) (mulE (parse x) (parse y))]
         [(symbol=? f '/) (divE (parse x) (parse y))]
         [else (error 'parse "unrecognized symbol applied")]))]
    [else (error 'parse "unrecognized form in s-expression")]))

;;
;; Expression
;;

(define-type Exp
  [numE (n : Number)]
  [boolE (b : Boolean)]
  [varE (name : Symbol)]
  [cndE (test : Exp) (then : Exp) (else : Exp)]
  [let1E (var : Symbol) (value : Exp) (body : Exp)]
  [lamE (var : Symbol) (body : Exp)]
  [appE (fun : Exp) (arg : Exp)]
  [addE (left : Exp) (right : Exp)]
  [subE (left : Exp) (right : Exp)]
  [mulE (left : Exp) (right : Exp)]
  [divE (left : Exp) (right : Exp)])

;;
;; Value
;;

(define-type Value
  [numV (the-number : Number)]
  [boolV (the-boolean : Boolean)]
  [closV (var : Symbol) (body : Exp) (nv : Env)])

;;
;; Interpret
;;

(define-type-alias Env (Hashof Symbol Value))

;; Interpret from an empty environment
(define (calc [e : Exp]) : Value
  (interp e (hash empty)))

(define (interp [e : Exp] [nv : Env]) : Value 
  (type-case Exp e
             [(numE n) (numV n)]
             [(boolE b) (boolV b)]
             [(addE l r) (binary-numeric + '+ (interp l nv) (interp r nv))]
             [(subE l r) (binary-numeric - '- (interp l nv) (interp r nv))]
             [(mulE l r) (binary-numeric * '* (interp l nv) (interp r nv))]
             [(divE l r) (binary-numeric divide '/ (interp l nv) (interp r nv))]
             [(cndE c t e) (if (as-boolean (interp c nv))
                               (interp t nv)
                               (interp e nv))]
             [(varE s) (lookup s nv)]
             [(let1E var val body)
              (let ([new-env (hash-set nv var (interp val nv))])
                (interp body new-env))]
             [(lamE var body) (error 'interp "lamE NYI")]
             [(appE fun arg) (error 'interp "appE NYI")]))


;; Lookup symbol in environment, error if not found
(define (lookup [s : Symbol] [nv : Env]) : Value
  (type-case (Optionof Value) (hash-ref nv s)
             [(none) (error s "not bound")]
             [(some v) v]))


;; Apply a binary function to two values which must be numbers
(define (binary-numeric [f : (Number Number -> Number)] [symbol : Symbol] [v1 : Value] [v2 : Value]) : Value
  (type-case Value v1
             [(numV n1)
              (type-case Value v2
                         [(numV n2) (numV (f n1 n2))]
                         [else (error symbol "expects right hand side to be a number")])]
             [else (error symbol "expects left hand side to be a be a number")]))

;; Integer division catching division by zero
(define (divide [n1 : Number] [n2 : Number]) : Number
  (if (zero? n2)
      (error '/ "division by zero")
      (floor (/ n1 n2))))

;; Return value if its a boolean, otherwise error
(define (as-boolean [v : Value]) : Boolean
  (type-case Value v
             [(boolV b) b]
             [else (error 'if "expects conditional to evaluate to a boolean")]))


;;
;; Run
;;

(define (run [s : S-Exp]) : Value
  (calc (parse s)))

