/* This file is part of KeY - https://key-project.org
 * KeY is licensed under the GNU General Public License Version 2
 * SPDX-License-Identifier: GPL-2.0-only */
package org.key_project.rusty.parser.hir.expr;

import org.key_project.rusty.parser.hir.HirAdapter;
import org.key_project.rusty.parser.hir.HirId;

import org.jspecify.annotations.Nullable;

public interface MatchSource {
    record Normal() implements MatchSource {
    }

    record Postfix() implements MatchSource {
    }

    record ForLoopDesugar() implements MatchSource {
    }

    record TryDesugar(HirId hirId) implements MatchSource {
    }

    record AwaitDesugar() implements MatchSource {
    }

    record FormatArgs() implements MatchSource {
    }

    class Adapter extends HirAdapter<MatchSource> {
        @Override
        public @Nullable Class<? extends MatchSource> getType(String tag) {
            return switch (tag) {
                case "Normal" -> Normal.class;
                case "Postfix" -> Postfix.class;
                case "ForLoopDesugar" -> ForLoopDesugar.class;
                case "TryDesugar" -> TryDesugar.class;
                case "AwaitDesugar" -> AwaitDesugar.class;
                case "FormatArgs" -> FormatArgs.class;
                default -> null;
            };
        }
    }
}
