/* This file is part of KeY - https://key-project.org
 * KeY is licensed under the GNU General Public License Version 2
 * SPDX-License-Identifier: GPL-2.0-only */
package org.key_project.rusty.ast.pat;

import java.util.Objects;

import org.key_project.logic.SyntaxElement;
import org.key_project.rusty.ast.visitor.Visitor;
import org.key_project.util.collection.ImmutableArray;

import org.jspecify.annotations.NonNull;
import org.jspecify.annotations.Nullable;

public record SlicePattern(ImmutableArray<Pattern> start, @Nullable Pattern mid,
        ImmutableArray<Pattern> end) implements Pattern {
    @Override
    public void visit(Visitor v) {
        v.performActionOnSlicePattern(this);
    }

    @Override
    public @NonNull SyntaxElement getChild(int n) {
        if (0 <= n && n < start.size()) {
            return Objects.requireNonNull(start.get(n));
        }
        n -= start.size();
        if (n == 0 && mid != null) {
            return mid;
        }
        if (mid != null) {
            n--;
        }
        return Objects.requireNonNull(end.get(n));
    }

    @Override
    public int getChildCount() {
        return start.size() + (mid == null ? 0 : 1) + end.size();
    }
}
