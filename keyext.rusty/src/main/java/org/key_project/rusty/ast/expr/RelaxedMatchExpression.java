package org.key_project.rusty.ast.expr;

import org.key_project.rusty.ast.visitor.Visitor;
import org.key_project.util.collection.ImmutableArray;

public class RelaxedMatchExpression  extends MatchExpression{
    public RelaxedMatchExpression(Expr expr, ImmutableArray<IMatchArm> arms) {
        super(expr, arms);
    }

    @Override
    public void visit(Visitor v) {
        v.performActionOnRelaxedMatch(this);
    }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append("r_match (").append(expr()).append(") {\n");
        for (int i = 0; i < arms().size(); i++) {
            if (i > 0)
                sb.append(", ");
            sb.append(arms().get(i));
        }
        sb.append("}");
        return sb.toString();
    }
}
