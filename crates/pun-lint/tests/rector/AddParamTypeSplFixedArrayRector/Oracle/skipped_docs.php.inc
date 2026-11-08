<?php

namespace Pun\Oracle\SplFixedArray;

use PhpCsFixer\Tokenizer\Tokens;

final class Fixer
{
    /**
     * @param Tokens $tokens
     */
    public function mixed(Tokens $tokens, Tokens $others)
    {
    }

    /**
     * @param Tokens<
     */
    public function invalid(Tokens $tokens)
    {
    }
}
-----
<?php

namespace Pun\Oracle\SplFixedArray;

use PhpCsFixer\Tokenizer\Tokens;

final class Fixer
{
    /**
     * @param \PhpCsFixer\Tokenizer\Tokens<\PhpCsFixer\Tokenizer\Token> $tokens
     * @param \PhpCsFixer\Tokenizer\Tokens<\PhpCsFixer\Tokenizer\Token> $others
     */
    public function mixed(Tokens $tokens, Tokens $others)
    {
    }

    /**
     * @param Tokens<
     */
    public function invalid(Tokens $tokens)
    {
    }
}
