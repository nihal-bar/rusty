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

public record StructPattern(QPath path, ImmutableArray<PatField> fields, boolean rest)
        implements Pattern {
    @Override
    public void visit(Visitor v) {
        v.performActionOnStructPattern(this);
    }

    @Override
    public @NonNull SyntaxElement getChild(int n) {
        if (n == 0)
            return path;
        n--;
        return Objects.requireNonNull(fields.get(n));
    }

    @Override
    public int getChildCount() {
        return 1 + fields.size();
    }

    @Override
    public @NonNull String toString() {
        var sb = new StringBuilder();
        for (PatField field : fields) {
            if (!sb.isEmpty()) {
                sb.append(", ");
            }
            sb.append(field.toString());
        }
        if (rest) {
            sb.append(", ..");
        }
        return path.toString() + " { " + sb + "}";
    }
}
