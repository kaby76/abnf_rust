// Generated from Abnf.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr4rust::PredictionContextCache;
use antlr4rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr4rust::token_stream::TokenStream;
use antlr4rust::TokenSource;
use antlr4rust::parser_atn_simulator::ParserATNSimulator;
use antlr4rust::errors::*;
use antlr4rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::atn::{ATN, INVALID_ALT};
use antlr4rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr4rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr4rust::tree::*;
use antlr4rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr4rust::int_stream::EOF;
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr4rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::abnflistener::*;
use antlr4rust::lazy_static;
use antlr4rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const Abnf_T__0:i32=1; 
		pub const Abnf_T__1:i32=2; 
		pub const Abnf_T__2:i32=3; 
		pub const Abnf_T__3:i32=4; 
		pub const Abnf_T__4:i32=5; 
		pub const Abnf_T__5:i32=6; 
		pub const Abnf_T__6:i32=7; 
		pub const Abnf_NumberValue:i32=8; 
		pub const Abnf_ProseValue:i32=9; 
		pub const Abnf_ID:i32=10; 
		pub const Abnf_INT:i32=11; 
		pub const Abnf_COMMENT:i32=12; 
		pub const Abnf_WS:i32=13; 
		pub const Abnf_STRING:i32=14;
	pub const Abnf_EOF:i32=EOF;
	pub const RULE_rulelist:usize = 0; 
	pub const RULE_rule_:usize = 1; 
	pub const RULE_elements:usize = 2; 
	pub const RULE_alternation:usize = 3; 
	pub const RULE_concatenation:usize = 4; 
	pub const RULE_repetition:usize = 5; 
	pub const RULE_repeat_:usize = 6; 
	pub const RULE_element:usize = 7; 
	pub const RULE_group:usize = 8; 
	pub const RULE_option:usize = 9;
	pub const ruleNames: [&'static str; 10] =  [
		"rulelist", "rule_", "elements", "alternation", "concatenation", "repetition", 
		"repeat_", "element", "group", "option"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;8] = [
		None, Some("'='"), Some("'/'"), Some("'*'"), Some("'('"), Some("')'"), 
		Some("'['"), Some("']'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;15]  = [
		None, None, None, None, None, None, None, None, Some("NumberValue"), Some("ProseValue"), 
		Some("ID"), Some("INT"), Some("COMMENT"), Some("WS"), Some("STRING")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,AbnfParserExt<'input>, I, AbnfParserContextType , dyn AbnfListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type AbnfTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, AbnfParserContextType , dyn AbnfListener<'input> + 'a>;

/// Parser for Abnf grammar
pub struct AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >,
}

impl<'input, I> AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) -> Self {
		antlr4rust::recognizer::check_version("0","5");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				AbnfParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for AbnfParser
pub trait AbnfParserContext<'input>:
	for<'x> Listenable<dyn AbnfListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=AbnfParserContextType>
{}

antlr4rust::coerce_from!{ 'input : AbnfParserContext<'input> }

impl<'input> AbnfParserContext<'input> for TerminalNode<'input,AbnfParserContextType> {}
impl<'input> AbnfParserContext<'input> for ErrorNode<'input,AbnfParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn AbnfParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn AbnfListener<'input> + 'input }

pub struct AbnfParserContextType;
antlr4rust::tid!{AbnfParserContextType}

impl<'input> ParserNodeType<'input> for AbnfParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn AbnfParserContext<'input> + 'input;
}

