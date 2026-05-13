/* This file is part of KeY - https://key-project.org
 * KeY is licensed under the GNU General Public License Version 2
 * SPDX-License-Identifier: GPL-2.0-only */
package org.key_project.rusty.ast.pat;

import org.key_project.logic.SyntaxElement;
import org.key_project.rusty.ast.Identifier;
import org.key_project.rusty.ast.RustyProgramElement;
import org.key_project.rusty.ast.visitor.Visitor;

import org.jspecify.annotations.NonNull;

public record PatField(Identifier name, Pattern pattern, boolean isShorthand)
        implements RustyProgramElement {
    @Override
    public void visit(Visitor v) {
        v.performActionOnPatField(this);
    }

    @Override
    public @NonNull SyntaxElement getChild(int n) {
        if (isShorthand && n == 0) {
            return pattern;
        }
        if (!isShorthand) {
            if (n == 0)
                return name;
            if (n == 1)
                return pattern;
        }
        throw new IndexOutOfBoundsException(n);
    }

    @Override
    public int getChildCount() {
        return isShorthand ? 1 : 2;
    }

    @Override
    public @NonNull String toString() {
        if (isShorthand) {
            return name.toString();
        }
        return name + ": " + pattern.toString();
    }
}
