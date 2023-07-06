#![cfg_attr(not(feature = "no"), no_std)]

#[cfg(not(feature = "no"))]
extern crate alloc;

#[cfg(not(feature = "no"))]
use {alloc::{string::{String, ToString}, vec, vec::Vec, boxed::Box, sync::Arc, format}, spin::RwLock, core::default::Default};
#[cfg(feature = "no")]
use std::sync::{Arc, RwLock};
use core::any::Any;

#[cfg(not(feature = "no"))]
#[macro_export]
macro_rules! pr__ {
	($($arg:tt)*) => (
		cortex_m_semihosting::hprint!($($arg)*).unwrap();
	)
}

#[cfg(feature = "no")]
#[macro_export]
macro_rules! pr__ {
	($($arg:tt)*) => (
		print!($($arg)*);
	)
}

#[cfg(not(feature = "no"))] #[macro_export] macro_rules! l_r__ {($l:expr) => ($l.read())}
#[cfg    (feature = "no") ] #[macro_export] macro_rules! l_r__ {($l:expr) => ($l.read().unwrap())}
#[cfg(not(feature = "no"))] #[macro_export] macro_rules! l_w__ {($l:expr) => ($l.write())}
#[cfg    (feature = "no") ] #[macro_export] macro_rules! l_w__ {($l:expr) => ($l.write().unwrap())}
pub type T_<T> = Arc<RwLock<T>>;
fn new_t__<T>(t:T) -> T_<T> {Arc::new(RwLock::new(t))}

pub trait CodeImpl_ {
	fn kw__(&self) -> Vec<Keyword_>;
	fn s__(&self) -> Option<&str> {None}
	fn a__(&self) -> &Option<Codes_> {&None}
	fn mv_a__(&self) -> Option<Codes_> {None}
	fn a_tag__(&self) -> Option<&str> {None}
	fn as_any(&self) -> &dyn Any;
}
type CI_ = Arc<Box<dyn CodeImpl_>>;
fn new_ci__(i:Box<dyn CodeImpl_>) -> CI_ {Arc::new(i)}

#[derive(Clone)]
pub struct Codes_ {
	a_:Vec<CI_>,
}

pub mod world_ {
	#[cfg(not(feature = "no"))]
	use alloc::{string::String, vec};
	use super::{*};

