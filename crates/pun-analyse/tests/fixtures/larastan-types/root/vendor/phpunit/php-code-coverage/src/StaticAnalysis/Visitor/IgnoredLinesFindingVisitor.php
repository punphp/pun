<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\StaticAnalysis;

use function assert;
use function str_contains;
use PhpParser\Node;
use PhpParser\Node\Attribute;
use PhpParser\Node\Stmt\Class_;
use PhpParser\Node\Stmt\ClassMethod;
use PhpParser\Node\Stmt\Enum_;
use PhpParser\Node\Stmt\Function_;
use PhpParser\Node\Stmt\Interface_;
use PhpParser\Node\Stmt\Trait_;
use PhpParser\NodeVisitorAbstract;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class IgnoredLinesFindingVisitor extends NodeVisitorAbstract
{
    /**
     * @var array<int, true>
     */
    private array $ignoredLines = [];
    private readonly bool $useAnnotationsForIgnoringCode;
    private readonly bool $ignoreDeprecated;

    public function __construct(bool $useAnnotationsForIgnoringCode, bool $ignoreDeprecated)
    {}

    public function enterNode(Node $node): null
    {}

    /**
     * @return array<int, true>
     */
    public function ignoredLines(): array
    {}

    private function processDocComment(Node $node): void
    {}
}
