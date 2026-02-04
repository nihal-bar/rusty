/* This file is part of KeY - https://key-project.org
 * KeY is licensed under the GNU General Public License Version 2
 * SPDX-License-Identifier: GPL-2.0-only */
package org.key_project.rusty.ast.expr;

import java.util.Objects;

import org.key_project.logic.SyntaxElement;
import org.key_project.rusty.Services;
import org.key_project.rusty.ast.abstraction.Type;
import org.key_project.rusty.ast.visitor.Visitor;
import org.key_project.util.collection.ImmutableArray;

import org.jspecify.annotations.NonNull;

public class MatchExpression implements Expr {
    private final Expr expr;
    private final ImmutableArray<IMatchArm> arms;

    public MatchExpression(Expr expr, ImmutableArray<IMatchArm> arms) {
        this.expr = expr;
        this.arms = arms;
    }

    @Override
    public void visit(Visitor v) {
        v.performActionOnMatchExpression(this);
    }

    @Override
    public @NonNull SyntaxElement getChild(int n) {
        if (n == 0)
            return expr;
        return Objects.requireNonNull(arms.get(n - 1));
    }

    @Override
    public int getChildCount() {
        return 1 + arms.size();
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append("match (").append(expr).append(") {\n");
        for (int i = 0; i < arms.size(); i++) {
            if (i > 0)
                sb.append(", ");
            sb.append(arms.get(i));
        }
        sb.append("}");
        return sb.toString();
    }

    @Override
    public Type type(Services services) {
        throw new UnsupportedOperationException();
    }

    public Expr expr() {
        return expr;
    }

    public ImmutableArray<IMatchArm> arms() {
        return arms;
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == this) return true;
        if (obj == null || obj.getClass() != this.getClass()) return false;
        var that = (MatchExpression) obj;
        return Objects.equals(this.expr, that.expr) &&
                Objects.equals(this.arms, that.arms);
    }

    @Override
    public int hashCode() {
        return Objects.hash(expr, arms);
    }

}