	pub fn hello__(s:&str) {z2__(s, false)}
	pub fn hello2__(s:&str) {z2__(s, true)}
	pub fn hello3__(s:&str, dbg:&Debug_) {z3__(s, dbg)}
	pub fn hello4__<P:PathImpl_>(src:Text_, src_is_file:bool, v:Vec<Text_>, path:&P, data_up:Option<DA_>, qu_up:Option<Q_>, dbg:&Debug_) -> Return_ {
		let mut data = Data_::new(data_up.clone());
		data.src = if src_is_file {
			let src = &src.s_;
			let mut src2 = None;
			let mut data1 = data_up.clone();
			while data1.is_some() {
				let data = data1.clone().unwrap();
				if let Some(path2) = &data.path {
					src2 = path.open__(src, &path2);
					if src2.is_some() {
						break
					}
				}
				data1 = data.up_.clone();
			}
			if src2.is_none() {src2 = path.open__(src, "");}
			if src2.is_none() {return Return_::Err(format!("{} 失败", src))}
			data.path = Some(src.clone());
			Text_::new(src2.unwrap())
		} else {src};
		data.args.append(&mut tv_to_civ__(v));
		z4__(data, path, qu_up, dbg)
	}
	fn z2__(s:&str, tree:bool) {z3__(s, &Debug_ {tree, ..Default::default()})}
	fn z3__(s:&str, dbg:&Debug_) {
		let mut data = Data_::new(None);
		data.src = Text_::new3(s);
		match z4__(data, &Path_ {}, None, dbg) {
			Return_::Ok | Return_::Ok2(_) => {}
			ret@_ => {pr__!("{:?}\n", ret);}
		}
	}
	fn z4__<P:PathImpl_>(data:Data_, path:&P, qu_up:Option<Q_>, dbg:&Debug_) -> Return_ {
		let mut codes = vec![];
		match
			if let Some(def) = &data.by_def {
				let s = &l_r__!(def).code_;
				if dbg.src {
					pr__!("\n{}\n", s);
				}
				pars_::hello__(s, &mut codes)
			} else {
				let s = &data.src.s_;
				if dbg.src {
					pr__!("\n{}\n", s);
				}
				pars_::hello__(s, &mut codes)
			}
		{
			pars_::Return_::Ok => {}
			pars_::Return_::Err(s) => return Return_::Err(s),
			ret@_ => return Return_::Err(format!("{:?}", ret))
		}
		if dbg.tree {
			tree__(&codes);
		}
		if dbg.args {
			pr__!("args\n");
			for (idx, i) in data.args.iter().enumerate() {
				pr__!("  {}) {:?} {:?}\n", idx + 1, i.kw__(), i.s__());
			}
		}

		let data = Arc::new(data);
		let mut data2_mut = Data2Mut_::new();
		let qu = new_t__(Qu_::new(qu_up));
		let ret = z__(&codes, None, data.clone(), &Default::default(), path, &mut DataMut_::new(), &mut data2_mut, qu.clone(), dbg);
		if ret == Return_::Ok {
			match to_vec__(&data2_mut.ret, qu.clone(), data.clone(), path, dbg) {
				Ok(v) => {Return_::Ok2(v)}
				Err(ret2) => ret2
			}
		} else {ret}
	}
	fn z__<P:PathImpl_>(codes:&Vec<CI_>, end:Option<usize>, data:DA_, data2:&Data2_, path:&P, data_mut:&mut DataMut_, data2_mut:&mut Data2Mut_, qu:Q_, dbg:&Debug_) -> Return_ {
		let new_data2_2 = || {
			Data2_ {juhao:data2.juhao, juhao2:true, ..Default::default()}
		};
		loop {
			if data_mut.idx >= codes.len() {break}
			if let Some(end) = end {
				if data_mut.idx >= end {break}
			}
			let i = &codes[data_mut.idx];
			let kw = &i.kw__()[0];
			let for22 = |qu:Q_, data2:&Data2_, data2_mut2:&mut Data2Mut_, a:&Option<Codes_>, to_vec:bool, canot_empty:bool| {
				if let Some(codes2) = a {
					let ret = z__(&codes2.a_, None, data.clone(), data2, path, &mut DataMut_::new(), data2_mut2, qu.clone(), dbg);
					if ret == Return_::Ok && to_vec {
						match to_vec__(&data2_mut2.ret, qu.clone(), data.clone(), path, dbg) {
							Ok(v) => {
								if canot_empty {
									if v.is_empty() {
										return Return_::Ok
									}
								}
								Return_::Ok2(v)
							}
							Err(ret2) => ret2
						}
					} else {ret}
				} else {
					if canot_empty {
						return Return_::Ok
					}
					Return_::Err2
				}
			};
			let for221 = |qu:Q_, data2:&Data2_, data2_mut2:&mut Data2Mut_, a| {
				for22(qu, data2, data2_mut2, a, false, false)
			};
			let for21 = |qu:Q_, a| {
				for22(qu, data2, &mut Data2Mut_::new(), a, true, false)
			};
			let for2 = |qu| for21(qu, i.a__());
			let for21_ne = |qu:Q_, a| for22(qu, data2, &mut Data2Mut_::new(), a, true, true);
			let for2_ne = |qu| for21_ne(qu, i.a__());
			match kw {
				Keyword_::Code => {
					let ret = find_def__(i.s__().unwrap(), &mut 0, true, codes, data.clone(), path, data_mut, data2_mut, qu.clone(), dbg);
					if ret != Return_::Ok {return ret}
				}
				Keyword_::Var(_) => match val__(i, qu.clone(), data.clone(), path, data2_mut, dbg) {
					Ok(s) => data2_mut.ret.push(s),
					Err(ret2) => if ret2 != Return_::Ok {return ret2}
				}
				Keyword_::Block(_) => match for221(qu.clone(), data2, data2_mut, i.a__()) {
					Return_::Ok => {}
					ret@_ => return ret
				}
				Keyword_::For(_) => {
					let i = i.as_any().downcast_ref::<for_::Item_>().unwrap();
					let mut cnt = 1;
					let start = match for21_ne(qu.clone(), &i.start_) {
						Return_::Ok => 0,
						Return_::Ok2(v) => 
							if !v.is_empty() {
								let u = &v[0].s_;
								if let Ok(u) = u.parse::<isize>() {
									u - 1
								} else {return Return_::Err(["for start ", u].concat())}
							} else {0}
						ret => return ret
					};
					let max = match for21_ne(qu.clone(), &i.count_) {
						Return_::Ok => None,
						Return_::Ok2(v) => 
							if !v.is_empty() {
								let u = &v[0].s_;
								if let Ok(u) = u.parse::<usize>() {
									Some(u)
								} else {return Return_::Err(["for count ", u].concat())}
							} else {return Return_::err__("for count")}
						ret => return ret
					};
					let name = match for21_ne(qu.clone(), &i.name_) {
						Return_::Ok => None,
						Return_::Ok2(v) => if !v.is_empty() {Some(v[0].s_.clone())} else {None},
						ret => return ret
					};
					loop {
						if let Some(max) = max {
							if cnt > max {break}
						}
						if let Some(name) = &name {
							l_w__!(qu).val_to__(name.clone(), new_text__(if start > 0 {
								(cnt + start as usize).to_string()
							} else {
								(cnt as isize + start).to_string()
							}));
						}
						if max.is_some() || name.is_some() {
							cnt += 1;
						}

						let ret = for221(qu.clone(), &new_data2_2(), data2_mut, i.a__());
						match &ret {
							Return_::Ok => {}
							Return_::Break(name2) => {
								if let Some(name2) = name2 {
									if let Some(name) = &name {
										if name2 != name {
											return ret;
										}
										break;
									} else {
										return ret;
									}
								} else {
									break;
								}
							}
							Return_::Continue(name2) => {
								if let Some(name2) = name2 {
									if let Some(name) = &name {
										if name2 != name {
											return ret;
										}
									} else {
										return ret;
									}
								}
							}
							_ => return ret
						}
					}
				}
				Keyword_::Break(_) => return match for2(qu.clone()) {
					Return_::Ok => Return_::Break(None),
					Return_::Ok2(v) => Return_::Break(Some(if !v.is_empty() {&v[0].s_} else {""}.to_string())),
					ret => ret
				},
				Keyword_::Continue(_) => return match for2(qu.clone()) {
					Return_::Ok => Return_::Continue(None),
					Return_::Ok2(v) => Return_::Continue(Some(if !v.is_empty() {&v[0].s_} else {""}.to_string())),
					ret => ret
				},
				Keyword_::Switch(_) => match for2_ne(qu.clone()) {
					Return_::Ok2(v) => {
						let item = i.as_any().downcast_ref::<switch_::Item_>().unwrap();
						let op = item.op_.as_ref().unwrap();
						if let Some(codes2) = &item.case_ {
							let left = &v[0];
							let mut defa = vec![];
							let mut use_defa = true;
							for (idx, codes3) in codes2.iter().enumerate() {
								let codes2 = &codes3.a_;
								let len = codes2.len();
								if len >= 2 {
									let end = len - 1;
									let v = {
										let mut data2_mut2 = Data2Mut_::new();
										let ret = z__(&codes2, Some(end), data.clone(), data2, path, &mut DataMut_::new(), &mut data2_mut2, qu.clone(), dbg);
										if ret != Return_::Ok {return ret}
										match to_vec__(&data2_mut2.ret, qu.clone(), data.clone(), path, dbg) {
											Ok(v) => v,
											Err(ret) => return ret
										}
									};
									if match op[idx] {
										switch_::Op_::Eq => v.contains(left),
										switch_::Op_::Ne => !v.contains(left),
										_ => {
											let left = &left.s_;
											let right = &v[0].s_;
											match op[idx] {
												switch_::Op_::Le => if let Some((left, right)) = to_2d__(left, right) {left <= right} else {left <= right}
												switch_::Op_::Lt => if let Some((left, right)) = to_2d__(left, right) {left <  right} else {left <  right}
												switch_::Op_::Ge => if let Some((left, right)) = to_2d__(left, right) {left >= right} else {left >= right}
												switch_::Op_::Gt => if let Some((left, right)) = to_2d__(left, right) {left >  right} else {left >  right}
												_ => false
											}
										}
									} {
										use_defa = false;
										let ret2 = z__(&codes2, None, data.clone(), &&new_data2_2(), path, &mut DataMut_::new2(end), data2_mut, qu.clone(), dbg);
										if ret2 != Return_::Ok {return ret2}
									}
								}
								else if len == 1 {
									defa.push(idx);
								}
							}
							if use_defa {
								for idx in defa {
									let ret2 = z__(&codes2[idx].a_, None, data.clone(), data2, path, &mut DataMut_::new(), data2_mut, qu.clone(), dbg);
									if ret2 != Return_::Ok {return ret2}
								}
							}
						}
					}
					ret2 => return ret2
				}
				Keyword_::Set(_) => {
					let names = match for2(qu.clone()) {
						Return_::Ok2(v) => if v.is_empty() {vec![Text_::empty()]} else {v}
						ret => return ret
					};
					let vals = match for21(qu.clone(), &i.as_any().downcast_ref::<set_::Item_>().unwrap().vals_) {
						Return_::Ok2(v) => {
							let v = tv_to_civ__(v);
							if v.is_empty() {vec![new_text2__("")]} else {v}
						}
						ret => return ret
					};
					for (idx, name) in names.iter().enumerate() {
						l_w__!(qu).val_to__(name.s_.to_string(), vals[if idx >= vals.len() {vals.len() - 1} else {idx}].clone());
					}
				}
				Keyword_::Def(_) => {
					let mut names = match for2(qu.clone()) {
						Return_::Ok2(v) => if v.is_empty() {vec![Text_::empty()]} else {v}
						ret => return ret
					};
					let name1 = names.remove(0);
					let mut backarg = 0;
					let mut argc = core::usize::MAX;
					if let Some(rems) = name1.rems_ {
						let daogua = "倒挂";
						for i in rems {
							if i.starts_with(daogua) {
								let u = &i[daogua.len()..];
								if u.is_empty() {
									backarg += 1;
								} else if let Ok(u) = u.parse::<usize>() {
									backarg = u;
								} else {return Return_::Err2};
								continue
							}
							if i == "无参" {
								argc = 0;
								continue
							}
							return Return_::Err(["定义不识注解：", &i].concat());
						}
					}
					let name = name1.s_;
					if name.is_empty() {return Return_::err__("定义名不可空")}
					let code = match for21(qu.clone(), &i.as_any().downcast_ref::<def_::Item_>().unwrap().code_) {
						Return_::Ok2(v) => v,
						ret => return ret
					};
					l_w__!(qu).def_to__(name, Arc::new(names), backarg, argc, if code.is_empty() {""} else {&code[0].s_}.to_string());
				}
				Keyword_::Eval(_) | Keyword_::Load(_) => match for2(qu.clone()) {
					Return_::Ok2(mut v) => {
						let mut src = v.remove(0);
						if src.s_.is_empty() && !v.is_empty() {
							src = v.remove(0);
						}
						match hello4__(src, match kw {Keyword_::Load(_) => true, _ => false}, v, path, Some(data.clone()), Some(qu.clone()), dbg) {
							Return_::Ok => {}
							Return_::Ok2(v) => data2_mut.ret.append(&mut tv_to_civ__(v)),
							ret@_ => return ret
						}
					}
					Return_::Ok => {}
					ret@_ => return ret
				}
				Keyword_::Expl(_) => match for2(qu.clone()) {
					Return_::Ok2(v) => {
						let mut opa = vec![];
						{
							let mut s:Vec<char> = v[0].s_.chars().collect();
							s.push('\x00');
							let mut i = 0;
							let mut oldi = core::usize::MAX;
							let mut ret2 = Return_::Ok;
							loop {
								if i >= s.len() {break}
								let mut for2 = |op| {
									if oldi != core::usize::MAX {
										let s2 = &String::from_iter(&s[oldi..i]);
										if let Ok(d) = s2.parse::<f64>() {
											opa.push(expl_::Op_::Num(d));
										} else {
											ret2 = Return_::Err([s2, "非数字"].concat());
										}
										oldi = core::usize::MAX;
									}
									if let expl_::Op_::No = op {return}
									opa.push(op);
								};
								match s[i] {
									'+' | '加' => for2(expl_::Op_::Add),
									'-' | '减' => for2(expl_::Op_::Sub),
									'*' | '乘' => for2(expl_::Op_::Mul),
									'/' | '除' => for2(expl_::Op_::Div),
									'%' | '模' => for2(expl_::Op_::Mod),
									'^' | '方' => for2(expl_::Op_::Pow),
									'(' => for2(expl_::Op_::BeginG),
									')' => for2(expl_::Op_::EndG),
									'\x00' => for2(expl_::Op_::No),
									_ => if oldi == core::usize::MAX {oldi = i;}
								}
								if ret2 != Return_::Ok {return ret2}
								i += 1;
							}
						}
						if dbg.tree {
							pr__!("{:?}\n", opa);
						}
						match expl__(&opa) {
							Ok(d) => data2_mut.ret__(d.to_string()),
							Err(ret) => return ret
						}
					}
					Return_::Ok => {}
					ret@_ => return ret
				}
				Keyword_::Print(_) => match for2(qu.clone()) {
					Return_::Ok2(v) => {pr__!("{}", v[0].s_);}
					Return_::Ok => {}
					ret@_ => return ret
				}
				Keyword_::Juhao(_) => {
					if !data2.juhao2 {
						let len = data2_mut.ret.len();
						if len > 0 && !hao__(len - 1, &data2_mut.ret, false, true) {
							data2_mut.ret.push(i.clone());
						}
					}
					if data2.juhao {break}
				}
				_ => data2_mut.ret.push(i.clone())
			}
			data_mut.idx += 1;
		}
		Return_::Ok
	}
	
