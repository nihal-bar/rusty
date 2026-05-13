/* This file is part of KeY - https://key-project.org
 * KeY is licensed under the GNU General Public License Version 2
 * SPDX-License-Identifier: GPL-2.0-only */
package org.key_project.rusty.ast.pat;

import java.util.Objects;

import org.key_project.logic.SyntaxElement;
import org.key_project.rusty.ast.QPath;
import org.key_project.rusty.ast.visitor.Visitor;
import org.key_project.util.collection.ImmutableArray;

import org.jspecify.annotations.NonNull;

public record TupleStructPattern(QPath path, ImmutableArray<Pattern> patterns, int dotDotPos)
        implements Pattern {
    @Override
    public void visit(Visitor v) {
        v.performActionOnTupleStructPattern(this);
    }

    @Override
    public @NonNull SyntaxElement getChild(int n) {
        if (n == 0)
            return path;
        return Objects.requireNonNull(patterns.get(n - 1));
    }

    @Override
    public int getChildCount() {
        return 1 + patterns.size();
    }

    @Override
    public @NonNull String toString() {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < patterns.size(); i++) {
            var p = patterns.get(i);
            if (!sb.isEmpty()) {
                sb.append(", ");
            }
            if (dotDotPos == i) {
                sb.append("..");
                if (i != patterns.size() - 1) {
                    sb.append(", ");
                }
            }
            sb.append(p.toString());
        }
        return path() + "(" + sb + ")";
    }
}
