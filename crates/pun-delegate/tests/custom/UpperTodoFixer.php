<?php

// A project's own fixer: `// todo` comments become `// TODO`. Not an AbstractFixer, so the Runner checks
// supports() / isCandidate() itself. Test hooks: a file named crash.php kills the worker, `THROW` throws, `BREAK`
// produces invalid code.
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
        if ($file->getFilename() === 'crash.php') {
            exit(3);
        }
        foreach ($tokens as $i => $token) {
            if ($token->isComment() && str_contains($token->getContent(), 'THROW')) {
                throw new RuntimeException('boom');
            }
            if ($token->isComment() && str_contains($token->getContent(), 'BREAK')) {
                $tokens[$i] = new PhpCsFixer\Tokenizer\Token([T_COMMENT, "// broken\n)"]);
            }
            if ($token->isComment() && str_starts_with($token->getContent(), '// todo')) {
                echo 'noise on stdout';
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
        return !str_ends_with($file->getFilename(), '.skip.php');
    }
}
