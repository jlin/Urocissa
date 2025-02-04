import { MyLexer, MyParser, MyVisitor } from '@/script/lexer/lexer'
export function generateJsonString(inputText: string): string {
  console.log('inputText is', inputText)

  const lexingResult = MyLexer.tokenize(inputText)
  if (lexingResult.errors.length) {
    console.error(lexingResult.errors)
    throw new Error('Lexing errors detected')
  }

  const parser = new MyParser()
  parser.input = lexingResult.tokens
  const cst = parser.expression()
  if (parser.errors.length) {
    console.error('Parsing errors detected')
    console.error(parser.errors)
    throw new Error('Parsing errors detected')
  }

  const visitor = new MyVisitor()
  const json = visitor.visit(cst)

  return JSON.stringify(json)
}
