<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\CssSelector\Parser;

/**
 * CSS selector token.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class Token
{
    public const TYPE_FILE_END = 'eof';
    public const TYPE_DELIMITER = 'delimiter';
    public const TYPE_WHITESPACE = 'whitespace';
    public const TYPE_IDENTIFIER = 'identifier';
    public const TYPE_HASH = 'hash';
    public const TYPE_NUMBER = 'number';
    public const TYPE_STRING = 'string';

    /**
     * @param self::TYPE_*|null $type
     */
    public function __construct(
        private ?string $type,
        private ?string $value,
        private ?int $position,
    ) {}

    /**
     * @return self::TYPE_*|null
     */
    public function getType(): ?string
    {}

    public function getValue(): ?string
    {}

    public function getPosition(): ?int
    {}

    public function isFileEnd(): bool
    {}

    public function isDelimiter(array $values = []): bool
    {}

    public function isWhitespace(): bool
    {}

    public function isIdentifier(): bool
    {}

    public function isHash(): bool
    {}

    public function isNumber(): bool
    {}

    public function isString(): bool
    {}

    public function __toString(): string
    {}
}
