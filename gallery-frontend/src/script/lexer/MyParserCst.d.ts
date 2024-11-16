import type { CstNode, ICstVisitor, IToken } from "chevrotain";

export interface ExpressionCstNode extends CstNode {
  name: "expression";
  children: ExpressionCstChildren;
}

export interface ExpressionCstChildren {
  orExpression?: OrExpressionCstNode[];
  andExpression?: AndExpressionCstNode[];
  notExpression?: NotExpressionCstNode[];
  atomicExpression?: AtomicExpressionCstNode[];
}

export interface OrExpressionCstNode extends CstNode {
  name: "orExpression";
  children: OrExpressionCstChildren;
}

export interface OrExpressionCstChildren {
  Or: IToken[];
  OpenParenthesis: IToken[];
  expression: (ExpressionCstNode)[];
  Comma?: IToken[];
  CloseParenthesis: IToken[];
}

export interface AndExpressionCstNode extends CstNode {
  name: "andExpression";
  children: AndExpressionCstChildren;
}

export interface AndExpressionCstChildren {
  And: IToken[];
  OpenParenthesis: IToken[];
  expression: (ExpressionCstNode)[];
  Comma?: IToken[];
  CloseParenthesis: IToken[];
}

export interface AtomicExpressionCstNode extends CstNode {
  name: "atomicExpression";
  children: AtomicExpressionCstChildren;
}

export interface AtomicExpressionCstChildren {
  tagExpression?: TagExpressionCstNode[];
  typeExpression?: TypeExpressionCstNode[];
  extExpression?: ExtExpressionCstNode[];
  makeExpression?: MakeExpressionCstNode[];
  modelExpression?: ModelExpressionCstNode[];
  albumExpression?: AlbumExpressionCstNode[];
  pathExpression?: PathExpressionCstNode[];
  anyExpression?: AnyExpressionCstNode[];
}

export interface NotExpressionCstNode extends CstNode {
  name: "notExpression";
  children: NotExpressionCstChildren;
}

export interface NotExpressionCstChildren {
  Not: IToken[];
  OpenParenthesis: IToken[];
  expression: ExpressionCstNode[];
  CloseParenthesis: IToken[];
}

export interface TagExpressionCstNode extends CstNode {
  name: "tagExpression";
  children: TagExpressionCstChildren;
}

export interface TagExpressionCstChildren {
  Tag: IToken[];
  Identifier: IToken[];
}

export interface TypeExpressionCstNode extends CstNode {
  name: "typeExpression";
  children: TypeExpressionCstChildren;
}

export interface TypeExpressionCstChildren {
  Type: IToken[];
  Identifier: IToken[];
}

export interface ExtExpressionCstNode extends CstNode {
  name: "extExpression";
  children: ExtExpressionCstChildren;
}

export interface ExtExpressionCstChildren {
  Ext: IToken[];
  Identifier: IToken[];
}

export interface MakeExpressionCstNode extends CstNode {
  name: "makeExpression";
  children: MakeExpressionCstChildren;
}

export interface MakeExpressionCstChildren {
  Makel: IToken[];
  Identifier: IToken[];
}

export interface ModelExpressionCstNode extends CstNode {
  name: "modelExpression";
  children: ModelExpressionCstChildren;
}

export interface ModelExpressionCstChildren {
  Model: IToken[];
  Identifier: IToken[];
}

export interface AlbumExpressionCstNode extends CstNode {
  name: "albumExpression";
  children: AlbumExpressionCstChildren;
}

export interface AlbumExpressionCstChildren {
  Album: IToken[];
  Identifier: IToken[];
}

export interface PathExpressionCstNode extends CstNode {
  name: "pathExpression";
  children: PathExpressionCstChildren;
}

export interface PathExpressionCstChildren {
  Path: IToken[];
  Identifier: IToken[];
}

export interface AnyExpressionCstNode extends CstNode {
  name: "anyExpression";
  children: AnyExpressionCstChildren;
}

export interface AnyExpressionCstChildren {
  Any: IToken[];
  Identifier: IToken[];
}

export interface ICstNodeVisitor<IN, OUT> extends ICstVisitor<IN, OUT> {
  expression(children: ExpressionCstChildren, param?: IN): OUT;
  orExpression(children: OrExpressionCstChildren, param?: IN): OUT;
  andExpression(children: AndExpressionCstChildren, param?: IN): OUT;
  atomicExpression(children: AtomicExpressionCstChildren, param?: IN): OUT;
  notExpression(children: NotExpressionCstChildren, param?: IN): OUT;
  tagExpression(children: TagExpressionCstChildren, param?: IN): OUT;
  typeExpression(children: TypeExpressionCstChildren, param?: IN): OUT;
  extExpression(children: ExtExpressionCstChildren, param?: IN): OUT;
  makeExpression(children: MakeExpressionCstChildren, param?: IN): OUT;
  modelExpression(children: ModelExpressionCstChildren, param?: IN): OUT;
  albumExpression(children: AlbumExpressionCstChildren, param?: IN): OUT;
  pathExpression(children: PathExpressionCstChildren, param?: IN): OUT;
  anyExpression(children: AnyExpressionCstChildren, param?: IN): OUT;
}
