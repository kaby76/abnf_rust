// Generated from Abnf.g4 by ANTLR 4.13.2

use super::abnfparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by AbnfParser.

pub trait AbnfBaseListener<'input>:
    ParseTreeListener<'input, AbnfParserContextType> {

    /**
     * Enter a parse tree produced by \{@link AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_rulelist(&mut self, _ctx: &RulelistContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_rulelist(&mut self, _ctx: &RulelistContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_rule_(&mut self, _ctx: &Rule_Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_rule_(&mut self, _ctx: &Rule_Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elements(&mut self, _ctx: &ElementsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elements(&mut self, _ctx: &ElementsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_alternation(&mut self, _ctx: &AlternationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_alternation(&mut self, _ctx: &AlternationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_concatenation(&mut self, _ctx: &ConcatenationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_concatenation(&mut self, _ctx: &ConcatenationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_repetition(&mut self, _ctx: &RepetitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_repetition(&mut self, _ctx: &RepetitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_repeat_(&mut self, _ctx: &Repeat_Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_repeat_(&mut self, _ctx: &Repeat_Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_element(&mut self, _ctx: &ElementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_element(&mut self, _ctx: &ElementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_group(&mut self, _ctx: &GroupContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_group(&mut self, _ctx: &GroupContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_option(&mut self, _ctx: &OptionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  AbnfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_option(&mut self, _ctx: &OptionContext<'input>) {}


}