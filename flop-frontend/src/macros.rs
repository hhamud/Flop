#[macro_export]
macro_rules! push_token {
    (keyword; $stack:expr, $keyword:expr, $kind:expr, $row:expr, $col:expr, $namespace:expr) => {
        $stack.push(Token::new(
            $keyword,
            $kind,
            $row,
            $col,
            $keyword.len(),
            $namespace,
        ))
    };

    ($chars:expr, $stack:expr, $value:expr, $kind:expr, $row:expr, $col:expr, $namespace:expr) => {
        $stack.push(Token::new(
            &$value.to_string(),
            $kind,
            $row,
            $col,
            1,
            $namespace,
        ));

        $chars.next();
    };
}
