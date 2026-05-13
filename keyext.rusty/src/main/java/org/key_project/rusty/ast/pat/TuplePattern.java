/* This file is part of KeY - https://key-project.org
 * KeY is licensed under the GNU General Public License Version 2
 * SPDX-License-Identifier: GPL-2.0-only */
package org.key_project.rusty.ast.pat;

import java.util.Objects;

import org.key_project.logic.SyntaxElement;
import org.key_project.rusty.ast.visitor.Visitor;
import org.key_project.util.collection.ImmutableArray;

import org.jspecify.annotations.NonNull;

public record TuplePattern(ImmutableArray<Pattern> patterns, int dotDotPos) implements Pattern {
    @Override
    public void visit(Visitor v) {
        v.performActionOnTuplePattern(this);
    }

    @Override
    public @NonNull SyntaxElement getChild(int n) {
        return Objects.requireNonNull(patterns.get(n));
    }

    @Override
    public int getChildCount() {
        return patterns().size();
    }
}
