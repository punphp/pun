<?php

// A project's own fixer, which `pun migrate` names as unsupported: `// todo` comments become `// TODO`.
final class UpperTodoFixer implements PhpCsFixer\Fixer\FixerInterface
{
    public function isCandidate(PhpCsFixer\Tokenizer\Tokens $tokens): bool
    {
        return $tokens->isTokenKindFound(T_COMMENT);
    }

    public function isRisky(): bool
    {
        return false;
    }

    public function fix(SplFileInfo $file, PhpCsFixer\Tokenizer\Tokens $tokens): void
    {
        foreach ($tokens as $i => $token) {
            if ($token->isComment() && str_starts_with($token->getContent(), '// todo')) {
                $tokens[$i] = new PhpCsFixer\Tokenizer\Token([T_COMMENT, '// TODO'.substr($token->getContent(), 7)]);
            }
        }
    }

    public function getDefinition(): PhpCsFixer\FixerDefinition\FixerDefinitionInterface
    {
        return new PhpCsFixer\FixerDefinition\FixerDefinition('Uppercase todo comments.', []);
    }

    public function getName(): string
    {
        return 'Acme/upper_todo';
    }

    public function getPriority(): int
    {
        return 5;
    }

    public function supports(SplFileInfo $file): bool
    {
        return true;
    }
}
