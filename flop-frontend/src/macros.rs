#[macro_export]
macro_rules! push_keyword_token {
    ($stack:expr, $keyword:expr, $kind:expr, $row:expr, $col:expr, $namespace:expr) => {
        $stack.push(Token::new(
            $keyword,
            $kind,
            $row,
            $col + 1,
            $keyword.len(),
            $namespace,
        ))
    };
}

#[macro_export]
macro_rules! push_token {
    ($stack:expr, $value:expr, $kind:expr, $row:expr, $col:expr, $namespace:expr) => {
        $stack.push(Token::new(
            &$value.to_string(),
            $kind,
            $row,
            $col,
            1,
            $namespace,
        ));
    };
}