	fn to_2d__(s1:&str, s2:&str) -> Option<(f64, f64)> {
		if let Ok(d1) = s1.parse::<f64>() {
			if let Ok(d2) = s2.parse::<f64>() {
				return Some((d1, d2))
			}
		}
		None
	}
	
	fn to_vec__<P:PathImpl_>(v:&Vec<CI_>, qu:Q_, data:DA_, path:&P, dbg:&Debug_) -> Result<Vec<Text_>, Return_> {
		let mut ret = vec![];
		let mut buf = String::new();
		let mut push = 0;
		let clear_buf = |buf:&mut String, dunhao, push:&mut i32, ret:&mut Vec<Text_>| {
			if dunhao || *push != 0 {
				ret.push(Text_::new(buf.clone()));
				buf.clear();
				*push = 0;
			}
		};
		for i in v {
			let kw = &i.kw__()[0];
			let mut for2 = || {
				if let Some(s2) = i.s__() {
					buf.push_str(s2);
					push = 1;
				}
			};
			match kw {
				Keyword_::Dunhao(_) | Keyword_::Juhao(_) => {
					if push >= 0 {
						clear_buf(&mut buf, true, &mut push, &mut ret);
					}
					push = 2;
				}
				Keyword_::Text | Keyword_::Code | Keyword_::Lf(_) | Keyword_::Cr(_) | Keyword_::Esc(_) => for2(),
				Keyword_::Rem2(_) => {
					let v2 = z5__(i, qu.clone(), data.clone(), path, dbg)?;
					if ret.is_empty() {
						clear_buf(&mut buf, true, &mut push, &mut ret);
						push = -1;
					}
					let idx = ret.len() - 1;
					let i = &mut ret[idx];
					if i.rems_.is_none() {
						i.rems_ = Some(vec![]);
					}
					if let Some(rems) = &mut i.rems_ {
						for i2 in v2 {
							rems.push(i2.s_.to_string());
						}
					}
				}
				//Keyword_::Var(_) => buf.push_str(val__(i, qu)?),
				_ => {
					//panic!
					pr__!("err to_vec__ {:?}\n", kw); loop{}
				}
			}
		}
		clear_buf(&mut buf, false, &mut push, &mut ret);
		Ok(ret)
	}
	fn tv_to_civ__(v:Vec<Text_>) -> Vec<CI_> {v.into_iter().map(|i| new_ci__(Box::new(i))).collect()}
	
	fn hao__(idx:usize, codes:&Vec<CI_>, dun:bool, ju:bool) -> bool {
		if idx < codes.len() {
			match codes[idx].kw__()[0] {
				Keyword_::Dunhao(_) => dun,
				Keyword_::Juhao(_) => ju,
				_ => false
			}
		} else {false}
	}
	
	fn z5__<P:PathImpl_>(i:&CI_, qu:Q_, data:DA_, path:&P, dbg:&Debug_) -> Result<Vec<Text_>, Return_> {
		if let Some(codes2) = i.a__() {
			let mut data2_mut2 = Data2Mut_::new();
			let ret2 = z__(&codes2.a_, None, Arc::new(Data_::new(Some(data.clone()))), &Default::default(), path, &mut DataMut_::new(), &mut data2_mut2, qu.clone(), dbg);
			if ret2 != Return_::Ok {return Err(ret2);}
			to_vec__(&data2_mut2.ret, qu.clone(), data.clone(), path, dbg)
		} else {return Err(Return_::Err2)}
	}

