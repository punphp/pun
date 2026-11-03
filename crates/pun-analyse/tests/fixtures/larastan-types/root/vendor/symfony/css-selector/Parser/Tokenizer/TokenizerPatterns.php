<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\CssSelector\Parser\Tokenizer;

/**
 * CSS selector tokenizer patterns builder.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class TokenizerPatterns
{
    private string $unicodeEscapePattern;
    private string $simpleEscapePattern;
    private string $newLineEscapePattern;
    private string $escapePattern;
    private string $stringEscapePattern;
    private string $nonAsciiPattern;
    private string $nmCharPattern;
    private string $nmStartPattern;
    private string $identifierPattern;
    private string $hashPattern;
    private string $numberPattern;
    private string $quotedStringPattern;

    public function __construct()
    {}

    public function getNewLineEscapePattern(): string
    {}

    public function getSimpleEscapePattern(): string
    {}

    public function getUnicodeEscapePattern(): string
    {}

    public function getIdentifierPattern(): string
    {}

    public function getHashPattern(): string
    {}

    public function getNumberPattern(): string
    {}

    public function getQuotedStringPattern(string $quote): string
    {}
}
