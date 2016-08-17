//! # yeild
//!
//! generator yield implmentation
//!

use std::any::Any;
// use generator::Generator;
use gen_impl::GeneratorImpl;
use rt::{Error, Context, ContextStack};
use reg_context::Context as RegContext;

/// switch back to parent context
#[inline]
pub fn yield_now() {
    let env = ContextStack::current();
    let mut cur = env.top();
    raw_yield_now(env, cur);
}

#[inline]
pub fn raw_yield_now(env: &mut ContextStack, cur: &mut Context) {
    let sp = &cur.stack;
    // judge if this is root context
    if sp.size() > 0 {
        env.pop();
        let parent = env.top();
        RegContext::swap(&mut cur.regs, &parent.regs);
    }
}

/// raw yiled without catch passed in para
#[inline]
fn raw_yield<T: Any>(env: &mut ContextStack, context: &mut Context, v: T) {
    // check the context
    if !context.is_generator() {
        info!("yield from none generator context");
        // do nothing, just return
        return;
        // panic!(Error::ContextErr);
    }

    // here we just panic to exit the func
    if context._ref != 1 {
        panic!(Error::Cancel);
    }

    context.set_ret(v);
    context._ref -= 1;
    raw_yield_now(env, context);
}

/// yiled something without catch passed in para
#[inline]
pub fn yield_with<T: Any>(v: T) {
    let env = ContextStack::current();
    let context = env.top();
    raw_yield(env, context, v);
}

/// get the passed in para
#[inline]
pub fn get_yield<A: Any>() -> Option<A> {
    let context = ContextStack::current().top();
    raw_get_yield(context)
}

/// get the passed in para from context
#[inline]
fn raw_get_yield<A: Any>(context: &mut Context) -> Option<A> {
    // check the context
    if !context.is_generator() {
        error!("get yield from none generator context");
        panic!(Error::ContextErr);
    }

    context.get_para()
}

/// yiled and get the send para
// here yield need to return a static lifttime value, which is Any required
// this is fine, but it's totally safe that we can refer to the function block
// since we will come back later
#[inline]
pub fn yield_<A: Any, T: Any>(v: T) -> Option<A> {
    let env = ContextStack::current();
    let context = env.top();
    raw_yield(env, context, v);
    raw_get_yield(context)
}

/// `yiled_from`
pub fn yield_from<A: Any, T: Any>(mut g: Box<GeneratorImpl<A, T>>) -> Option<A> {
    let env = ContextStack::current();
    let context = env.top();
    let mut p = context.get_para();
    while !g.is_done() {
        let r = g.raw_send(p).unwrap();
        raw_yield(env, context, r);
        p = context.get_para();
    }
    p
}

/// get the current context
pub fn get_context() -> &'static mut Context {
    let env = ContextStack::current();
    env.top()
}