	fn val__<P:PathImpl_>(i:&CI_, qu:Q_, data:DA_, path:&P, data2_mut:&mut Data2Mut_, dbg:&Debug_) -> Result<CI_, Return_> {
		let v = z5__(i, qu.clone(), data.clone(), path, dbg)?;
		let name0 = Text_::empty();
		let name1 = if v.is_empty() {&name0} else {&v[0]};
		let name = &name1.s_;
		let err = || {Err(Return_::Err(["缺变量：", name].concat()))};
		if let Some(def) = &data.by_def {
			for (idx, name2) in l_r__!(def).argnames_.iter().enumerate() {
				if name2 == name {
					return Ok(if idx < data.args.len() {
						data.args[idx].clone()
					} else {new_text2__("")})
				}
			}
		}
		let mut for2 = |stack| {
			let mut idx2 = 0;
			let end2 = if let Some(rems) = &name1.rems_ {
				if rems.len() > 0 {
					if let Ok(u) = rems[0].parse::<usize>() {
						if u > 0 {
							idx2 = u - 1;
						}
					}
				}
				if rems.len() > 1 {
					if let Ok(u) = rems[1].parse::<usize>() {
						Some(u)
					} else {None}
				} else {None}
			} else {None};
			
			let args = &data.args;
			let first = idx2;
			if stack {
				while idx2 < args.len() {
					if let Some(u) = end2 {if idx2 >= u {break}}
					if idx2 > first {
						data2_mut.ret.push(new_dunhao__());
					}
					data2_mut.ret.push(args[idx2].clone());
					idx2 += 1;
				}
				Err(Return_::Ok)
			} else {
				let mut s = String::new();
				while idx2 < args.len() {
					if let Some(u) = end2 {if idx2 >= u {break}}
					let i2 = &args[idx2];
					if idx2 > first {s += " ";}
					if let Some(s2) = i2.s__() {
						s += "\"";
						s += &s2.replace(r#"""#, r#"\""#);
						s += "\"";
					}
					idx2 += 1;
				}
				Ok(new_text__(s))
			}
		};
		match name.as_ref() {
			"参数栈" => return for2(true),
			"参数" => return for2(false),
			"参数0" => return 
			if let Some(def) = &data.by_def {
				Ok(new_text__(l_r__!(def).name_.clone()))
			} else {
				Ok(new_text__(data.src.s_.clone()))
			},
			_ => return if let Some(val) = l_r__!(qu).val__(name) {
					Ok(val.s_.clone())
				} else {err()}
		}
	}

	fn find_def__<P:PathImpl_>(s:&str, idx:&mut usize, ok_par:bool, codes:&Vec<CI_>, data:DA_, path:&P, data_mut:&mut DataMut_, data2_mut:&mut Data2Mut_, qu:Q_, dbg:&Debug_) -> Return_ {
		let mut old = core::usize::MAX;
		let clear = |old:&mut usize, idx, data2_mut:&mut Data2Mut_| {
			if *old != core::usize::MAX {
				if *old <  s.len() {
					data2_mut.ret__(s[*old..idx].to_string());
				}
				*old = core::usize::MAX;
			}
		};
		while *idx < s.len() {
			if s.is_char_boundary(*idx) {
				if let Some((def, len)) = Qu_::find_def__(qu.clone(), s, *idx) {
					if dbg.def {
						pr__!("{:?}\n", def);
					}
					if dbg.src {
						line__(s);
					}
					clear(&mut old, *idx, data2_mut);
					*idx += len;
					let mut jiucaihezi = Data_::new(Some(data.clone()));
					if l_r__!(def).argc_ > 0 {
						data_mut.idx += 1;
						let mut data2_mut3 = Data2Mut_::new();
						let ret = find_def__(s, idx, false, codes, data.clone(), path, data_mut, &mut data2_mut3, qu.clone(), dbg);
						if ret != Return_::Ok {return ret}
						let mut data2_mut2 = Data2Mut_::new();
						match to_vec__(&data2_mut3.ret, qu.clone(), data.clone(), path, dbg) {
							Ok(mut v) => {
								{
									let ba = l_r__!(def).backarg_;
									let mut len = 0;
									let src = &mut data2_mut.ret;
									let mut from = src.len();
									while from > 0 {
										from -= 1;
										if hao__(from, src, true, true) {
											len += 1;
											if len >= ba {
												from += 1;
												break;
											}
										} else if from == 0 {
											len += 1;
										}
									}
									if len >= ba {
										for _ in from..src.len() {
											jiucaihezi.args.insert(0, src.pop().unwrap());
										}
									} else {return Return_::Err([&len.to_string(), "不足以倒挂", &ba.to_string()].concat())}
								}
								{
									let s2 = &s[*idx..];
									if !s2.is_empty() {
										let len = v.len();
										if len == 0 {
											jiucaihezi.args.push(new_text2__(s2));
										} else {
											v[len - 1].s_.insert_str(0, s2);
										}
									}
								}
								jiucaihezi.args.append(&mut tv_to_civ__(v));
								if hao__(data_mut.idx, codes, true, false) {
									data_mut.idx += 1;
								}
							}
							Err(ret) => return ret
						}
						let ret = z__(codes, None, data.clone(), &Data2_ {juhao:true, juhao2:false, ..Default::default()}, path, data_mut, &mut data2_mut2, qu.clone(), dbg);
						if ret != Return_::Ok {return ret}
						match to_vec__(&data2_mut2.ret, qu.clone(), data.clone(), path, dbg) {
							Ok(v) => jiucaihezi.args.append(&mut tv_to_civ__(v)),
							Err(ret) => return ret
						}
					}
					jiucaihezi.by_def = Some(def.clone());
					let ret = z4__(jiucaihezi, path, Some(qu.clone()), dbg);
					match ret {
						Return_::Ok => {}
						Return_::Ok2(v) => data2_mut.ret.append(&mut  tv_to_civ__(v)),
						_ => return ret
					}
					if l_r__!(def).argc_ == 0 {continue}
					break;
				} else if !ok_par {
					break;
				}
				if old == core::usize::MAX {old = *idx;}
			}
			*idx += 1;
		}
		clear(&mut old, *idx, data2_mut);
		Return_::Ok
	}

	type ExplRes_ = Result<f64, Return_>;
	fn expl__(opa:&Vec<expl_::Op_>) -> ExplRes_ {
		expl_2__(opa, &mut 0)
	}
	fn expl_2__(opa:&Vec<expl_::Op_>, idx:&mut usize) -> ExplRes_ {
		let mut ret = expl_3__(opa, idx)?;
		while *idx < opa.len() {
			match opa[*idx] {
				expl_::Op_::Add => {*idx += 1; ret += expl_3__(opa, idx)?;}
				expl_::Op_::Sub => {*idx += 1; ret -= expl_3__(opa, idx)?;}
				_ => break
			}
		}
		Ok(ret)
	}
	fn expl_3__(opa:&Vec<expl_::Op_>, idx:&mut usize) -> ExplRes_ {
		let mut ret = expl_4__(opa, idx)?;
		while *idx < opa.len() {
			match opa[*idx] {
				expl_::Op_::Mul => {*idx += 1; ret *= expl_4__(opa, idx)?;}
				expl_::Op_::Div => {*idx += 1; ret /= expl_4__(opa, idx)?;}
				expl_::Op_::Mod => {*idx += 1; ret %= expl_4__(opa, idx)?;}
				_ => break
			}
		}
		Ok(ret)
	}
	fn expl_4__(opa:&Vec<expl_::Op_>, idx:&mut usize) -> ExplRes_ {
		let mut ret = expl_5__(opa, idx)?;
		while *idx < opa.len() {
			match opa[*idx] {
				expl_::Op_::Pow => {*idx += 1; /*ret = ret.powf();*/ 
					let mut d2 = expl_5__(opa, idx)?;
					let d1 = ret;
					while d2 > 1. {
						ret *= d1;
						d2 -= 1.;
					}
				}
				_ => break
			}
		}
		Ok(ret)
	}
	fn expl_5__(opa:&Vec<expl_::Op_>, idx:&mut usize) -> ExplRes_ {
		match opa[*idx] {
			expl_::Op_::Num(d) => {*idx += 1; Ok(d)}
			expl_::Op_::BeginG => {*idx += 1; let ret = expl_2__(opa, idx)?; *idx += 1; Ok(ret)}
			_ => Ok(0.)
		}
	}

	#[derive(Debug, PartialEq)]
	pub enum Return_ {
		Ok,
		Ok2(Vec<Text_>),
		Break(Option<String>),
		Continue(Option<String>),
		Exit(i32),
		Err(String),
		Err2
	}
	#[derive(Default)]
	pub struct Debug_ {
		pub tree:bool,
		pub src:bool,
		pub args:bool,
		pub def:bool,
	}
	type DA_ = Arc<Data_>;
	pub struct Data_ {
		args:Vec<CI_>,
		by_def:Option<D_>,
		src:Text_,
		path:Option<String>,
		up_:Option<DA_>,
	}
	#[derive(Default)]
	struct Data2_ {
		juhao:bool,
		juhao2:bool,
	}
	struct DataMut_ {
		idx:usize,
	}
	struct Data2Mut_ {
		ret:Vec<CI_>,
	}
	impl Return_ {
		fn err__(s:&str) -> Self {Self::Err(s.to_string())}
	}
	impl Data_ {
		fn new(up:Option<DA_>) -> Self {Self::new2(vec![], up)}
		fn new2(args:Vec<CI_>, up_:Option<DA_>) -> Self {Self {args, by_def:None, src:Text_::empty(), path:None, up_}}
	}
	impl DataMut_ {
		fn new() -> Self {Self::new2(0)}
		fn new2(idx:usize) -> Self {Self {idx}}
	}
	impl Data2Mut_ {
		fn new() -> Self {Self {ret:vec![]}}
		fn ret__(&mut self, s:String) {
			self.ret.push(new_text__(s));
		}
	}
	
	type D_ = T_<Defv_>;
	type Q_ = T_<Qu_>;
	pub struct Qu_ {
		vals_:Vec<Val_>,
		defs_:Vec<D_>,
		up_:Option<Q_>,
	}
	impl Qu_ {
		fn new(up_:Option<Q_>) -> Self {Self {vals_:vec![], defs_:vec![], up_}}
		fn val__(&self, name:&str) -> Option<&Val_> {
			if let Some(idx) = self.vals_.iter().position(|i| i.name_ == name) {
				Some(&self.vals_[idx])
			} else {None}
		}
		fn val_to__(&mut self, name_:String, s_:CI_) {
			if let Some(idx) = self.vals_.iter().position(|i| i.name_ == name_) {
				self.vals_[idx].s_ = s_;
			} else {
				self.vals_.push(Val_ {name_, s_});
			}
		}
		fn def_to__(&mut self, name_:String, argnames_:Argnames_, backarg_:usize, argc_:usize, code_:String) {
			if let Some(idx) = self.defs_.iter().position(|i| l_r__!(i).name_ == name_) {
				let mut i = l_w__!(self.defs_[idx]);
				i.argnames_ = argnames_;
				i.backarg_ = backarg_;
				i.argc_ = argc_;
				i.code_ = code_;
			} else {
				self.defs_.push(new_t__(Defv_ {name_, argnames_, backarg_, argc_, code_}));
			}
		}
		fn find_def__(qu:Q_, s:&str, from:usize) -> Option<(D_, usize)> {
			let s = &s[from..];
			Self::for__(qu, |sel:&Qu_| {
				for i in &sel.defs_ {
					let name = &l_r__!(i).name_;
					if s.starts_with(name) {
						return Some((i.clone(), name.len()));
					}
				}
				None
			})
		}
		fn for__<T>(qu:Q_, f:impl Fn(&Qu_) -> Option<T>) -> Option<T> {
			let mut qu = Some(qu);
			while qu.is_some() {
				let sel = qu.unwrap();
				let ret = f(&l_r__!(sel));
				if ret.is_some() {
					return ret
				}
				qu = l_r__!(sel).up_.clone();
			}
			None
		}
	}

	pub trait PathImpl_ {
		fn open__<'a, 'b>(&self, _src:&'b str, _src2:&'a str) -> Option<String> {None}
	}
	pub struct Path_ {}
	impl PathImpl_ for Path_ {}
}

mod pars_ {
	#[cfg(not(feature = "no"))]
	use alloc::{string::String, vec, vec::Vec, boxed::Box, };
	use super::{*};

