<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\CssSelector\Exception;

use Symfony\Component\CssSelector\Parser\Token;

/**
 * ParseException is thrown when a CSS selector syntax is not valid.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/scrapy/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 */
class SyntaxErrorException extends ParseException
{
    public static function unexpectedToken(string $expectedValue, Token $foundToken): self
    {}

    public static function pseudoElementFound(string $pseudoElement, string $unexpectedLocation): self
    {}

    public static function unclosedString(int $position): self
    {}

    public static function nestedNot(): self
    {}

    public static function nestedHas(): self
    {}

    public static function notAtTheStartOfASelector(string $pseudoElement): self
    {}

    public static function stringAsFunctionArgument(): self
    {}
}
