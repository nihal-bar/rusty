/* This file is part of KeY - https://key-project.org
 * KeY is licensed under the GNU General Public License Version 2
 * SPDX-License-Identifier: GPL-2.0-only */
package org.key_project.rusty.ast.pat;

import org.key_project.logic.SyntaxElement;
import org.key_project.rusty.ast.RustyProgramElement;
import org.key_project.rusty.ast.visitor.Visitor;

import org.jspecify.annotations.Nullable;
import org.key_project.util.ExtList;

import java.util.Objects;

/// This class represents range patterns.
///
/// <a href="https://doc.rust-lang.org/reference/patterns.html#range-patterns">RangePattern
/// Grammar</a>
public final class RangePattern
        implements Pattern {
    private final @Nullable PatExpr left;
    private final Bounds bounds;
    private final @Nullable PatExpr right;

    /**
     *
     */
    public RangePattern(@Nullable PatExpr left, Bounds bounds, @Nullable PatExpr right) {
        this.left = left;
        this.bounds = bounds;
        this.right = right;
    }

    public RangePattern(ExtList children) {
        bounds = children.get(Bounds.class);
        var pats = children.collect(PatExpr.class);
        assert pats.length <= 2;
        if (pats.length == 2) {
            left = pats[0];
            right = pats[1];
        } else {
            left = null;
            right = pats[0];
        }

    }

    public enum Bounds
            implements RustyProgramElement {
        Inclusive("..="), Exclusive(".."), Obsolete("...");

        private final String bounds;

        Bounds(String bounds) {
            this.bounds = bounds;
        }

        @Override
        public String toString() {
            return bounds;
        }

        @Override
        public SyntaxElement getChild(int n) {
            throw new IndexOutOfBoundsException();
        }

        @Override
        public int getChildCount() {
            return 0;
        }

        @Override
        public void visit(Visitor v) {
            // Bounds should stay invisible to the visitors and therefore no visit is needed
        }
    }

    @Override
    public void visit(Visitor v) {
        v.performActionOnRangepattern(this);
    }

    @Override
    public SyntaxElement getChild(int n) {
        if (n == 0 && left != null)
            return left;
        if (left != null)
            --n;
        if (n == 0)
            return bounds;
        --n;
        if (n == 0 && right != null)
            return right;
        throw new IndexOutOfBoundsException(
                "RangePattern has only " + getChildCount() + " children");
    }

    @Override
    public int getChildCount() {
        int count = 1; // for ../..=/...
        if (left != null)
            ++count;
        if (right != null)
            ++count;
        return count;
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        if (left != null)
            sb.append(left);
        sb.append(bounds);
        if (right != null)
            sb.append(right);
        return sb.toString();
    }

    public @Nullable PatExpr left() {
        return left;
    }

    public Bounds bounds() {
        return bounds;
    }

    public @Nullable PatExpr right() {
        return right;
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == this) return true;
        if (obj == null || obj.getClass() != this.getClass()) return false;
        var that = (RangePattern) obj;
        return Objects.equals(this.left, that.left) &&
                Objects.equals(this.bounds, that.bounds) &&
                Objects.equals(this.right, that.right);
    }

    @Override
    public int hashCode() {
        return Objects.hash(left, bounds, right);
    }

}
