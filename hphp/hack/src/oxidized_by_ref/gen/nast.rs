// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
//
// @generated SignedSource<<f89104af28f52219207878730e0722e2>>
//
// To regenerate this file, run:
//   hphp/hack/src/oxidized_by_ref/regen.sh

use ocamlrep_derive::ToOcamlRep;
use serde::Serialize;

#[allow(unused_imports)]
use crate::*;

pub use crate::ast_defs::shape_map;

pub use aast::Sid;

#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub enum FuncBodyAnn<'a> {
    Named,
    NamedWithUnsafeBlocks,
    Unnamed(&'a namespace_env::Env<'a>),
}

pub type Program<'a> = aast::Program<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Def<'a> = aast::Def<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Expr<'a> = aast::Expr<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Expr_<'a> = aast::Expr_<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Stmt<'a> = aast::Stmt<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Block<'a> = aast::Block<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type UserAttribute<'a> = aast::UserAttribute<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type ClassId_<'a> = aast::ClassId_<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Class_<'a> = aast::Class_<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Method_<'a> = aast::Method_<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type FileAttribute<'a> = aast::FileAttribute<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Fun_<'a> = aast::Fun_<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type FunDef<'a> = aast::FunDef<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type FuncBody<'a> = aast::FuncBody<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type FunParam<'a> = aast::FunParam<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type FunVariadicity<'a> = aast::FunVariadicity<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Typedef<'a> = aast::Typedef<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type RecordDef<'a> = aast::RecordDef<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Tparam<'a> = aast::Tparam<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Gconst<'a> = aast::Gconst<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type ClassId<'a> = aast::ClassId<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Catch<'a> = aast::Catch<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Case<'a> = aast::Case<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Field<'a> = aast::Field<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Afield<'a> = aast::Afield<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type MethodRedeclaration<'a> =
    aast::MethodRedeclaration<'a, &'a pos::Pos<'a>, FuncBodyAnn<'a>, (), ()>;

pub type Targ<'a> = aast::Targ<'a, ()>;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, ToOcamlRep
)]
pub struct IgnoreAttributeEnv<'a> {
    pub ignored_attributes: &'a [&'a str],
}
