<?php

declare(strict_types=1);

use PhpParser\Node;
use PhpParser\Node\Expr;
use PHPStan\Type\VerbosityLevel;
use Rector\NodeTypeResolver\Node\AttributeKey;
use Rector\Rector\AbstractRector;
use Symplify\RuleDocGenerator\ValueObject\RuleDefinition;

final class UnmatchedCatchCaptureRector extends AbstractRector
{
    public function getRuleDefinition(): RuleDefinition
    {
        return new RuleDefinition('Capture real decorated expression scopes without modifying nodes', []);
    }

    public function getNodeTypes(): array
    {
        return [Expr::class];
    }

    public function refactor(Node $node): ?Node
    {
        $source = file_get_contents(getenv('UNMATCHED_CAPTURE_INPUT'));
        $tail = substr($source, $node->getEndFilePos() + 1);
        if (!preg_match('/^; \/\/ observe:([a-z-]+)/', $tail, $match)) {
            return null;
        }
        $scope = $node->getAttribute(AttributeKey::SCOPE);
        $path = getenv('UNMATCHED_CAPTURE_OUTPUT');
        $records = is_file($path) ? json_decode(file_get_contents($path), true, flags: JSON_THROW_ON_ERROR) : [];
        $records[$match[1]] = [
            'start' => $node->getStartFilePos(),
            'end' => $node->getEndFilePos() + 1,
            'expression' => substr($source, $node->getStartFilePos(), $node->getEndFilePos() - $node->getStartFilePos() + 1),
            'full' => $scope?->getType($node)->describe(VerbosityLevel::precise()),
            'native' => $scope?->getNativeType($node)->describe(VerbosityLevel::precise()),
            'rector_native' => $this->nodeTypeResolver->getNativeType($node)->describe(VerbosityLevel::precise()),
        ];
        ksort($records);
        file_put_contents($path, json_encode($records, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR)."\n");
        return null;
    }
}