impl<'input, I> Deref for AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct AbnfParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> AbnfParserExt<'input>{
}
antlr4rust::tid! { AbnfParserExt<'a> }

impl<'input> TokenAware<'input> for AbnfParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for AbnfParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for AbnfParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "Abnf.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- rulelist ----------------
pub type RulelistContextAll<'input> = RulelistContext<'input>;


pub type RulelistContext<'input> = BaseParserRuleContext<'input,RulelistContextExt<'input>>;

#[derive(Clone)]
pub struct RulelistContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AbnfParserContext<'input> for RulelistContext<'input>{}

impl<'input,'a> Listenable<dyn AbnfListener<'input> + 'a> for RulelistContext<'input>{
		fn enter(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_rulelist(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_rulelist(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for RulelistContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AbnfParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rulelist }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rulelist }
}
antlr4rust::tid!{RulelistContextExt<'a>}

impl<'input> RulelistContextExt<'input>{
	fn new(parent: Option<Rc<dyn AbnfParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<RulelistContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RulelistContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait RulelistContextAttrs<'input>: AbnfParserContext<'input> + BorrowMut<RulelistContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,AbnfParserContextType>>> where Self:Sized{
	self.get_token(Abnf_EOF, 0)
}
fn rule__all(&self) ->  Vec<Rc<Rule_ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn rule_(&self, i: usize) -> Option<Rc<Rule_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RulelistContextAttrs<'input> for RulelistContext<'input>{}

impl<'input, I> AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn rulelist(&mut self,)
	-> Result<Rc<RulelistContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RulelistContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_rulelist);
        let mut _localctx: Rc<RulelistContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(23);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Abnf_ID {
				{
				{
				/*InvokeRule rule_*/
				recog.base.set_state(20);
				recog.rule_()?;

				}
				}
				recog.base.set_state(25);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(26);
			recog.base.match_token(Abnf_EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- rule_ ----------------
pub type Rule_ContextAll<'input> = Rule_Context<'input>;


pub type Rule_Context<'input> = BaseParserRuleContext<'input,Rule_ContextExt<'input>>;

#[derive(Clone)]
pub struct Rule_ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AbnfParserContext<'input> for Rule_Context<'input>{}

impl<'input,'a> Listenable<dyn AbnfListener<'input> + 'a> for Rule_Context<'input>{
		fn enter(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_rule_(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_rule_(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Rule_ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AbnfParserContextType;
	fn get_rule_index(&self) -> usize { RULE_rule_ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_rule_ }
}
antlr4rust::tid!{Rule_ContextExt<'a>}

impl<'input> Rule_ContextExt<'input>{
	fn new(parent: Option<Rc<dyn AbnfParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Rule_ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Rule_ContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Rule_ContextAttrs<'input>: AbnfParserContext<'input> + BorrowMut<Rule_ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,AbnfParserContextType>>> where Self:Sized{
	self.get_token(Abnf_ID, 0)
}
fn elements(&self) -> Option<Rc<ElementsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Rule_ContextAttrs<'input> for Rule_Context<'input>{}

impl<'input, I> AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn rule_(&mut self,)
	-> Result<Rc<Rule_ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Rule_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_rule_);
        let mut _localctx: Rc<Rule_ContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(28);
			recog.base.match_token(Abnf_ID,&mut recog.err_handler)?;

			recog.base.set_state(29);
			recog.base.match_token(Abnf_T__0,&mut recog.err_handler)?;

			recog.base.set_state(31);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Abnf_T__1 {
				{
				recog.base.set_state(30);
				recog.base.match_token(Abnf_T__1,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule elements*/
			recog.base.set_state(33);
			recog.elements()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- elements ----------------
pub type ElementsContextAll<'input> = ElementsContext<'input>;


pub type ElementsContext<'input> = BaseParserRuleContext<'input,ElementsContextExt<'input>>;

#[derive(Clone)]
pub struct ElementsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AbnfParserContext<'input> for ElementsContext<'input>{}

impl<'input,'a> Listenable<dyn AbnfListener<'input> + 'a> for ElementsContext<'input>{
		fn enter(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_elements(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_elements(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ElementsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AbnfParserContextType;
	fn get_rule_index(&self) -> usize { RULE_elements }
	//fn type_rule_index() -> usize where Self: Sized { RULE_elements }
}
antlr4rust::tid!{ElementsContextExt<'a>}

impl<'input> ElementsContextExt<'input>{
	fn new(parent: Option<Rc<dyn AbnfParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ElementsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ElementsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ElementsContextAttrs<'input>: AbnfParserContext<'input> + BorrowMut<ElementsContextExt<'input>>{

fn alternation(&self) -> Option<Rc<AlternationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ElementsContextAttrs<'input> for ElementsContext<'input>{}

impl<'input, I> AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn elements(&mut self,)
	-> Result<Rc<ElementsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ElementsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_elements);
        let mut _localctx: Rc<ElementsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule alternation*/
			recog.base.set_state(35);
			recog.alternation()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- alternation ----------------
pub type AlternationContextAll<'input> = AlternationContext<'input>;


pub type AlternationContext<'input> = BaseParserRuleContext<'input,AlternationContextExt<'input>>;

#[derive(Clone)]
pub struct AlternationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AbnfParserContext<'input> for AlternationContext<'input>{}

impl<'input,'a> Listenable<dyn AbnfListener<'input> + 'a> for AlternationContext<'input>{
		fn enter(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_alternation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_alternation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for AlternationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AbnfParserContextType;
	fn get_rule_index(&self) -> usize { RULE_alternation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_alternation }
}
antlr4rust::tid!{AlternationContextExt<'a>}

impl<'input> AlternationContextExt<'input>{
	fn new(parent: Option<Rc<dyn AbnfParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AlternationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AlternationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AlternationContextAttrs<'input>: AbnfParserContext<'input> + BorrowMut<AlternationContextExt<'input>>{

fn concatenation_all(&self) ->  Vec<Rc<ConcatenationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn concatenation(&self, i: usize) -> Option<Rc<ConcatenationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AlternationContextAttrs<'input> for AlternationContext<'input>{}

impl<'input, I> AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn alternation(&mut self,)
	-> Result<Rc<AlternationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AlternationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_alternation);
        let mut _localctx: Rc<AlternationContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule concatenation*/
			recog.base.set_state(37);
			recog.concatenation()?;

			recog.base.set_state(42);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==Abnf_T__1 {
				{
				{
				recog.base.set_state(38);
				recog.base.match_token(Abnf_T__1,&mut recog.err_handler)?;

				/*InvokeRule concatenation*/
				recog.base.set_state(39);
				recog.concatenation()?;

				}
				}
				recog.base.set_state(44);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- concatenation ----------------
pub type ConcatenationContextAll<'input> = ConcatenationContext<'input>;


pub type ConcatenationContext<'input> = BaseParserRuleContext<'input,ConcatenationContextExt<'input>>;

#[derive(Clone)]
pub struct ConcatenationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AbnfParserContext<'input> for ConcatenationContext<'input>{}

impl<'input,'a> Listenable<dyn AbnfListener<'input> + 'a> for ConcatenationContext<'input>{
		fn enter(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_concatenation(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_concatenation(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ConcatenationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AbnfParserContextType;
	fn get_rule_index(&self) -> usize { RULE_concatenation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_concatenation }
}
antlr4rust::tid!{ConcatenationContextExt<'a>}

impl<'input> ConcatenationContextExt<'input>{
	fn new(parent: Option<Rc<dyn AbnfParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ConcatenationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConcatenationContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ConcatenationContextAttrs<'input>: AbnfParserContext<'input> + BorrowMut<ConcatenationContextExt<'input>>{

fn repetition_all(&self) ->  Vec<Rc<RepetitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn repetition(&self, i: usize) -> Option<Rc<RepetitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ConcatenationContextAttrs<'input> for ConcatenationContext<'input>{}

impl<'input, I> AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn concatenation(&mut self,)
	-> Result<Rc<ConcatenationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConcatenationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_concatenation);
        let mut _localctx: Rc<ConcatenationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(46); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					/*InvokeRule repetition*/
					recog.base.set_state(45);
					recog.repetition()?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(48); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- repetition ----------------
pub type RepetitionContextAll<'input> = RepetitionContext<'input>;


pub type RepetitionContext<'input> = BaseParserRuleContext<'input,RepetitionContextExt<'input>>;

#[derive(Clone)]
pub struct RepetitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AbnfParserContext<'input> for RepetitionContext<'input>{}

impl<'input,'a> Listenable<dyn AbnfListener<'input> + 'a> for RepetitionContext<'input>{
		fn enter(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_repetition(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_repetition(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for RepetitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AbnfParserContextType;
	fn get_rule_index(&self) -> usize { RULE_repetition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_repetition }
}
antlr4rust::tid!{RepetitionContextExt<'a>}

impl<'input> RepetitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn AbnfParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<RepetitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RepetitionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait RepetitionContextAttrs<'input>: AbnfParserContext<'input> + BorrowMut<RepetitionContextExt<'input>>{

fn element(&self) -> Option<Rc<ElementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn repeat_(&self) -> Option<Rc<Repeat_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RepetitionContextAttrs<'input> for RepetitionContext<'input>{}

impl<'input, I> AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn repetition(&mut self,)
	-> Result<Rc<RepetitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RepetitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_repetition);
        let mut _localctx: Rc<RepetitionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(51);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==Abnf_T__2 || _la==Abnf_INT {
				{
				/*InvokeRule repeat_*/
				recog.base.set_state(50);
				recog.repeat_()?;

				}
			}

			/*InvokeRule element*/
			recog.base.set_state(53);
			recog.element()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- repeat_ ----------------
pub type Repeat_ContextAll<'input> = Repeat_Context<'input>;


pub type Repeat_Context<'input> = BaseParserRuleContext<'input,Repeat_ContextExt<'input>>;

#[derive(Clone)]
pub struct Repeat_ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AbnfParserContext<'input> for Repeat_Context<'input>{}

impl<'input,'a> Listenable<dyn AbnfListener<'input> + 'a> for Repeat_Context<'input>{
		fn enter(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_repeat_(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_repeat_(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for Repeat_ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AbnfParserContextType;
	fn get_rule_index(&self) -> usize { RULE_repeat_ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_repeat_ }
}
antlr4rust::tid!{Repeat_ContextExt<'a>}

impl<'input> Repeat_ContextExt<'input>{
	fn new(parent: Option<Rc<dyn AbnfParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Repeat_ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Repeat_ContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Repeat_ContextAttrs<'input>: AbnfParserContext<'input> + BorrowMut<Repeat_ContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token INT in current rule
fn INT_all(&self) -> Vec<Rc<TerminalNode<'input,AbnfParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token INT, starting from 0.
/// Returns `None` if number of children corresponding to token INT is less or equal than `i`.
fn INT(&self, i: usize) -> Option<Rc<TerminalNode<'input,AbnfParserContextType>>> where Self:Sized{
	self.get_token(Abnf_INT, i)
}

}

impl<'input> Repeat_ContextAttrs<'input> for Repeat_Context<'input>{}

impl<'input, I> AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn repeat_(&mut self,)
	-> Result<Rc<Repeat_ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Repeat_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_repeat_);
        let mut _localctx: Rc<Repeat_ContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(63);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(7,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(55);
					recog.base.match_token(Abnf_INT,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(57);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Abnf_INT {
						{
						recog.base.set_state(56);
						recog.base.match_token(Abnf_INT,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(59);
					recog.base.match_token(Abnf_T__2,&mut recog.err_handler)?;

					recog.base.set_state(61);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==Abnf_INT {
						{
						recog.base.set_state(60);
						recog.base.match_token(Abnf_INT,&mut recog.err_handler)?;

						}
					}

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- element ----------------
pub type ElementContextAll<'input> = ElementContext<'input>;


pub type ElementContext<'input> = BaseParserRuleContext<'input,ElementContextExt<'input>>;

#[derive(Clone)]
pub struct ElementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AbnfParserContext<'input> for ElementContext<'input>{}

impl<'input,'a> Listenable<dyn AbnfListener<'input> + 'a> for ElementContext<'input>{
		fn enter(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_element(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_element(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ElementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AbnfParserContextType;
	fn get_rule_index(&self) -> usize { RULE_element }
	//fn type_rule_index() -> usize where Self: Sized { RULE_element }
}
antlr4rust::tid!{ElementContextExt<'a>}

impl<'input> ElementContextExt<'input>{
	fn new(parent: Option<Rc<dyn AbnfParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ElementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ElementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ElementContextAttrs<'input>: AbnfParserContext<'input> + BorrowMut<ElementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,AbnfParserContextType>>> where Self:Sized{
	self.get_token(Abnf_ID, 0)
}
fn group(&self) -> Option<Rc<GroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn option(&self) -> Option<Rc<OptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,AbnfParserContextType>>> where Self:Sized{
	self.get_token(Abnf_STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token NumberValue
/// Returns `None` if there is no child corresponding to token NumberValue
fn NumberValue(&self) -> Option<Rc<TerminalNode<'input,AbnfParserContextType>>> where Self:Sized{
	self.get_token(Abnf_NumberValue, 0)
}
/// Retrieves first TerminalNode corresponding to token ProseValue
/// Returns `None` if there is no child corresponding to token ProseValue
fn ProseValue(&self) -> Option<Rc<TerminalNode<'input,AbnfParserContextType>>> where Self:Sized{
	self.get_token(Abnf_ProseValue, 0)
}

}

impl<'input> ElementContextAttrs<'input> for ElementContext<'input>{}

impl<'input, I> AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn element(&mut self,)
	-> Result<Rc<ElementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_element);
        let mut _localctx: Rc<ElementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(71);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Abnf_ID 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(65);
					recog.base.match_token(Abnf_ID,&mut recog.err_handler)?;

					}
				}

			Abnf_T__3 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule group*/
					recog.base.set_state(66);
					recog.group()?;

					}
				}

			Abnf_T__5 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule option*/
					recog.base.set_state(67);
					recog.option()?;

					}
				}

			Abnf_STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(68);
					recog.base.match_token(Abnf_STRING,&mut recog.err_handler)?;

					}
				}

			Abnf_NumberValue 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(69);
					recog.base.match_token(Abnf_NumberValue,&mut recog.err_handler)?;

					}
				}

			Abnf_ProseValue 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					recog.base.set_state(70);
					recog.base.match_token(Abnf_ProseValue,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- group ----------------
pub type GroupContextAll<'input> = GroupContext<'input>;


pub type GroupContext<'input> = BaseParserRuleContext<'input,GroupContextExt<'input>>;

#[derive(Clone)]
pub struct GroupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AbnfParserContext<'input> for GroupContext<'input>{}

impl<'input,'a> Listenable<dyn AbnfListener<'input> + 'a> for GroupContext<'input>{
		fn enter(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_group(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_group(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for GroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AbnfParserContextType;
	fn get_rule_index(&self) -> usize { RULE_group }
	//fn type_rule_index() -> usize where Self: Sized { RULE_group }
}
antlr4rust::tid!{GroupContextExt<'a>}

impl<'input> GroupContextExt<'input>{
	fn new(parent: Option<Rc<dyn AbnfParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<GroupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GroupContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait GroupContextAttrs<'input>: AbnfParserContext<'input> + BorrowMut<GroupContextExt<'input>>{

fn alternation(&self) -> Option<Rc<AlternationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> GroupContextAttrs<'input> for GroupContext<'input>{}

impl<'input, I> AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn group(&mut self,)
	-> Result<Rc<GroupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GroupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_group);
        let mut _localctx: Rc<GroupContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(73);
			recog.base.match_token(Abnf_T__3,&mut recog.err_handler)?;

			/*InvokeRule alternation*/
			recog.base.set_state(74);
			recog.alternation()?;

			recog.base.set_state(75);
			recog.base.match_token(Abnf_T__4,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- option ----------------
pub type OptionContextAll<'input> = OptionContext<'input>;


pub type OptionContext<'input> = BaseParserRuleContext<'input,OptionContextExt<'input>>;

#[derive(Clone)]
pub struct OptionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AbnfParserContext<'input> for OptionContext<'input>{}

impl<'input,'a> Listenable<dyn AbnfListener<'input> + 'a> for OptionContext<'input>{
		fn enter(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_option(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn AbnfListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_option(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OptionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AbnfParserContextType;
	fn get_rule_index(&self) -> usize { RULE_option }
	//fn type_rule_index() -> usize where Self: Sized { RULE_option }
}
antlr4rust::tid!{OptionContextExt<'a>}

impl<'input> OptionContextExt<'input>{
	fn new(parent: Option<Rc<dyn AbnfParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OptionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OptionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OptionContextAttrs<'input>: AbnfParserContext<'input> + BorrowMut<OptionContextExt<'input>>{

fn alternation(&self) -> Option<Rc<AlternationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OptionContextAttrs<'input> for OptionContext<'input>{}

impl<'input, I> AbnfParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn option(&mut self,)
	-> Result<Rc<OptionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_option);
        let mut _localctx: Rc<OptionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(77);
			recog.base.match_token(Abnf_T__5,&mut recog.err_handler)?;

			/*InvokeRule alternation*/
			recog.base.set_state(78);
			recog.alternation()?;

			recog.base.set_state(79);
			recog.base.match_token(Abnf_T__6,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
	lazy_static!{
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
    static ref _decision_to_DFA: Arc<Vec<antlr4rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len() as i32;
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i,
            ).into())
        }
        Arc::new(dfa)
    };
	static ref _serializedATN: Vec<i32> = vec![
		4, 1, 14, 82, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 7, 
		4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 1, 0, 5, 
		0, 22, 8, 0, 10, 0, 12, 0, 25, 9, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 3, 
		1, 32, 8, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 3, 1, 3, 1, 3, 5, 3, 41, 8, 3, 
		10, 3, 12, 3, 44, 9, 3, 1, 4, 4, 4, 47, 8, 4, 11, 4, 12, 4, 48, 1, 5, 
		3, 5, 52, 8, 5, 1, 5, 1, 5, 1, 6, 1, 6, 3, 6, 58, 8, 6, 1, 6, 1, 6, 3, 
		6, 62, 8, 6, 3, 6, 64, 8, 6, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 3, 7, 
		72, 8, 7, 1, 8, 1, 8, 1, 8, 1, 8, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 0, 0, 
		10, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 0, 0, 84, 0, 23, 1, 0, 0, 0, 2, 
		28, 1, 0, 0, 0, 4, 35, 1, 0, 0, 0, 6, 37, 1, 0, 0, 0, 8, 46, 1, 0, 0, 
		0, 10, 51, 1, 0, 0, 0, 12, 63, 1, 0, 0, 0, 14, 71, 1, 0, 0, 0, 16, 73, 
		1, 0, 0, 0, 18, 77, 1, 0, 0, 0, 20, 22, 3, 2, 1, 0, 21, 20, 1, 0, 0, 0, 
		22, 25, 1, 0, 0, 0, 23, 21, 1, 0, 0, 0, 23, 24, 1, 0, 0, 0, 24, 26, 1, 
		0, 0, 0, 25, 23, 1, 0, 0, 0, 26, 27, 5, 0, 0, 1, 27, 1, 1, 0, 0, 0, 28, 
		29, 5, 10, 0, 0, 29, 31, 5, 1, 0, 0, 30, 32, 5, 2, 0, 0, 31, 30, 1, 0, 
		0, 0, 31, 32, 1, 0, 0, 0, 32, 33, 1, 0, 0, 0, 33, 34, 3, 4, 2, 0, 34, 
		3, 1, 0, 0, 0, 35, 36, 3, 6, 3, 0, 36, 5, 1, 0, 0, 0, 37, 42, 3, 8, 4, 
		0, 38, 39, 5, 2, 0, 0, 39, 41, 3, 8, 4, 0, 40, 38, 1, 0, 0, 0, 41, 44, 
		1, 0, 0, 0, 42, 40, 1, 0, 0, 0, 42, 43, 1, 0, 0, 0, 43, 7, 1, 0, 0, 0, 
		44, 42, 1, 0, 0, 0, 45, 47, 3, 10, 5, 0, 46, 45, 1, 0, 0, 0, 47, 48, 1, 
		0, 0, 0, 48, 46, 1, 0, 0, 0, 48, 49, 1, 0, 0, 0, 49, 9, 1, 0, 0, 0, 50, 
		52, 3, 12, 6, 0, 51, 50, 1, 0, 0, 0, 51, 52, 1, 0, 0, 0, 52, 53, 1, 0, 
		0, 0, 53, 54, 3, 14, 7, 0, 54, 11, 1, 0, 0, 0, 55, 64, 5, 11, 0, 0, 56, 
		58, 5, 11, 0, 0, 57, 56, 1, 0, 0, 0, 57, 58, 1, 0, 0, 0, 58, 59, 1, 0, 
		0, 0, 59, 61, 5, 3, 0, 0, 60, 62, 5, 11, 0, 0, 61, 60, 1, 0, 0, 0, 61, 
		62, 1, 0, 0, 0, 62, 64, 1, 0, 0, 0, 63, 55, 1, 0, 0, 0, 63, 57, 1, 0, 
		0, 0, 64, 13, 1, 0, 0, 0, 65, 72, 5, 10, 0, 0, 66, 72, 3, 16, 8, 0, 67, 
		72, 3, 18, 9, 0, 68, 72, 5, 14, 0, 0, 69, 72, 5, 8, 0, 0, 70, 72, 5, 9, 
		0, 0, 71, 65, 1, 0, 0, 0, 71, 66, 1, 0, 0, 0, 71, 67, 1, 0, 0, 0, 71, 
		68, 1, 0, 0, 0, 71, 69, 1, 0, 0, 0, 71, 70, 1, 0, 0, 0, 72, 15, 1, 0, 
		0, 0, 73, 74, 5, 4, 0, 0, 74, 75, 3, 6, 3, 0, 75, 76, 5, 5, 0, 0, 76, 
		17, 1, 0, 0, 0, 77, 78, 5, 6, 0, 0, 78, 79, 3, 6, 3, 0, 79, 80, 5, 7, 
		0, 0, 80, 19, 1, 0, 0, 0, 9, 23, 31, 42, 48, 51, 57, 61, 63, 71
	];
}
