#lang plait
(print-only-errors #true)

(define (parse [s : S-Exp]) : Exp
  (cond
    [(s-exp-number? s) (num (s-exp->number s))]
    [(s-exp-symbol? s) (error 'parse "cannot parse a symbol")]
    [(s-exp-boolean? s) (error 'parse "cannot parse a boolean")]
    [(s-exp-match? `(SYMBOL ANY ANY) s)
     (let* ([l (s-exp->list s)]
            [f (s-exp->symbol (first l))]
            [x (second l)]
            [y (third l)])
       (cond
         [(symbol=? f '+) (plus (parse x) (parse y))]
         [else (error 'parse "unrecognized symbol applied")]))]
    [else (error 'parse "unrecognized form in s-expression")]))

(module+ test
  (test (parse `1) (num 1))
  (test (parse `2.3) (num 2.3))
  (test (parse `{+ 1 2}) (plus (num 1) (num 2)))
  (test (parse `{+ 1 {+ {+ 2 3} 4}})
        (plus (num 1)
              (plus (plus (num 2) (num 3))(num 4))))
  (test/exn (parse `z) "cannot parse a symbol")
  (test/exn (parse `#f) "cannot parse a boolean")
  (test/exn (parse `{- 3 2}) "unrecognized symbol")
  (test/exn (parse `{1 + 2}) "unrecognized form"))

(define-type Exp
  [num (n : Number)]
  [plus (left : Exp) (right : Exp)])

(define (calc [e : Exp]) : Number
  (type-case Exp e
             [(num n) n]
             [(plus l r) (+ (calc l) (calc r))]))

(module+ test
  (test (calc (num 1)) 1)
  (test (calc (num 2.3)) 2.3)
  (test (calc (plus (num 1) (num 2))) 3)
  (test (calc (plus (plus (num 1) (num 2)) (num 3))) 6)
  (test (calc (plus (num 1) (plus (num 2) (num 4)))) 7)
  (test (calc (plus (num 1) (plus (plus (num 2) (num 3))(num 4)))) 10)
  (test (calc (plus (num 0.1) (num 0.2))) 0.3))
