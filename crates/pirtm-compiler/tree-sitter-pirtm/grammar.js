module.exports = grammar({
  name: 'pirtm',

  rules: {
    source_file: $ => repeat($.prime_indexed_op),

    prime_indexed_op: $ => seq(
      'Ap',
      '(',
      field('prime_index', $.prime_index),
      ',',
      field('expression', $.expression),
      ')'
    ),

    prime_index: $ => /\d+/,

    expression: $ => choice(
      $.atom,
      $.binary_expression,
      $.prime_indexed_op
    ),

    binary_expression: $ => prec.left(1, seq($.expression, '⊗', $.expression)),

    atom: $ => /[a-zA-Z_][a-zA-Z0-9_]*/
  }
});
