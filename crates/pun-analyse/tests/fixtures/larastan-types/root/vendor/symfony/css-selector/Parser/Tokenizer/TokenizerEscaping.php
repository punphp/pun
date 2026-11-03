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
 * CSS selector tokenizer escaping applier.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class TokenizerEscaping
{
    public function __construct(
        private TokenizerPatterns $patterns,
    ) {}

    public function escapeUnicode(string $value): string
    {}

    public function escapeUnicodeAndNewLine(string $value): string
    {}

    private function replaceUnicodeSequences(string $value): string
    {}
}
