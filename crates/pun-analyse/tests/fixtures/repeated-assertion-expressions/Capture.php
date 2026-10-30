<?php

namespace RepeatedAssertionExpressions\Oracle;

use PhpParser\Node;
use PhpParser\Node\Stmt\Expression;
use PHPStan\Type\VerbosityLevel;
use Rector\NodeTypeResolver\Node\AttributeKey;
use Rector\Rector\AbstractRector;
use Symplify\RuleDocGenerator\ValueObject\RuleDefinition;

final class Capture extends AbstractRector
{
    public function getRuleDefinition(): RuleDefinition
    {
        return new RuleDefinition('Observe original repeated-expression assertion scopes without changing the AST', []);
    }

    public function getNodeTypes(): array
    {
        return [Expression::class];
    }

    public function refactor(Node $node): ?Node
    {
        $expression = $node->expr;
        $scope = $node->getAttribute(AttributeKey::SCOPE);
        $source = file_get_contents($scope->getFile());
        if (!str_starts_with(substr($source, $node->getEndFilePos() + 1), ' // observe')) {
            return null;
        }
        $environment = getenv('REPEATED_ASSERTION_ENVIRONMENT');
        if ($environment && !is_file($environment)) {
            $testCase = new \ReflectionClass(\PHPUnit\Framework\TestCase::class);
            $assert = new \ReflectionClass(\PHPUnit\Framework\Assert::class);
            $methods = [];
            foreach (['assertTrue', 'assertNotNull', 'assertNull', 'fail'] as $name) {
                $method = $assert->getMethod($name);
                $methods[$name] = [
                    'declaring_class' => $method->getDeclaringClass()->getName(),
                    'return' => (string) $method->getReturnType(),
                    'assertions' => substr_count($method->getDocComment() ?: '', '@phpstan-assert '),
                ];
            }
            file_put_contents($environment, json_encode([
                'php' => PHP_VERSION,
                'test_case_source' => substr($testCase->getFileName(), strpos($testCase->getFileName(), '/crates/') + 1),
                'test_case_sha256' => hash_file('sha256', $testCase->getFileName()),
                'parent' => $testCase->getParentClass()->getName(),
                'assert_source' => substr($assert->getFileName(), strpos($assert->getFileName(), '/crates/') + 1),
                'assert_sha256' => hash_file('sha256', $assert->getFileName()),
                'methods' => $methods,
            ], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR)."\n");
        }
        file_put_contents(getenv('REPEATED_ASSERTION_CAPTURE'), json_encode([
            'line' => $expression->getStartLine(),
            'start' => $expression->getStartFilePos(),
            'end' => $expression->getEndFilePos() + 1,
            'expression' => substr($source, $expression->getStartFilePos(), $expression->getEndFilePos() - $expression->getStartFilePos() + 1),
            'full' => $scope->getType($expression)->describe(VerbosityLevel::precise()),
            'native' => $scope->getNativeType($expression)->describe(VerbosityLevel::precise()),
        ], JSON_UNESCAPED_SLASHES)."\n", FILE_APPEND);
        return null;
    }
}
