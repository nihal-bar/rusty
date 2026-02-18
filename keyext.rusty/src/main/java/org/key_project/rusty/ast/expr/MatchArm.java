/* This file is part of KeY - https://key-project.org
 * KeY is licensed under the GNU General Public License Version 2
 * SPDX-License-Identifier: GPL-2.0-only */
package org.key_project.rusty.ast.expr;

import java.util.Objects;

import org.key_project.logic.SyntaxElement;
import org.key_project.rusty.ast.RustyProgramElement;
import org.key_project.rusty.ast.pat.Pattern;
import org.key_project.rusty.ast.visitor.Visitor;
import org.key_project.util.ExtList;

import org.jspecify.annotations.NonNull;
import org.jspecify.annotations.Nullable;

public final class MatchArm
        implements RustyProgramElement, IMatchArm {
    private final Pattern pattern;
    private final @Nullable Expr guard;
    private final Expr body;

    public MatchArm(Pattern pattern, @Nullable Expr guard, Expr body) {
        this.pattern = pattern;
        this.guard = guard;
        this.body = body;
    }

    public MatchArm(ExtList children) {
        this.pattern = children.removeFirstOccurrence(Pattern.class);
        var exprs = children.collect(Expr.class);
        assert exprs.length <= 2;
        if (exprs.length == 2) {
            guard = exprs[0];
            body = exprs[1];
        } else {
            guard = null;
            body = exprs[0];
        }
    }

    @Override
    public void visit(Visitor v) {
        v.performActionOnMatchArm(this);
    }

    @Override
    public @NonNull SyntaxElement getChild(int n) {
        if (n == 0) {
            return pattern;
        }
        if (guard != null) {
            if (n == 1) {
                return guard;
            }
            --n;
        }
        if (n == 1) {
            return body;
        }
        throw new IndexOutOfBoundsException("MatchArm has only " + getChildCount() + " children");
    }

    @Override
    public int getChildCount() {
        return 2 + (guard == null ? 0 : 1);
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append(pattern);
        if (guard != null) {
            sb.append(" if ").append(guard);
        }
        sb.append(" => ").append(body);
        return sb.toString();
    }

    public Pattern pattern() {
        return pattern;
    }

    public @Nullable Expr guard() {
        return guard;
    }

    public Expr body() {
        return body;
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == this)
            return true;
        if (obj == null || obj.getClass() != this.getClass())
            return false;
        var that = (MatchArm) obj;
        return Objects.equals(this.pattern, that.pattern) &&
                Objects.equals(this.guard, that.guard) &&
                Objects.equals(this.body, that.body);
    }

    @Override
    public int hashCode() {
        return Objects.hash(pattern, guard, body);
    }

}
