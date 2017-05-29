
fn func(x:int, y:int) -> int {
    var z:int[n];

    z = x+y;
    #if z == 1 {


    } else {

    }

    for var i:int =0; i<10; i=i+1 {
    }

    {
        // comment
    }

    #{
        comment
    }

    return z;
}


program ::= (var_decl | func_decl | comment_stmt)*

var_decl ::= "var" ident ":" type ("=" logical_or_exp)? ";"

func_decl ::= "fn" ident "(" (ident ":" type ( "," ident ":" type)*)? ")" ("->" type)? block_stmt


stmt ::= block_stmt | if_stmt | for_stmt | assign_exp_stmt | comment_stmt | break_stmt | continue_stmt | return_stmt

block_stmt ::= "{" (var_decl | stmt)* "}"

if_stmt ::= "if" exp block_stmt ("else" block_stmt)?

for_stmt ::= "for" (var_decl | assign_exp_stmt) assign_exp_stmt assign_exp? block_stmt

assign_exp_stmt ::= assign_exp? ";"

comment_stmt ::= "#" (stmt | var_decl | func_decl)

break_stmt ::= "break" ";"

continue_stmt ::= "continue" ";"

return_stmt ::= "return" logical_or_stmt


assign_exp ::= (left_value "=")? logical_or_exp

logical_or_exp ::= logical_and_exp ("||" logical_and_exp)*

logical_and_exp ::= compare_exp ("&&" compare_exp)*

compare_exp ::= add_sub_exp (("=="|"!="|"<"|">"|"<="|">=") add_sub_exp)*

add_sub_exp ::= mul_div_exp (("+"|"-") mul_div_exp)*

mul_div_exp ::= unary_exp (("*"|"/"|"%") unary_exp)*

unary_exp ::= ("+"|"-")* member_access_exp

member_access_exp ::= func_call_exp ("." ident (arg_list | array_list)*)*

func_array_exp ::= primary_exp (arg_list | array_list)*

primary_exp ::= "(" logical_or_exp ")" | ident