	pub fn hello__(s:&str, codes:&mut Vec<CI_>) -> Return_ {
		let begin_rem = new_ci__(Box::new(begin_rem_::Item_{}));
		let end_rem = new_ci__(Box::new(end_rem_::Item_{}));
		let begin_text = new_ci__(Box::new(begin_text_::Item_{}));
		let end_text = new_ci__(Box::new(end_text_::Item_{}));
		let begin_text2 = new_ci__(Box::new(begin_text2_::Item_{}));
		let end_text2 = new_ci__(Box::new(end_text2_::Item_{}));
		let begin_code = new_ci__(Box::new(begin_code_::Item_{}));
		let end_code = new_ci__(Box::new(end_code_::Item_{}));
		let begin_code2 = new_ci__(Box::new(begin_code2_::Item_{}));
		let end_code2 = new_ci__(Box::new(end_code2_::Item_{}));
		let var = new_ci__(Box::new(var_::Item_{a_:None, }));
		z__(&Data_ {
			kws:vec![
				new_juhao__(),
				new_dunhao__(),
				begin_rem.clone(),
				end_rem.clone(),
				begin_text.clone(),
				end_text.clone(),
				begin_text2.clone(),
				end_text2.clone(),
				begin_code.clone(),
				end_code.clone(),
				begin_code2.clone(),
				end_code2.clone(),
				new_ci__(Box::new(rem2_::Item_{a_:None, })),
				var.clone(),
				new_ci__(Box::new(block_::Item_{a_:None, })),
				new_ci__(Box::new(for_::Item_{a_:None, count_:None, start_:None, name_:None})),
				new_ci__(Box::new(break_::Item_{a_:None, })),
				new_ci__(Box::new(continue_::Item_{a_:None, })),
				new_ci__(Box::new(switch_::Item_{a_:None, case_:None, op_:None})),
				new_ci__(Box::new(set_::Item_{names_:None, vals_:None})),
				new_ci__(Box::new(def_::Item_{names_:None, code_:None})),
				new_ci__(Box::new(print_::Item_{a_:None, })),
				new_ci__(Box::new(expl_::Item_{a_:None, })),
				new_ci__(Box::new(eval_::Item_{a_:None, })),
				new_ci__(Box::new(load_::Item_{a_:None, })),
				new_ci__(Box::new(par_brkpoint_::Item_{})),
				new_ci__(Box::new(lf_::Item_{})),
				new_ci__(Box::new(cr_::Item_{})),
				new_ci__(Box::new(esc_::Item_{})),
			],
			kws_rem:vec![begin_rem.clone(), end_rem.clone()],
			kws_text:vec![begin_text.clone(), end_text.clone(), var.clone()],
			kws_text2:vec![begin_text2.clone(), end_text2.clone()],
			kws_code:vec![begin_code.clone(), end_code.clone(), begin_rem.clone(), end_rem.clone()],
			kws_code2:vec![begin_code2.clone(), end_code2.clone()],
			cs:s.chars().collect()
		},
		&mut DataMut_ {idx:0, is_rem:0, is_text:0, is_text2:0, is_code:0, is_code2:0, is_var:0},
		Data2_ {juhao:false}, &From_::Indiff, codes)
	}
	fn z__(data:&Data_, data_mut:&mut DataMut_, data2:Data2_, from:&From_, codes:&mut Vec<CI_>) -> Return_ {
		let mut buf = String::new();
		let clear_buf = |buf:&mut String, data_mut:&DataMut_, codes:&mut Vec<CI_>| {
			if !buf.is_empty() {
				codes.push(
					if data_mut.as_text__() {
						new_text__(buf.clone())
					} else {
						new_ci__(Box::new(Code_ {s_:buf.clone()}))
					}
				);
				buf.clear();
			}
		};
		let mut ret = Return_::Ok;
		'l1: loop {
			if data_mut.idx >= data.cs.len() {break}
			let i = data.cs[data_mut.idx];
			for kw2 in
				if data_mut.is_rem != 0 {&data.kws_rem}
				else if data_mut.is_text != 0 {&data.kws_text}
				else if data_mut.is_text2 != 0 {&data.kws_text2}
				else if data_mut.is_code != 0 {&data.kws_code}
				else if data_mut.is_code2 != 0 {&data.kws_code2}
				else {&data.kws}
			{
				for kw in &kw2.kw__() {
					let mut is_th = false;
					let mut idx2 = 0;
					let mut kw_i = 0;
					let mut for22 = |a:&Vec<Vec<char>>, from2, from1, juhao, subnum, canot_empty:Option<&str>| -> Option<(Return_, Option<Codes_>, Option<Codes_>)> {
						kw_i = 0;
						loop {
							if kw_i >= a.len() {break}
							let a = &a[kw_i];
							is_th = true;
							idx2 = data_mut.idx;
							'l2: for i2 in a {
								loop {
									let i3 = data.cs[idx2];
									if i3 <= ' ' {
										if i3 == ' ' {
											let i = 0;
										}
										if data_mut.as_text__() {
											is_th = false;
											break 'l2;
										}
										idx2 += 1;
										if idx2 >= data.cs.len() {
											is_th = false;
											break 'l2;
										}
										continue;
									}
									if i2 != &i3 {
										is_th = false;
										break 'l2;
									}
									idx2 += 1;
									break;
								}
							}
							if is_th {
								data_mut.idx = idx2;
								
								let mut can_add = false;
								let mut need_c = false;
								match from2 {
									From2_::BeginText => {
										can_add = data_mut.is_text > 0;
										need_c = data_mut.is_text == 0;
									}
									From2_::EndText => {
										can_add = data_mut.is_text > 1;
										need_c = data_mut.is_text == 1;
									}
									From2_::BeginCode => {
										can_add = data_mut.is_code > 0;
										need_c = data_mut.is_code == 0;
									}
									From2_::EndCode => {
										can_add = data_mut.is_code > 1;
										need_c = data_mut.is_code == 1;
									}
									_ => {}
								}
								if need_c || subnum > 0 {
									clear_buf(&mut buf, data_mut, codes);
								}
								if can_add {
									for i2 in a {
										buf.push(*i2);
									}
								}
								
								match kw {
									Keyword_::Var(_) => {
										if kw_i == 0 {
											data_mut.is_var += 1;
										} else {
											data_mut.is_var -= 1;
										}
									}
									_ => {}
								}
								if kw_i == 0 && subnum >= 1 {
									let mut codes2 = Codes_ {a_:vec![]};
									let mut ret2 = z__(data, data_mut, Data2_ {juhao}, &from1, &mut codes2.a_);
									if let Some(ret) = canot_empty {
										if codes2.a_.is_empty() {
											ret2 = Return_::Err(format!("{:?}{}不可空", kw, ret))
										}
									}
									match from1 {
										From_::Set | From_::Def => {
											if ret2 != Return_::To {
												ret2 = Return_::Err(format!("{:?}缺后句", kw));
											} else {
												ret2 = Return_::Continue;
											}
										}
										From_::For => {
											if ret2 != Return_::Juhao {
												ret2 = Return_::Continue;
											}
										}
										_ =>
										match ret2 {
											Return_::Ok | Return_::Juhao => ret2 = Return_::Continue,
											_ => {}
										}
									}
									match ret2 {
										Return_::Err(_) => return Some((ret2, None, None)),
										_ => {}
									}
									if subnum == 1 {
										return Some((ret2, Some(codes2), None));
									}
									let mut codes3 = Codes_ {a_:vec![]};
									let mut ret2 = z__(data, data_mut, Data2_ {juhao:true}, &From_::Indiff, &mut codes3.a_);
									match ret2 {
										Return_::Ok | Return_::Juhao => ret2 = Return_::Continue,
										_ => {}
									}
									return Some((ret2, Some(codes2), Some(codes3)));
								}
								return Some((Return_::Ok, None, None));
							}
							kw_i += 1;
						}
						None
					};
					let mut for2 = |a, from2, from1, juhao, subnum| for22(a, from2, from1, juhao, subnum, None);
					let mut for2_0 = |a| for2(a, From2_::Indiff, From_::Indiff, false, 0);
					let mut ret2 = Return_::Ok;
					match kw {
						Keyword_::Juhao(a) => if for2_0(a).is_some() {
							if data2.juhao {
								ret2 = Return_::Juhao
							}
						}
						Keyword_::Dunhao(a) | Keyword_::Lf(a) | Keyword_::Cr(a) | Keyword_::Esc(a) => if for2_0(a).is_some() {}
						Keyword_::BeginRem(a) => if for2(a, From2_::BeginRem, From_::Indiff, false, 0).is_some() {data_mut.is_rem += 1; continue 'l1}
						Keyword_::EndRem(a) => if for2(a, From2_::EndRem, From_::Indiff, false, 0).is_some() {data_mut.is_rem -= 1; continue 'l1}
						Keyword_::BeginText(a) => if for2(a, From2_::BeginText, From_::Indiff, false, 0).is_some() {data_mut.is_text += 1; continue 'l1}
						Keyword_::EndText(a) => if for2(a, From2_::EndText, From_::Indiff, false, 0).is_some() {data_mut.is_text -= 1; continue 'l1}
						Keyword_::BeginText2(a) => if for2(a, From2_::BeginText2, From_::Indiff, false, 0).is_some() {data_mut.is_text2 += 1; continue 'l1}
						Keyword_::EndText2(a) => if for2(a, From2_::EndText2, From_::Indiff, false, 0).is_some() {data_mut.is_text2 -= 1; continue 'l1}
						Keyword_::BeginCode(a) => if for2(a, From2_::BeginCode, From_::Indiff, false, 0).is_some() {data_mut.is_code += 1; continue 'l1}
						Keyword_::EndCode(a) => if for2(a, From2_::EndCode, From_::Indiff, false, 0).is_some() {data_mut.is_code -= 1; continue 'l1}
						Keyword_::BeginCode2(a) => if for2(a, From2_::BeginCode2, From_::Indiff, false, 0).is_some() {data_mut.is_code2 += 1; continue 'l1}
						Keyword_::EndCode2(a) => if for2(a, From2_::EndCode2, From_::Indiff, false, 0).is_some() {data_mut.is_code2 -= 1; continue 'l1}
						Keyword_::Rem2(a) => if let Some((ret, a_, None)) = for2(a, From2_::Indiff, From_::Rem2, false, 1) {
							match ret {
								Return_::EndRem2 => {
									codes.push(new_ci__(Box::new(rem2_::Item_{a_})));
									ret2 = Return_::Continue
								}
								Return_::Ok => ret2 = Return_::EndRem2,
								_ => ret2 = ret
							}
						}
						Keyword_::Var(a) => if let Some((ret, a_, None)) = for2(a, From2_::Indiff, From_::Var, false, 1) {
							match ret {
								Return_::EndVar => {
									codes.push(new_ci__(Box::new(var_::Item_{a_})));
									ret2 = Return_::Continue
								}
								Return_::Ok => {
									ret2 = Return_::EndVar
								}
								_ => ret2 = ret
							}
						}
						Keyword_::Block(a) => if let Some((ret, a_, None)) = for2(a, From2_::Indiff, From_::Block, false, 1) {
							match ret {
								Return_::EndBlock => {
									codes.push(new_ci__(Box::new(block_::Item_{a_})));
									ret2 = match from {
										From_::For | From_::Switch => Return_::Block,
										_ => Return_::Continue
									}
								}
								Return_::Ok => ret2 = Return_::EndBlock,
								_ => ret2 = ret
							}
						}
						Keyword_::For(a) => if let Some((ret, Some(mut codes2), None)) = for2(a, From2_::Indiff, From_::For, true, 1) {
							let mut count_ = None;
							let mut start_ = None;
							let mut name_ = None;
							let mut idxs = vec![];
							for (idx, i) in codes2.a_.iter().enumerate() {
								if let Keyword_::Rem2(_) = i.kw__()[0] {
									idxs.push(idx);
								}
							}
							let mut idx = 0;
							for i in idxs.iter().rev() {
								let i2 = codes2.a_.remove(*i);
								let i2 = Some(i2.mv_a__().unwrap());
								match idxs.len() {
									1 => match idx {
										0 => if let Some(i3) = &i2 {
												let a = &i3.a_;
												if a.len() == 1 {
													let i3 = &a[0];
													match i3.kw__()[0] {
														Keyword_::Text | Keyword_::Code => {
															if i3.s__().unwrap().parse::<usize>().is_ok() {
																count_ = i2
															} else {name_ = i2}
														}
														_ => name_ = i2
													}
												} else {name_ = i2}
											} else {name_ = i2}
										_ => {}
									}
									2 => match idx {
										1 => count_ = i2,
										0 => name_ = i2,
										_ => {}
									}
									3 => match idx {
										2 => start_ = i2,
										1 => count_ = i2,
										0 => name_ = i2,
										_ => {}
									}
									_ => {}
								}
								idx += 1;
							}
							codes.push(new_ci__(Box::new(for_::Item_{a_:Some(codes2), count_, start_, name_})));
							ret2 = if ret == Return_::Block {Return_::Continue}
							else if ret == Return_::Juhao {
								codes.push(new_juhao__());
								Return_::Continue
							}
							else {ret}
						}
						Keyword_::Break(a) |
						Keyword_::Continue(a) => if let Some((ret, a_, None)) = for2(a, From2_::Indiff, From_::Indiff, true, 1) {
							codes.push(new_ci__(match kw {
								Keyword_::Continue(_) => Box::new(continue_::Item_{a_}),
								_ => Box::new(break_::Item_{a_}),
							}));
							ret2 = ret
						}
						Keyword_::Expl(a) |
						Keyword_::Eval(a) |
						Keyword_::Load(a) |
						Keyword_::Print(a) => if let Some((ret, a_, None)) = for22(a, From2_::Indiff, From_::Indiff, true, 1, Some("体")) {
							codes.push(new_ci__(match kw {
								Keyword_::Expl(_) => Box::new(expl_::Item_{a_}),
								Keyword_::Eval(_) => Box::new(eval_::Item_{a_}),
								Keyword_::Load(_) => Box::new(load_::Item_{a_}),
								_ => Box::new(print_::Item_{a_})
							}));
							ret2 = ret
						}
						Keyword_::Switch(a) => if let Some((ret, Some(mut codes2), None)) = for2(a, From2_::Indiff, From_::Switch, false, 1) {
							let body = codes2.a_.pop();
							let mut op = vec![];
							let case_ = if let Some(codes) = body.unwrap().a__() {
								let mut case = vec![];
								let mut case_i = 0;
								let mut case_i2 = 0;
								for (idx, i) in codes.a_.iter().enumerate() {
									let kw = &i.kw__()[0];
									if idx == 0
									|| if let Keyword_::Juhao(_) = kw {true} else {false}
									{
										case_i = case.len();
										case.push(Codes_ {a_:vec![]});
										case_i2 = 0;
										if idx > 0 {
											continue;
										}
									}
									case_i2 += 1;
									if case_i2 == 1 {
										match kw {
											Keyword_::Code => {
												if let Some(s) = i.s__() {
													let mut z = |e, s2:&str| {
														if s.starts_with(s2) {
															op.push(e);
															case[case_i].a_.push(new_ci__(Box::new(Code_ {s_:s[s2.len()..].to_string()})));
															true
														} else {false}
													};
													if z(switch_::Op_::Ne, "不等于") {continue}
													if z(switch_::Op_::Le, "小于等于") {continue}
													if z(switch_::Op_::Lt, "小于") {continue}
													if z(switch_::Op_::Ge, "大于等于") {continue}
													if z(switch_::Op_::Gt, "大于") {continue}
												}
											}
											_ => {}
										}
										op.push(switch_::Op_::Eq);
									}
									case[case_i].a_.push(i.clone());
								}
								Some(case)
							} else {None};
							codes.push(new_ci__(Box::new(switch_::Item_{a_:Some(codes2), case_, op_:Some(op)})));
							ret2 = if ret == Return_::Block {Return_::Continue} else {ret}
						}
						Keyword_::Set(a) |
						Keyword_::Def(a) => if let Some((ret, names_, vals_)) = for2(a, From2_::Indiff, match kw {
									Keyword_::Def(_) => From_::Def,
									_ => From_::Set
								}, false, 2) {
							match ret {
								Return_::Continue => {
									codes.push(new_ci__(match kw {
										Keyword_::Def(_) => Box::new(def_::Item_{names_, code_:vals_}),
										_ => Box::new(set_::Item_{names_, vals_})
									}));
									ret2 = ret;
								}
								Return_::Ok => ret2 = Return_::To,
								_ => ret2 = ret
							}
						}
						Keyword_::ParBrkpoint(a) => if for2(a, From2_::Indiff, From_::Indiff, false, 0).is_some() {
							#[allow(non_snake_case)] #[allow(unused_variables)] let o_H_o = true;
						}
						Keyword_::Text | Keyword_::Code | Keyword_::Val | Keyword_::Defv => {}
					}
					match ret2 {
						Return_::Ok => {}
						Return_::Continue => continue 'l1,
						_ => {
							ret = ret2;
							break 'l1;
						}
					}
					if is_th {
						clear_buf(&mut buf, data_mut, codes);
						codes.push(kw2.clone());
						continue 'l1;
					}
				}
			}
			if i == ' ' {
				let i = 0;
			}
			if data_mut.as_text_2__() || data_mut.is_rem == 0 && (i > ' ' || data_mut.as_text_1__() && i >= ' ') {
				buf.push(i);
			}
			data_mut.idx += 1;
		}
		clear_buf(&mut buf, data_mut, codes);
		ret
	}

	#[derive(Debug, PartialEq)]
	pub enum Return_ {
		Ok,
		Continue,
		Juhao,
		EndRem2,
		EndVar,
		EndBlock,
		Block,
		To,
		Err(String)
	}
	#[derive(Debug)]
	enum From_ {
		Indiff,
		Rem2,
		Var,
		Block,
		For,
		Switch,
		Set,
		Def,
	}
	#[derive(Debug)]
	enum From2_ {
		Indiff,
		BeginRem,
		EndRem,
		BeginText,
		EndText,
		BeginText2,
		EndText2,
		BeginCode,
		EndCode,
		BeginCode2,
		EndCode2,
	}
	struct Data_ {
		kws:Vec<CI_>,
		kws_rem:Vec<CI_>,
		kws_text:Vec<CI_>,
		kws_text2:Vec<CI_>,
		kws_code:Vec<CI_>,
		kws_code2:Vec<CI_>,
		cs:Vec<char>,
	}
	struct Data2_ {
		juhao:bool,
	}
	struct DataMut_ {
		idx:usize,
		is_rem:i32,
		is_text:i32,
		is_text2:i32,
		is_code:i32,
		is_code2:i32,
		is_var:i32,
	}
	impl DataMut_ {
		fn as_text_1__(&self) -> bool {self.is_text > 0 || self.is_code > 0 || self.is_var > 0}
		fn as_text_2__(&self) -> bool {self.is_text2 > 0 || self.is_code2 > 0}
		fn as_text__(&self) -> bool {self.as_text_1__() || self.as_text_2__()}
	}
}

#[derive(Debug)]
pub struct Text_ {
	pub s_:String,
	pub rems_:Option<Vec<String>>,
}
impl Text_ {
	pub fn new(s:String) -> Self {Self::new2(s, None)}
	pub fn new2(s_:String, rems_:Option<Vec<String>>) -> Self {Self {s_, rems_}}
	pub fn new3(s:&str) -> Self {Self::new(s.to_string())}
	pub fn empty() -> Self {Self::new3("")}
}
fn new_text__(s:String) -> CI_ {new_ci__(Box::new(Text_::new(s)))}
fn new_text2__(s:&str) -> CI_ {new_text__(s.to_string())}
impl CodeImpl_ for Text_ {
	fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Text]}
	fn s__(&self) -> Option<&str> {Some(&self.s_)}
	fn as_any(&self) -> &dyn Any {self}
}
impl PartialEq for Text_ {
	fn eq(&self, other: &Self) -> bool {self.s_ == other.s_}
}
impl PartialEq<String> for Text_ {
	fn eq(&self, other: &String) -> bool {&self.s_ == other}
}

struct Code_ {s_:String}
impl CodeImpl_ for Code_ {
	fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Code]}
	fn s__(&self) -> Option<&str> {Some(&self.s_)}
	fn as_any(&self) -> &dyn Any {self}
}

struct Val_ {
	s_:CI_,
	name_:String,
}
impl CodeImpl_ for Val_ {
	fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Val]}
	fn s__(&self) -> Option<&str> {Some(&self.name_)}
	fn as_any(&self) -> &dyn Any {self}
}

type Argnames_ = Arc<Vec<Text_>>;
#[derive(Debug)]
struct Defv_ {
	name_:String,
	argnames_:Argnames_,
	backarg_:usize,
	argc_:usize,
	code_:String,
}
impl CodeImpl_ for Defv_ {
	fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Defv]}
	fn s__(&self) -> Option<&str> {Some(&self.name_)}
	fn as_any(&self) -> &dyn Any {self}
}

fn new_juhao__() -> CI_ {new_ci__(Box::new(juhao_::Item_{}))}
fn new_dunhao__() -> CI_ {new_ci__(Box::new(dunhao_::Item_{}))}

fn tree__(codes:&Vec<CI_>) {
	tree2__(codes, -1, None, false)
}
fn tree2__(codes:&Vec<CI_>, mut suojin:i32, tag:Option<&str>, mut eoe0:bool) {
	suojin += 1;
	for (idx, i) in codes.iter().enumerate() {
		let eoe = idx == codes.len() - 1;
		pr__!("{}", match suojin {
			0 if codes.len() == 1 => {eoe0 = true; "< "}
			0 if eoe => {eoe0 = true; r"\ "}
			0 if idx == 0 => "/ ",
			_ => if eoe0 {"  "} else {"| "},
		});
		for _ in 0..suojin {pr__!("  ");}
		if let Some(tag) = tag {
			pr__!("*{} ", tag);
		}

		let kw = &i.kw__()[0];
		match kw {
			Keyword_::Text | Keyword_::Code => {
				if let Some(s) = i.s__() {
					pr__!("{:?} ", kw);
					line__(s);
				}
			}
			_ => {
				pr__!("{:?}\n", kw);
				loop {
					if let Some(i) = i.as_any().downcast_ref::<for_::Item_>() {
						if let Some(codes) = &i.start_ {
							tree2__(&codes.a_, suojin, Some("start"), eoe0);
						}
						if let Some(codes) = &i.count_ {
							tree2__(&codes.a_, suojin, Some("count"), eoe0);
						}
						if let Some(codes) = &i.name_ {
							tree2__(&codes.a_, suojin, Some("name"), eoe0);
						}
						break;
					}
					break;
				}
				if let Some(codes) = i.a__() {
					tree2__(&codes.a_, suojin, i.a_tag__(), eoe0);
				}
				loop {
					if let Some(i) = i.as_any().downcast_ref::<set_::Item_>() {
						if let Some(codes) = &i.vals_ {
							tree2__(&codes.a_, suojin, None, eoe0);
						}
						break;
					}
					if let Some(i) = i.as_any().downcast_ref::<def_::Item_>() {
						if let Some(codes) = &i.code_ {
							tree2__(&codes.a_, suojin, None, eoe0);
						}
						break;
					}
					if let Some(i) = i.as_any().downcast_ref::<switch_::Item_>() {
						if let Some(codes) = &i.case_ {
							for codes in codes {
								pr__!("{}\n", if eoe0 {""} else {"|"});
								tree2__(&codes.a_, suojin, None, eoe0);
							}
						}
						break;
					}
					break;
				}
			}
		}
	}
}
fn line__(s:&str) {
	pr__!("{}\n", s.chars().map(|c| match c {_ if c <= ' ' => '_', _ => c}).collect::<String>());
}

#[derive(Debug)]
pub enum Keyword_ {
	Text,
	Code,
	Val,
	Defv,
	Juhao(Vec<Vec<char>>),
	Dunhao(Vec<Vec<char>>),
	BeginRem(Vec<Vec<char>>),
	EndRem(Vec<Vec<char>>),
	BeginText(Vec<Vec<char>>),
	EndText(Vec<Vec<char>>),
	BeginText2(Vec<Vec<char>>),
	EndText2(Vec<Vec<char>>),
	BeginCode(Vec<Vec<char>>),
	EndCode(Vec<Vec<char>>),
	BeginCode2(Vec<Vec<char>>),
	EndCode2(Vec<Vec<char>>),
	Rem2(Vec<Vec<char>>),
	Var(Vec<Vec<char>>),
	Block(Vec<Vec<char>>),
	For(Vec<Vec<char>>),
	Break(Vec<Vec<char>>),
	Continue(Vec<Vec<char>>),
	Switch(Vec<Vec<char>>),
	Set(Vec<Vec<char>>),
	Def(Vec<Vec<char>>),
	Print(Vec<Vec<char>>),
	Expl(Vec<Vec<char>>),
	Eval(Vec<Vec<char>>),
	Load(Vec<Vec<char>>),
	ParBrkpoint(Vec<Vec<char>>),
	Lf(Vec<Vec<char>>),
	Cr(Vec<Vec<char>>),
	Esc(Vec<Vec<char>>),
}

mod juhao_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Juhao(vec![vec!['。', ], ])]}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod dunhao_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Dunhao(vec![vec!['、', ], ])]}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod begin_rem_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::BeginRem(vec![vec!['（', ], ])]}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod end_rem_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::EndRem(vec![vec!['）', ], ])]}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod begin_text_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::BeginText(vec![vec!['“', ], ])]}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod end_text_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::EndText(vec![vec!['”', ], ])]}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod begin_text2_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::BeginText2(vec![vec!['下', '原', '样', ], ])]}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod end_text2_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::EndText2(vec![vec!['上', '原', '样', ], ])]}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod begin_code_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::BeginCode(vec![vec!['下', '代', '码', ], ])]}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod end_code_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::EndCode(vec![vec!['上', '代', '码', ], ])]}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod begin_code2_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::BeginCode2(vec![vec!['下', '源', '码', ], ])]}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod end_code2_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::EndCode2(vec![vec!['上', '源', '码', ], ])]}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod rem2_ {
	use super::*;
	pub struct Item_ {
		pub a_:Option<Codes_>,
	}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Rem2(vec![vec!['【', ], vec!['】', ], ])]}
		fn a__(&self) -> &Option<Codes_> {&self.a_}
		fn mv_a__(&self) -> Option<Codes_> {self.a_.clone()}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod var_ {
	use super::*;
	pub struct Item_ {
		pub a_:Option<Codes_>,
	}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Var(vec![vec!['‘', ], vec!['’', ], ])]}
		fn a__(&self) -> &Option<Codes_> {&self.a_}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod block_ {
	use super::*;
	pub struct Item_ {
		pub a_:Option<Codes_>,
	}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Block(vec![vec!['则', ], vec!['了', ], ])]}
		fn a__(&self) -> &Option<Codes_> {&self.a_}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod for_ {
	use super::*;
	pub struct Item_ {
		pub a_:Option<Codes_>,
		pub count_:Option<Codes_>, pub start_:Option<Codes_>, pub name_:Option<Codes_>
	}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::For(vec![vec!['循', '环', ], ])]}
		fn a__(&self) -> &Option<Codes_> {&self.a_}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod break_ {
	use super::*;
	pub struct Item_ {
		pub a_:Option<Codes_>,
	}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Break(vec![vec!['跳', '出', ], ])]}
		fn a__(&self) -> &Option<Codes_> {&self.a_}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod continue_ {
	use super::*;
	pub struct Item_ {
		pub a_:Option<Codes_>,
	}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Continue(vec![vec!['再', '来', ], ])]}
		fn a__(&self) -> &Option<Codes_> {&self.a_}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod switch_ {
	use super::*;
	pub struct Item_ {
		pub a_:Option<Codes_>,
		pub case_:Option<Vec<Codes_>>, pub op_:Option<Vec<Op_>>,
	}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Switch(vec![vec!['分', '叉', ], ])]}
		fn a__(&self) -> &Option<Codes_> {&self.a_}
		fn a_tag__(&self) -> Option<&str> {Some("left")}

		fn as_any(&self) -> &dyn Any {self}
	}
	#[derive(Debug)]
	pub enum Op_ {
		Eq,
		Ne,
		Le,
		Lt,
		Ge,
		Gt
	}

}
mod set_ {
	use super::*;
	pub struct Item_ {
		pub names_:Option<Codes_>, pub vals_:Option<Codes_>,
	}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Set(vec![vec!['赋', '予', ], vec!['以', ], ])]}
		fn a__(&self) -> &Option<Codes_> {&self.names_}
		fn a_tag__(&self) -> Option<&str> {Some("name")}

		fn as_any(&self) -> &dyn Any {self}
	}
}
mod def_ {
	use super::*;
	pub struct Item_ {
		pub names_:Option<Codes_>, pub code_:Option<Codes_>,
	}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Def(vec![vec!['定', '义', ], vec!['以', ], ])]}
		fn a__(&self) -> &Option<Codes_> {&self.names_}
		fn a_tag__(&self) -> Option<&str> {Some("name")}

		fn as_any(&self) -> &dyn Any {self}
	}
}
mod print_ {
	use super::*;
	pub struct Item_ {
		pub a_:Option<Codes_>,
	}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Print(vec![vec!['显', '示', ], ])]}
		fn a__(&self) -> &Option<Codes_> {&self.a_}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod expl_ {
	use super::*;
	pub struct Item_ {
		pub a_:Option<Codes_>,
	}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Expl(vec![vec!['算', '术', ], ])]}
		fn a__(&self) -> &Option<Codes_> {&self.a_}
		fn as_any(&self) -> &dyn Any {self}
	}
	#[derive(Debug)]
	pub enum Op_ {
		Add,
		Sub,
		Mul,
		Div,
		Mod,
		Pow,
		BeginG,
		EndG,
		Num(f64),
		No
	}

}
mod eval_ {
	use super::*;
	pub struct Item_ {
		pub a_:Option<Codes_>,
	}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Eval(vec![vec!['解', '释', ], ])]}
		fn a__(&self) -> &Option<Codes_> {&self.a_}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod load_ {
	use super::*;
	pub struct Item_ {
		pub a_:Option<Codes_>,
	}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Load(vec![vec!['加', '载', ], ])]}
		fn a__(&self) -> &Option<Codes_> {&self.a_}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod par_brkpoint_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::ParBrkpoint(vec![vec!['这', '析', '断', '点', ], ])]}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod lf_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Lf(vec![vec!['换', '行', ], ])]}
		fn s__(&self) -> Option<&str> {Some("\n")}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod cr_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Cr(vec![vec!['回', '车', ], ])]}
		fn s__(&self) -> Option<&str> {Some("\r")}
		fn as_any(&self) -> &dyn Any {self}
	}
}
mod esc_ {
	use super::*;
	pub struct Item_ {}
	impl CodeImpl_ for Item_ {
		fn kw__(&self) -> Vec<Keyword_> {vec![Keyword_::Esc(vec![vec!['E', 'S', 'C', ], ])]}
		fn s__(&self) -> Option<&str> {Some("\x1b")}
		fn as_any(&self) -> &dyn Any {self}
	}
}
