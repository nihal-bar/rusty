/* This file is part of KeY - https://key-project.org
 * KeY is licensed under the GNU General Public License Version 2
 * SPDX-License-Identifier: GPL-2.0-only */
package org.key_project.rusty.ast.pat;

import org.key_project.logic.SyntaxElement;
import org.key_project.logic.op.Function;
import org.key_project.rusty.ast.QPath;
import org.key_project.rusty.ast.abstraction.Type;
import org.key_project.rusty.ast.expr.Expr;
import org.key_project.rusty.ast.visitor.Visitor;

import org.jspecify.annotations.NonNull;

public record PathPatExpr(QPath path, Type ty, Function constant) implements PatExpr {
    @Override
    public void visit(Visitor v) {
        v.performActionOnPathPatExpr(this);
    }

    @Override
    public @NonNull SyntaxElement getChild(int n) {
        if (n == 0)
            return path;
        throw new IndexOutOfBoundsException(n);
    }

    @Override
    public int getChildCount() {
        return 1;
    }

    @Override
    public Expr toExpr() {
        return null;
    }
}
