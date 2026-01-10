use backyard_lexer::token::{ Token, TokenType };
use backyard_nodes::{
  utils::{ IntoBoxedNode, IntoBoxedOptionNode },
  CommentBlockNode,
  CommentDocNode,
  CommentLineNode,
  DocArrayTypeNode,
  DocCallableTypeNode,
  DocIntersectionTypeNode,
  DocListTypeNode,
  DocParamNode,
  DocStructuredArrayItemTypeNode,
  DocStructuredArrayTypeNode,
  DocTextNode,
  DocTypeNode,
  DocUnionTypeNode,
  DocVariableNode,
  Location,
  Node,
  NodeType,
  RangeLocation,
};
use bstr::{ BString, ByteVec };

use crate::{
  error::ParserError,
  parser::{ LocationHelper, LoopArgument, Parser },
  utils::{ match_pattern, Lookup, LookupResult },
};

#[derive(Debug, Clone)]
pub struct CommentParser;

impl CommentParser {
  pub fn test<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    _: &mut LoopArgument
  ) -> Option<std::vec::Vec<LookupResult<'arena>>> {
    match_pattern(
      parser,
      &[
        Lookup::Equal(
          &[TokenType::CommentLine, TokenType::CommentBlock, TokenType::CommentDocOpen]
        ),
      ]
    )
  }

  pub fn parse<'arena, 'a, 'b>(
    parser: &mut Parser<'arena, 'a>,
    matched: std::vec::Vec<LookupResult<'arena>>,
    start_loc: Location,
    args: &mut LoopArgument<'arena, 'b>
  ) -> Result<Node<'arena>, ParserError> {
    if let [comment] = matched.as_slice() {
      let comment = comment.as_equal(parser)?;
      let comment = match comment.token_type {
        TokenType::CommentLine =>
          CommentLineNode::loc(comment.value.to_owned(), parser.gen_loc(start_loc)),
        TokenType::CommentBlock =>
          CommentBlockNode::loc(comment.value.to_owned(), parser.gen_loc(start_loc)),
        TokenType::CommentDocOpen => CommentDocParser::parse_doc(parser, start_loc)?,
        // TokenType::CommentDoc =>
        //   CommentDocNode::loc(comment.value.to_owned(), parser.gen_loc(start_loc)),
        _ => {
          return Err(ParserError::Internal);
        }
      };
      let expr = parser.get_statement(
        &mut LoopArgument::safe(
          parser.arena,
          "comment",
          args.separators,
          args.breakers,
          args.parsers
        )
      )?;
      if let Some(test_expr) = &expr {
        if
          ![NodeType::CommentBlock, NodeType::CommentDoc, NodeType::CommentLine].contains(
            &test_expr.node_type
          )
        {
          let mut expr = expr.unwrap();
          if let Some(expr_loc) = &expr.loc {
            if let Some(comment_loc) = &comment.loc {
              if expr_loc.start.offset < comment_loc.start.offset {
                expr.trailings_push(parser.arena, comment);
              } else {
                expr.leadings_shift(parser.arena, comment);
              }
            }
          }
          return Ok(expr);
        }
      }
      if let Some(mut last_expr) = args.last_expr.take() {
        if
          ![NodeType::CommentBlock, NodeType::CommentDoc, NodeType::CommentLine].contains(
            &last_expr.node_type
          )
        {
          last_expr.trailings_push(parser.arena, comment);
          if let Some(next_expr) = expr {
            if
              [NodeType::CommentBlock, NodeType::CommentDoc, NodeType::CommentLine].contains(
                &next_expr.node_type
              )
            {
              last_expr.trailings_push(parser.arena, next_expr);
            }
          }
          return Ok(last_expr);
        }
      }
      if let Some(statements) = &mut args.statements {
        if let Some(expr) = statements.pop() {
          if
            ![NodeType::CommentBlock, NodeType::CommentDoc, NodeType::CommentLine].contains(
              &expr.node_type
            )
          {
            return Ok(expr);
          }
        }
      }
      return Ok(comment);
    }
    Err(ParserError::Internal)
  }
}

