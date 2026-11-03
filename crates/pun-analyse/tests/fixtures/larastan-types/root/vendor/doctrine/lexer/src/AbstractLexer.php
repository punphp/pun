<?php

declare(strict_types=1);

namespace Doctrine\Common\Lexer;

use ReflectionClass;
use UnitEnum;

use function count;
use function implode;
use function preg_split;
use function sprintf;
use function substr;

use const PREG_SPLIT_DELIM_CAPTURE;
use const PREG_SPLIT_NO_EMPTY;
use const PREG_SPLIT_OFFSET_CAPTURE;

/**
 * Base class for writing simple lexers, i.e. for creating small DSLs.
 *
 * @template T of UnitEnum|string|int
 * @template V of string|int|float|bool
 */
abstract class AbstractLexer
{
    /**
     * Lexer original input string.
     */
    private string $input;

    /**
     * Array of scanned tokens.
     *
     * @var list<Token<T, V>>
     */
    private array $tokens = [];

    /**
     * Current lexer position in input string.
     */
    private int $position = 0;

    /**
     * Current peek of current lexer position.
     */
    private int $peek = 0;

    /**
     * The next token in the input.
     *
     * @var Token<T, V>|null
     */
    public Token|null $lookahead;

    /**
     * The last matched/seen token.
     *
     * @var Token<T, V>|null
     */
    public Token|null $token;

    /**
     * Composed regex for input parsing.
     *
     * @var non-empty-string|null
     */
    private string|null $regex = null;

    /**
     * Sets the input data to be tokenized.
     *
     * The Lexer is immediately reset and the new input tokenized.
     * Any unprocessed tokens from any previous input are lost.
     *
     * @param string $input The input to be tokenized.
     *
     * @return void
     */
    public function setInput(string $input)
    {}

    /**
     * Resets the lexer.
     *
     * @return void
     */
    public function reset()
    {}

    /**
     * Resets the peek pointer to 0.
     *
     * @return void
     */
    public function resetPeek()
    {}

    /**
     * Resets the lexer position on the input to the given position.
     *
     * @param int $position Character offset into the input, as found in
     *                      Token::$position.
     *
     * @return void
     */
    public function resetPosition(int $position = 0)
    {}

    /**
     * Finds the index of the first token at or after the given offset.
     *
     * Token offsets are strictly increasing, so a linear scan suffices.
     * Returns one past the end when there is no such token.
     */
    private function findTokenIndexAtOrAfterOffset(int $offset): int
    {}

    /**
     * Retrieve the original lexer's input until a given position.
     *
     * @return string
     */
    public function getInputUntilPosition(int $position)
    {}

    /**
     * Checks whether a given token matches the current lookahead.
     *
     * @param T $type
     *
     * @return bool
     *
     * @phpstan-assert-if-true !=null $this->lookahead
     */
    public function isNextToken(int|string|UnitEnum $type)
    {}

    /**
     * Checks whether any of the given tokens matches the current lookahead.
     *
     * @param list<T> $types
     *
     * @return bool
     *
     * @phpstan-assert-if-true !=null $this->lookahead
     */
    public function isNextTokenAny(array $types)
    {}

    /**
     * Moves to the next token in the input string.
     *
     * @return bool
     *
     * @phpstan-impure
     * @phpstan-assert-if-true !null $this->lookahead
     */
    public function moveNext()
    {}

    /**
     * Tells the lexer to skip input tokens until it sees a token with the given value.
     *
     * @param T $type The token type to skip until.
     *
     * @return void
     */
    public function skipUntil(int|string|UnitEnum $type)
    {}

    /**
     * Checks if given value is identical to the given token.
     *
     * @return bool
     */
    public function isA(string $value, int|string|UnitEnum $token)
    {}

    /**
     * Moves the lookahead token forward.
     *
     * @return Token<T, V>|null The next token or NULL if there are no more tokens ahead.
     */
    public function peek()
    {}

    /**
     * Peeks at the next token, returns it and immediately resets the peek.
     *
     * @return Token<T, V>|null The next token or NULL if there are no more tokens ahead.
     *
     * @phpstan-impure
     */
    public function glimpse()
    {}

    /**
     * Scans the input string for tokens.
     *
     * @param string $input A query string.
     *
     * @return void
     */
    protected function scan(string $input)
    {}

    /**
     * Gets the literal for a given token.
     *
     * @param T $token
     *
     * @return int|string
     */
    public function getLiteral(int|string|UnitEnum $token)
    {}

    /**
     * Regex modifiers
     *
     * @return string
     */
    protected function getModifiers()
    {}

    /**
     * Lexical catchable patterns.
     *
     * @return string[]
     */
    abstract protected function getCatchablePatterns();

    /**
     * Lexical non-catchable patterns.
     *
     * @return string[]
     */
    abstract protected function getNonCatchablePatterns();

    /**
     * Retrieve token type. Also processes the token value if necessary.
     *
     * @return T|null
     *
     * @param-out V $value
     */
    abstract protected function getType(string &$value);
}
