<?php

namespace Egulias\EmailValidator;

use Doctrine\Common\Lexer\AbstractLexer;
use Doctrine\Common\Lexer\Token;

/** @extends AbstractLexer<int, string> */
class EmailLexer extends AbstractLexer
{
    //ASCII values
    public const S_EMPTY            = -1;
    public const C_NUL              = 0;
    public const S_HTAB             = 9;
    public const S_LF               = 10;
    public const S_CR               = 13;
    public const S_SP               = 32;
    public const EXCLAMATION        = 33;
    public const S_DQUOTE           = 34;
    public const NUMBER_SIGN        = 35;
    public const DOLLAR             = 36;
    public const PERCENTAGE         = 37;
    public const AMPERSAND          = 38;
    public const S_SQUOTE           = 39;
    public const S_OPENPARENTHESIS  = 40;
    public const S_CLOSEPARENTHESIS = 41;
    public const ASTERISK           = 42;
    public const S_PLUS             = 43;
    public const S_COMMA            = 44;
    public const S_HYPHEN           = 45;
    public const S_DOT              = 46;
    public const S_SLASH            = 47;
    public const S_COLON            = 58;
    public const S_SEMICOLON        = 59;
    public const S_LOWERTHAN        = 60;
    public const S_EQUAL            = 61;
    public const S_GREATERTHAN      = 62;
    public const QUESTIONMARK       = 63;
    public const S_AT               = 64;
    public const S_OPENBRACKET      = 91;
    public const S_BACKSLASH        = 92;
    public const S_CLOSEBRACKET     = 93;
    public const CARET              = 94;
    public const S_UNDERSCORE       = 95;
    public const S_BACKTICK         = 96;
    public const S_OPENCURLYBRACES  = 123;
    public const S_PIPE             = 124;
    public const S_CLOSECURLYBRACES = 125;
    public const S_TILDE            = 126;
    public const C_DEL              = 127;
    public const INVERT_QUESTIONMARK = 168;
    public const INVERT_EXCLAMATION = 173;
    public const GENERIC            = 300;
    public const S_IPV6TAG          = 301;
    public const INVALID            = 302;
    public const CRLF               = 1310;
    public const S_DOUBLECOLON      = 5858;
    public const ASCII_INVALID_FROM = 127;
    public const ASCII_INVALID_TO   = 199;

    /**
     * US-ASCII visible characters not valid for atext (@link http://tools.ietf.org/html/rfc5322#section-3.2.3)
     *
     * @var array<string, int>
     */
    protected $charValue = [
        '{'    => self::S_OPENCURLYBRACES,
        '}'    => self::S_CLOSECURLYBRACES,
        '('    => self::S_OPENPARENTHESIS,
        ')'    => self::S_CLOSEPARENTHESIS,
        '<'    => self::S_LOWERTHAN,
        '>'    => self::S_GREATERTHAN,
        '['    => self::S_OPENBRACKET,
        ']'    => self::S_CLOSEBRACKET,
        ':'    => self::S_COLON,
        ';'    => self::S_SEMICOLON,
        '@'    => self::S_AT,
        '\\'   => self::S_BACKSLASH,
        '/'    => self::S_SLASH,
        ','    => self::S_COMMA,
        '.'    => self::S_DOT,
        "'"    => self::S_SQUOTE,
        "`"    => self::S_BACKTICK,
        '"'    => self::S_DQUOTE,
        '-'    => self::S_HYPHEN,
        '::'   => self::S_DOUBLECOLON,
        ' '    => self::S_SP,
        "\t"   => self::S_HTAB,
        "\r"   => self::S_CR,
        "\n"   => self::S_LF,
        "\r\n" => self::CRLF,
        'IPv6' => self::S_IPV6TAG,
        ''     => self::S_EMPTY,
        '\0'   => self::C_NUL,
        '*'    => self::ASTERISK,
        '!'    => self::EXCLAMATION,
        '&'    => self::AMPERSAND,
        '^'    => self::CARET,
        '$'    => self::DOLLAR,
        '%'    => self::PERCENTAGE,
        '~'    => self::S_TILDE,
        '|'    => self::S_PIPE,
        '_'    => self::S_UNDERSCORE,
        '='    => self::S_EQUAL,
        '+'    => self::S_PLUS,
        '¿'    => self::INVERT_QUESTIONMARK,
        '?'    => self::QUESTIONMARK,
        '#'    => self::NUMBER_SIGN,
        '¡'    => self::INVERT_EXCLAMATION,
    ];

    public const INVALID_CHARS_REGEX = "/[^\p{S}\p{C}\p{Cc}]+/iu";

    public const VALID_UTF8_REGEX = '/\p{Cc}+/u';

    public const CATCHABLE_PATTERNS = [
        '[a-zA-Z]+[46]?', //ASCII and domain literal
        '[^\x00-\x7F]',  //UTF-8
        '[0-9]+',
        '\r\n',
        '::',
        '\s+?',
        '.',
    ];

    public const NON_CATCHABLE_PATTERNS = [
        '[\xA0-\xff]+',
    ];

    public const MODIFIERS = 'iu';

    /** @var bool */
    protected $hasInvalidTokens = false;

    /**
     * @var Token<int, string>
     */
    protected Token $previous;

    /**
     * The last matched/seen token.
     *
     * @var Token<int, string>
     */
    public Token $current;

    /**
     * @var Token<int, string>
     */
    private Token $nullToken;

    /** @var string */
    private $accumulator = '';

    /** @var bool */
    private $hasToRecord = false;

    public function __construct()
    {}

    public function reset(): void
    {}

    /**
     * @param int $type
     * @throws \UnexpectedValueException
     * @return boolean
     *
     */
    public function find($type): bool
    {}

    /**
     * moveNext
     *
     * @return boolean
     */
    public function moveNext(): bool
    {}

    /**
     * Retrieve token type. Also processes the token value if necessary.
     *
     * @param string $value
     * @throws \InvalidArgumentException
     * @return integer
     */
    protected function getType(&$value): int
    {}

    protected function isValid(string $value): bool
    {}

    protected function isNullType(string $value): bool
    {}

    protected function isInvalidChar(string $value): bool
    {}

    protected function isUTF8Invalid(string $value): bool
    {}

    public function hasInvalidTokens(): bool
    {}

    /**
     * getPrevious
     *
     * @return Token<int, string>
     */
    public function getPrevious(): Token
    {}

    /**
     * Lexical catchable patterns.
     *
     * @return string[]
     */
    protected function getCatchablePatterns(): array
    {}

    /**
     * Lexical non-catchable patterns.
     *
     * @return string[]
     */
    protected function getNonCatchablePatterns(): array
    {}

    protected function getModifiers(): string
    {}

    public function getAccumulatedValues(): string
    {}

    public function startRecording(): void
    {}

    public function stopRecording(): void
    {}

    public function clearRecorded(): void
    {}
}
