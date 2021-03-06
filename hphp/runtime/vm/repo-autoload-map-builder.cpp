/*
   +----------------------------------------------------------------------+
   | HipHop for PHP                                                       |
   +----------------------------------------------------------------------+
   | Copyright (c) 2010-present Facebook, Inc. (http://www.facebook.com)  |
   +----------------------------------------------------------------------+
   | This source file is subject to version 3.01 of the PHP license,      |
   | that is bundled with this package in the file LICENSE, and is        |
   | available through the world-wide-web at the following url:           |
   | http://www.php.net/license/3_01.txt                                  |
   | If you did not receive a copy of the PHP license and are unable to   |
   | obtain it through the world-wide-web, please send a note to          |
   | license@php.net so we can mail you a copy immediately.               |
   +----------------------------------------------------------------------+
*/

#include "hphp/runtime/vm/repo-autoload-map-builder.h"

#include "hphp/runtime/vm/constant.h"
#include "hphp/runtime/vm/func-emitter.h"
#include "hphp/runtime/vm/preclass-emitter.h"
#include "hphp/runtime/vm/type-alias.h"

#include <boost/algorithm/string/predicate.hpp>

namespace HPHP {

//////////////////////////////////////////////////////////////////////

void RepoAutoloadMapBuilder::addUnit(const UnitEmitter& ue) {
  auto unitSn = ue.m_sn;
  assertx(unitSn != -1 && "unitSn is invalid");
  for (size_t n = 0; n < ue.numPreClasses(); ++n) {
    auto pce = ue.pce(n);
    if (!boost::starts_with(pce->name()->slice(), "Closure$")) {
      m_classes.insert(std::make_pair(pce->name(), unitSn));
    }
  }
  for (auto& fe : ue.fevec()) {
    if (fe->top && !boost::ends_with(fe->name->slice(), "$memoize_impl")) {
      m_funcs.insert(std::make_pair(fe->name, unitSn));
    }
  }
  for (auto& ta : ue.typeAliases()) {
    m_typeAliases.insert(std::make_pair(ta.name, unitSn));
  }

  for (auto& c : ue.constants()) {
    m_constants.insert(std::make_pair(c.name, unitSn));
  }
}

void RepoAutoloadMapBuilder::serde(BlobEncoder& sd) const {
  serdeMap(sd, m_classes);
  serdeMap(sd, m_funcs);
  serdeMap(sd, m_typeAliases);
  serdeMap(sd, m_constants);
}

std::unique_ptr<RepoAutoloadMap> RepoAutoloadMapBuilder::serde(BlobDecoder& sd) {
  RepoAutoloadMap::CaseInsensitiveMap classes = serdeMap<RepoAutoloadMap::CaseInsensitiveMap>(sd);
  RepoAutoloadMap::CaseInsensitiveMap funcs = serdeMap<RepoAutoloadMap::CaseInsensitiveMap>(sd);
  RepoAutoloadMap::CaseInsensitiveMap typeAliases = serdeMap<RepoAutoloadMap::CaseInsensitiveMap>(sd);
  RepoAutoloadMap::CaseSensitiveMap constants = serdeMap<RepoAutoloadMap::CaseSensitiveMap>(sd);
  return std::make_unique<RepoAutoloadMap>(classes, funcs, constants, typeAliases);
}

//////////////////////////////////////////////////////////////////////

}
