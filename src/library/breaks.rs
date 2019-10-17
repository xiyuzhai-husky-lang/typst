use super::prelude::*;

/// Ends the current line.
#[derive(Debug, PartialEq)]
pub struct LinebreakFunc;

impl Function for LinebreakFunc {
    fn parse(header: &FuncHeader, body: Option<&str>, _: ParseContext) -> ParseResult<Self>
    where Self: Sized {
        if has_arguments(header) {
            return err("linebreak: expected no arguments");
        }

        if body.is_some() {
            return err("linebreak: expected no body");
        }

        Ok(LinebreakFunc)
    }

    fn layout(&self, _: LayoutContext) -> LayoutResult<CommandList> {
        Ok(commands![Command::FinishFlexRun])
    }
}

/// Ends the current page.
#[derive(Debug, PartialEq)]
pub struct PagebreakFunc;

impl Function for PagebreakFunc {
    fn parse(header: &FuncHeader, body: Option<&str>, _: ParseContext) -> ParseResult<Self>
    where Self: Sized {
        if has_arguments(header) {
            return err("pagebreak: expected no arguments");
        }

        if body.is_some() {
            return err("pagebreak: expected no body");
        }

        Ok(PagebreakFunc)
    }

    fn layout(&self, _: LayoutContext) -> LayoutResult<CommandList> {
        Ok(commands![Command::FinishLayout])
    }
}
