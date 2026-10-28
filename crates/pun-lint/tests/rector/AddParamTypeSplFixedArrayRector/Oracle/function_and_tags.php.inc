<?php

namespace Pun\Oracle\SplFixedArray;

use PhpCsFixer\Tokenizer\Tokens;
use PhpCsFixer\Doctrine\Annotation\Tokens as AnnotationTokens;

function fix(Tokens $tokens, int $index)
{
}

final class Fixer
{
    /**
     * Fixes things.
     *
     * @param Tokens $tokens the tokens
     * @param int $index
     */
    public function withTag(Tokens $tokens, int $index)
    {
    }

    /**
     * @return void
     */
    public function withOtherDoc(AnnotationTokens $tokens)
    {
    }

    /**
     * @param Tokens<\PhpCsFixer\Tokenizer\Token> $tokens
     */
    public function alreadyGeneric(Tokens $tokens)
    {
    }

    public function nullable(?Tokens $tokens)
    {
    }

    public function other(\SplFixedArray $tokens)
    {
    }
}
-----
<?php

namespace Pun\Oracle\SplFixedArray;

use PhpCsFixer\Tokenizer\Tokens;
use PhpCsFixer\Doctrine\Annotation\Tokens as AnnotationTokens;

/**
 * @param \PhpCsFixer\Tokenizer\Tokens<\PhpCsFixer\Tokenizer\Token> $tokens
 */
function fix(Tokens $tokens, int $index)
{
}

final class Fixer
{
    /**
     * Fixes things.
     *
     * @param \PhpCsFixer\Tokenizer\Tokens<\PhpCsFixer\Tokenizer\Token> $tokens the tokens
     * @param int $index
     */
    public function withTag(Tokens $tokens, int $index)
    {
    }

    /**
     * @return void
     * @param \PhpCsFixer\Doctrine\Annotation\Tokens<\PhpCsFixer\Doctrine\Annotation\Token> $tokens
     */
    public function withOtherDoc(AnnotationTokens $tokens)
    {
    }

    /**
     * @param Tokens<\PhpCsFixer\Tokenizer\Token> $tokens
     */
    public function alreadyGeneric(Tokens $tokens)
    {
    }

    public function nullable(?Tokens $tokens)
    {
    }

    public function other(\SplFixedArray $tokens)
    {
    }
}