#[derive(Debug, Clone)]
pub struct CommentDocParser;

impl CommentDocParser {
  fn prepare_text(vec: Vec<Token>) -> BString {
    let mut result = BString::new(vec![]);
    let last_index = vec.len() - 1;
    for (i, item) in vec.into_iter().enumerate() {
      result.push_str(item.value.to_owned());
      if i < last_index {
        result.push(b' ');
      }
    }
    result
  }

  fn parse_doc<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    start_loc: Location
  ) -> Result<Node<'arena>, ParserError> {
    let mut contents = bumpalo::collections::Vec::new_in(parser.arena);
    let mut temp_text = vec![];
    loop {
      let token = parser.get_token(parser.position)?;
      if token.token_type == TokenType::DocUnknown {
        temp_text.push(token.clone());
        parser.position += 1;
      } else if token.token_type == TokenType::CommentDocClose {
        parser.position += 1;
        break;
      } else {
        if !temp_text.is_empty() {
          let start = temp_text.first().unwrap().get_location().unwrap();
          let end = temp_text.last().unwrap().get_range_location().unwrap().end;
          let cloned = temp_text.to_owned();
          temp_text = vec![];
          contents.push(
            DocTextNode::loc(Self::prepare_text(cloned), Some(RangeLocation { start, end }))
          );
        }

        // case '@param':
        // case '@phpstan-param':
        // case '@psalm-param':
        // case '@phan-param':
        match token.token_type {
          | TokenType::DocTagParam
          | TokenType::DocTagPhpstanParam
          | TokenType::DocTagPsalmParam
          | TokenType::DocTagPhanParam => {
            let start = token.get_location().unwrap();
            parser.position += 1;
            let t = Self::try_get_complex_type(parser)?.into_boxed(&parser.arena);
            let v = Self::try_get_variable(parser)?;
            let end = parser
              .get_token(parser.position - 1)?
              .get_range_location()
              .unwrap().end;
            contents.push(DocParamNode::loc(t, v, None, Some(RangeLocation { start, end })));
          }
          _ => {
            return Err(ParserError::Internal); // TODO: remove this
          }
        }
      }
    }
    let end = parser
      .get_token(parser.position - 1)?
      .get_range_location()
      .unwrap().end;
    Ok(CommentDocNode::loc(contents, Some(RangeLocation { start: start_loc, end })))
  }

  fn try_get_variable<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>
  ) -> Result<Option<bumpalo::boxed::Box<'arena, Node<'arena>>>, ParserError> {
    let token = parser.get_token(parser.position)?;
    if token.token_type == TokenType::DocVariable {
      let loc = token.get_range_location();
      let value = token.value.to_owned();
      parser.position += 1;
      return Ok(Some(DocVariableNode::loc(value, loc).into_boxed(&parser.arena)));
    }
    Ok(None)
  }

  fn try_get_complex_type<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>
  ) -> Result<Option<Node<'arena>>, ParserError> {
    let first_type = Self::try_get_type(parser)?;
    if first_type.is_none() {
      return Ok(None);
    }
    let next_token = parser.get_token(parser.position)?;
    if next_token.token_type == TokenType::DocBitwiseAnd {
      let items = Self::try_parse_union_or_intersection_type(
        parser,
        first_type.unwrap(),
        TokenType::DocBitwiseAnd
      )?;
      return Ok(Some(DocIntersectionTypeNode::loc(items, None)));
    } else if next_token.token_type == TokenType::DocBitwiseOr {
      let items = Self::try_parse_union_or_intersection_type(
        parser,
        first_type.unwrap(),
        TokenType::DocBitwiseOr
      )?;
      return Ok(Some(DocUnionTypeNode::loc(items, None)));
    }
    return Ok(first_type);
  }

  fn try_parse_union_or_intersection_type<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>,
    first_type: Node<'arena>,
    separator: TokenType
  ) -> Result<bumpalo::collections::Vec<'arena, Node<'arena>>, ParserError> {
    let mut result = bumpalo::vec![in parser.arena; first_type];
    let mut expect_separator = true;
    loop {
      let token = parser.get_token(parser.position)?;

      if
        !(
          [
            TokenType::DocType,
            TokenType::DocRelativeName,
            TokenType::DocQualifiedName,
            TokenType::DocUnqualifiedName,
            TokenType::DocFullyQualifiedName,
            TokenType::DocLeftParenthesis,
          ].contains(&token.token_type) || separator == token.token_type
        )
      {
        break;
      }
      if expect_separator {
        if separator == token.token_type {
          expect_separator = false;
          parser.position += 1;
          continue;
        }
        break;
      }
      let t = if let Some(t) = Self::try_get_type(parser)? {
        t
      } else {
        break;
      };
      result.push(t);
      expect_separator = true;
    }
    Ok(result)
  }

  fn try_parse_structured_array<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>
  ) -> Result<bumpalo::collections::Vec<'arena, Node<'arena>>, ParserError> {
    let mut result = bumpalo::vec![in parser.arena];
    let mut expect_separator = false;
    loop {
      let token = parser.get_token(parser.position)?;
      if ![TokenType::DocUnqualifiedName, TokenType::DocComma].contains(&token.token_type) {
        break;
      }
      if expect_separator {
        if TokenType::DocComma == token.token_type {
          expect_separator = false;
          parser.position += 1;
          continue;
        }
        break;
      }
      let label = token.value.to_owned();
      parser.position += 1;
      let is_nullable = if
        parser.get_token(parser.position)?.token_type == TokenType::DocQuestionMark
      {
        parser.position += 1;
        true
      } else {
        false
      };
      if parser.get_token(parser.position)?.token_type == TokenType::DocColon {
        parser.position += 1;
      } else {
        break;
      }
      let t = if let Some(t) = Self::try_get_type(parser)? {
        t
      } else {
        break;
      };
      result.push(
        DocStructuredArrayItemTypeNode::loc(is_nullable, label, t.into_boxed(&parser.arena), None)
      );
      expect_separator = true;
    }
    Ok(result)
  }

  fn try_get_type<'arena, 'a>(
    parser: &mut Parser<'arena, 'a>
  ) -> Result<Option<Node<'arena>>, ParserError> {
    let token = parser.get_token(parser.position)?;
    if
      [
        TokenType::DocRelativeName,
        TokenType::DocQualifiedName,
        TokenType::DocUnqualifiedName,
        TokenType::DocFullyQualifiedName,
      ].contains(&token.token_type)
    {
      let loc = token.get_range_location();
      let value = token.value.to_owned();
      parser.position += 1;
      return Ok(Some(DocTypeNode::loc(false, value, loc)));
    } else if token.token_type == TokenType::DocType {
      if token.value == "array" {
        let loc = token.get_range_location();
        let value = token.value.to_owned();
        parser.position += 1;
        let token = parser.get_token(parser.position)?;
        if token.token_type == TokenType::DocLeftAngleBracket {
          parser.position += 1;
          let key = if let Some(i) = Self::try_get_type(parser)? {
            i
          } else {
            return Ok(None);
          };
          if parser.get_token(parser.position)?.token_type == TokenType::DocComma {
            parser.position += 1;
          } else {
            return Ok(None);
          }
          let value = if let Some(i) = Self::try_get_type(parser)? {
            i
          } else {
            return Ok(None);
          };
          let close = parser.get_token(parser.position)?;
          let end_loc = close.get_range_location().unwrap().end;
          if close.token_type == TokenType::DocRightAngleBracket {
            parser.position += 1;
          } else {
            return Ok(None);
          }
          return Ok(
            Some(
              DocArrayTypeNode::loc(
                Some(key.into_boxed(&parser.arena)),
                value.into_boxed(&parser.arena),
                Some(RangeLocation { start: loc.unwrap().start, end: end_loc })
              )
            )
          );
        } else if token.token_type == TokenType::DocLeftCurlyBracket {
          parser.position += 1;
          let types = Self::try_parse_structured_array(parser)?;
          if parser.get_token(parser.position)?.token_type == TokenType::DocRightCurlyBracket {
            parser.position += 1;
          } else {
            return Ok(None);
          }
          let end_loc = parser
            .get_token(parser.position - 1)?
            .get_range_location()
            .unwrap().end;
          return Ok(
            Some(
              DocStructuredArrayTypeNode::loc(
                types,
                Some(RangeLocation { start: loc.unwrap().start, end: end_loc })
              )
            )
          );
        } else {
          return Ok(Some(DocTypeNode::loc(false, value, loc)));
        }
      } else if token.value == "list" {
        let loc = token.get_range_location();
        parser.position += 1;
        let token = parser.get_token(parser.position)?;
        if token.token_type != TokenType::DocLeftAngleBracket {
          return Ok(None);
        }
        parser.position += 1;
        let child = if let Some(i) = Self::try_get_type(parser)? {
          i
        } else {
          return Ok(None);
        };
        let close = parser.get_token(parser.position)?;
        let end_loc = close.get_range_location().unwrap().end;
        if close.token_type == TokenType::DocRightAngleBracket {
          parser.position += 1;
        } else {
          return Ok(None);
        }
        return Ok(
          Some(
            DocListTypeNode::loc(
              Some(child.into_boxed(&parser.arena)),
              Some(RangeLocation { start: loc.unwrap().start, end: end_loc })
            )
          )
        );
      } else if token.value == "callable" {
        let start = token.get_range_location().unwrap().start;
        parser.position += 1;
        if parser.get_token(parser.position)?.token_type == TokenType::DocLeftParenthesis {
          parser.position += 1;
        } else {
          return Ok(None);
        }
        let mut params = bumpalo::vec![in parser.arena];
        loop {
          let token = parser.get_token(parser.position)?;
          if
            [
              TokenType::DocRelativeName,
              TokenType::DocQualifiedName,
              TokenType::DocUnqualifiedName,
              TokenType::DocFullyQualifiedName,
            ].contains(&token.token_type)
          {
            let loc = token.get_range_location();
            let value = token.value.to_owned();
            parser.position += 1;
            params.push(DocTypeNode::loc(false, value, loc));
          } else if token.token_type == TokenType::DocColon {
            parser.position += 1;
            continue;
          }
          break;
        }
        if parser.get_token(parser.position)?.token_type == TokenType::DocRightCurlyBracket {
          parser.position += 1;
        } else {
          return Ok(None);
        }
        let mut return_type = None;
        if parser.get_token(parser.position)?.token_type == TokenType::DocColon {
          parser.position += 1;
          return_type = Self::try_get_type(parser)?.into_boxed(&parser.arena);
        }
        let end = parser
          .get_token(parser.position - 1)?
          .get_range_location()
          .unwrap().end;
        return Ok(
          Some(DocCallableTypeNode::loc(params, return_type, Some(RangeLocation { start, end })))
        );
      } else {
        let loc = token.get_range_location();
        let value = token.value.to_owned();
        parser.position += 1;
        return Ok(Some(DocTypeNode::loc(false, value, loc)));
      }
    } else if token.token_type == TokenType::DocLeftParenthesis {
      parser.position += 1;
      let child = Self::try_get_complex_type(parser)?;
      if parser.get_token(parser.position)?.token_type == TokenType::DocRightParenthesis {
        parser.position += 1;
      } else {
        return Ok(None);
      }
      return Ok(child);
    } else if token.token_type == TokenType::DocQuestionMark {
      let loc = token.get_range_location();
      let value = token.value.to_owned();
      parser.position += 1;
      return Ok(Some(DocTypeNode::loc(true, value, loc)));
      // if let Some(child) = Self::try_get_type(parser)? {
      //   let child_loc = if let Some(child_loc) = child.loc.clone() {
      //     child_loc.start = loc.start;
      //     Some(child_loc)
      //   } else {
      //     Some(RangeLocation { start: loc.start, end: loc.end })
      //   };
      //   return Ok(Some(DocTypeNode::loc(true, child, child_loc)));
      // }
    }
    Ok(None)
  }
}
